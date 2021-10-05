use std::io;
use std::env;
use std::net::{SocketAddr, ToSocketAddrs};

mod api;
mod tui;
mod tracer;

fn main() {
    // let mut args = env::args();
    // let ip: String = args.nth(1).unwrap() + ":0";

    // geolookup
    let host = String::from("212.111.40.13");
    let coords_asciivector = api::get_geo_from_host(&host);
    let coords_s = String::from_utf8_lossy(&coords_asciivector);
    let coords_vec: Vec<&str> = coords_s.split("\n").collect();
    let lat = &coords_vec[0].parse::<f64>().unwrap();
    let lon = &coords_vec[1].parse::<f64>().unwrap();

    println!("lat: {}", lat);
    println!("lon: {}", lon);

    println!("Vec: {:?}", coords_vec);

    // trace
    // let mut hosts: Vec<SocketAddr> = Vec::new();
    // let mut i: usize = 0;
    // for trace_result in tracer::execute(format!("{}:0", host)).unwrap() {
    //     match trace_result {
    //         Ok(res) => hosts.push(res.host),
    //         Err(e)  => println!("Error Executing Traceroute: {}", e),
    //     }
    //     println!("Host: {}", hosts[i]);

    //     i += 1;
    // }

    // tui
    // TODO: Map Override
    let mut atlas_tui = tui::TUI::new().unwrap();
    // atlas_tui.draw_map();
    atlas_tui.draw_dot(&lat, &lon);
}