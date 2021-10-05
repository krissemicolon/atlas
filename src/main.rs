use std::io;
use std::env;
use std::net::{SocketAddr, ToSocketAddrs};

use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::widgets::canvas::*;
use tui::layout::{Layout, Constraint, Direction};
use tui::style::Color;

mod api;
mod tracer;

// TODO: Modularize
fn main() -> Result<(), io::Error> {
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
    let mut hosts: Vec<SocketAddr> = Vec::new();
    let mut i: usize = 0;
    for trace_result in tracer::execute(format!("{}:0", host)).unwrap() {
        match trace_result {
            Ok(res) => hosts.push(res.host),
            Err(e)  => println!("Error Executing Traceroute: {}", e),
        }
        println!("Host: {}", hosts[i]);

        i += 1;
    }

    // tui
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let canv = Canvas::default()
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .paint(|ctx| {
                ctx.draw(&Map {
                    resolution: MapResolution::High,

                    color: Color::White
                });
                ctx.draw(&Points {
                    coords: &[(*lon, *lat)],

                    color: Color::Red,
                });
            });

        // f.render_widget(canv, size);
    })?;
    Ok(())
}
