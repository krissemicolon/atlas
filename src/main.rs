use std::io;
use std::env;
use std::net::{SocketAddr, ToSocketAddrs};
use std::{thread, time};
use tui::style::Color;
use tui::widgets::canvas::*;

mod api;
mod ui;
mod tracer;

fn main() {
    // get host
    // ------------
    let mut args = env::args();
    let host =  match args.nth(1) {
        Some(o)   => o,
        None             => {
            println!("Usage: atlas <ip>");
            std::process::exit(1);
        },
    };

    // init tui
    let mut atlas_tui = ui::TUI::new().unwrap();
    // draw map
    atlas_tui.draw_map();

    // trace
    let mut hosts: Vec<String> = Vec::new();
    let mut coords: Vec<(f64, f64)> = Vec::new();
    let mut i: usize = 0;

    for trace_result in tracer::execute(format!("{}:0", host)).unwrap() {
        match trace_result {
            Ok(res) => hosts.push(res.host.to_string()),
            Err(e)  => panic!("Error Executing Traceroute: {}", e),
        }

        // -":0"
        hosts[i].pop();
        hosts[i].pop();

        // geolookup
        let coords_asciivector = api::get_geo_from_host(&hosts[i]);
        let coords_s = String::from_utf8_lossy(&coords_asciivector);
        let coords_vec: Vec<&str> = coords_s.split("\n").collect();

        let lat: f64 = match &coords_vec[0].parse::<f64>() {
            Ok(o)  => *o,
            Err(_) => {
                i += 1;
                continue;
            },
        };

        let lon: f64 = match &coords_vec[1].parse::<f64>() {
            Ok(o)  => *o,
            Err(_) => {
                i += 1;
                continue;
            },
        };

        coords.push((lon, lat));

        // draw to ui
        atlas_tui.draw_result(&coords).unwrap();

        i += 1;
    }

    thread::sleep(time::Duration::from_secs(100));
}