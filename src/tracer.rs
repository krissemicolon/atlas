extern crate libc;
extern crate rand;
extern crate socket;
extern crate time;

use std::iter::Iterator;
use std::io::{self, Error, ErrorKind};
use std::net::{SocketAddr, ToSocketAddrs};

use libc::{SO_RCVTIMEO, timeval};
use socket::{AF_INET, IP_TTL, IPPROTO_IP, SOCK_RAW, SOL_SOCKET, Socket};
use time::{Duration, SteadyTime};

const IPPROTO_ICMP: i32 = 1;

pub struct TraceResult {
    addr: SocketAddr,
    ttl: u8,
    ident: u16,
    seq_num: u16,
    done: bool,
    timeout: Duration,
}

#[derive(Debug)]
pub struct TraceHop {
    /// The Time-To-Live value used to find this hop
    pub ttl: u8,
    /// The address of the node in the hop
    pub host: SocketAddr,
    /// The round trip time to the hop
    pub rtt: Duration,
}

/// Performs a traceroute, waiting at each request for around one second before failing
pub fn execute<T: ToSocketAddrs>(address: T) -> io::Result<TraceResult> {
    execute_with_timeout(address, Duration::seconds(1))
}

/// Performs a traceroute, waiting at each request for around until timeout elapses before failing
pub fn execute_with_timeout<T: ToSocketAddrs>(address: T, timeout: Duration) -> io::Result<TraceResult> {
    match timeout.num_microseconds() {
        None => return Err(Error::new(ErrorKind::InvalidInput, "Timeout too large")),
        Some(0) => return Err(Error::new(ErrorKind::InvalidInput, "Timeout too small")),
        _ => (),
    };

    let mut addr_iter = address.to_socket_addrs()?;
    match addr_iter.next() {
        None => Err(Error::new(ErrorKind::InvalidInput, "Could not interpret address")),
        Some(addr) => Ok(TraceResult {
            addr: addr,
            ttl: 0,
            ident: rand::random(),
            seq_num: 0,
            done: false,
            timeout: timeout,
        })
    }
}

impl Iterator for TraceResult {
    type Item = io::Result<TraceHop>;

    fn next(&mut self) -> Option<io::Result<TraceHop>> {
        if self.done {
            return None;
        }

        let res = self.find_next_hop();
        if res.is_err() {
            self.done = true;
        }
        Some(res)
    }
}

impl TraceResult {
    fn find_next_hop(&mut self) -> io::Result<TraceHop> {
        let socket = Socket::new(AF_INET, SOCK_RAW, IPPROTO_ICMP)?;
        loop {
            let ping = construct_ping(self.ident, self.seq_num);
            self.seq_num += 1;

            self.ttl += 1;
            socket.setsockopt(IPPROTO_IP, IP_TTL, self.ttl)?;
            socket.setsockopt(SOL_SOCKET, SO_RCVTIMEO, compute_timeout(self.timeout))?;

            let wrote = socket.sendto(&ping, 0, &self.addr)?;
            assert_eq!(wrote, ping.len());
            let start_time = SteadyTime::now();

            // After deadline passes, restart the loop to advance the TTL and resend.
            while SteadyTime::now() < start_time + self.timeout {
                let (sender, data);
                match socket.recvfrom(4096, 0) {
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => continue,
                    Err(e) => return Err(e),
                    Ok((s, d)) => {
                        sender = s;
                        data = d;
                    },
                }

                let data = ip_payload(&data)?;
                match IcmpMessage::from_buf(&data) {
                    Some(IcmpMessage::EchoReply(header, _)) => {
                        if header.data == ping[4..8] {
                            let hop = TraceHop {
                                ttl: self.ttl,
                                host: sender,
                                rtt: SteadyTime::now() - start_time,
                            };
                            self.done = true;
                            return Ok(hop);
                        }
                    }
                    Some(IcmpMessage::TimeExceeded(_, ip_payload)) => {
                        if self.ttl == 255 {
                            self.done = true;
                            return Err(Error::new(ErrorKind::TimedOut, "too many hops"));
                        }
                        if ip_payload[4..8] == ping[4..8] {
                            let hop = TraceHop {
                                ttl: self.ttl,
                                host: sender,
                                rtt: SteadyTime::now() - start_time,
                            };
                            return Ok(hop);
                        }
                    },
                    _ => (),
                }
            }
        }
    }
}

/// Computes and populates the checksum of an ICMP message
fn fill_checksum(ip_payload: &mut [u8]) {
    ip_payload[2] = 0;
    ip_payload[3] = 0;

    let mut sum = 0u16;
    for word in ip_payload.chunks(2) {
        let mut part = (word[0] as u16) << 8;
        if word.len() > 1 {
            part += word[1] as u16;
        }
        sum = sum.wrapping_add(part);
    }

    sum = !sum;
    ip_payload[2] = (sum >> 8) as u8;
    ip_payload[3] = (sum & 0xff) as u8;
}

/// Constructs an ICMP ping IP payload with the given identifier and sequence number
fn construct_ping(ident: u16, seq_num: u16) -> Vec<u8> {
    let mut ping: Vec<u8> = vec![
        8u8, 0u8,
        0u8, 0u8,
        (ident >> 8) as u8, (ident & 0xff) as u8,
        (seq_num >> 8) as u8, (seq_num & 0xff) as u8];
    fill_checksum(&mut ping);
    ping
}

const ICMP_HEADER_LEN: usize = 8;

enum IcmpMessage<'a> {
    EchoReply(IcmpHeader, &'a [u8]),
    TimeExceeded(IcmpHeader, &'a [u8]),
    Unknown(IcmpHeader),
}

impl<'a> IcmpMessage<'a> {
    fn from_buf(buf: &'a [u8]) -> Option<IcmpMessage<'a>> {
        // TODO: Check checksum
        let header = IcmpHeader::from_buf(buf);
        if header.is_none() {
            return None;
        }
        let header = header.unwrap();
        let payload = &buf[ICMP_HEADER_LEN..];

        Some(match header.msg_type {
            0 => IcmpMessage::EchoReply(header, payload),
            11 => {
                match ip_payload(payload) {
                    Ok(body) => IcmpMessage::TimeExceeded(header, body),
                    Err(..) => return None,
                }
            },
            _ => IcmpMessage::Unknown(header),
        })
    }
}

#[derive(Debug)]
struct IcmpHeader {
    pub msg_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub data: [u8; 4],
}

impl IcmpHeader {
    fn from_buf(buf: &[u8]) -> Option<IcmpHeader> {
        if buf.len() < 8 {
            return None;
        }
        let data: [u8; 4] = [buf[4], buf[5], buf[6], buf[7]];

        Some(IcmpHeader {
            msg_type: buf[0],
            code: buf[1],
            checksum: ((buf[2] as u16) << 8) + (buf[3] as u16),
            data: data,
        })
    }
}

/// Takes a buffer containing a prefix of an IP packet (with a full header)
/// and returns the IP payload part of the packet
fn ip_payload(packet: &[u8]) -> io::Result<&[u8]> {
    if packet.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidData, "Packet too short"));
    }
    let len = ((packet[0] & 0x0f) * 4) as usize;
    if len < 20 || packet.len() < len {
        return Err(Error::new(ErrorKind::InvalidData, "Packet too short"));
    }
    Ok(&packet[len..])
}

fn compute_timeout(timeout: Duration) -> timeval {
    let usecs = timeout.num_microseconds().unwrap();
    timeval{
        tv_sec: usecs / 1000000,
        tv_usec: usecs % 1000000
    }
}
