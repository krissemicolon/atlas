use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::widgets::canvas::*;
use tui::layout::{Layout, Constraint, Direction};
use tui::style::Color;

mod api;

fn main() -> Result<(), io::Error> {
    // setting to test ip
    let host = String::from("212.111.40.13");
    let coords_asciivector = api::api(&host);
    let coords_s = String::from_utf8_lossy(&coords_asciivector);
    let coords_vec: Vec<&str> = coords_s.split("\n").collect();
    let lat = &coords_vec[0].parse::<f64>().unwrap();
    let lon = &coords_vec[1].parse::<f64>().unwrap();

    println!("lat: {}", lat);
    println!("lon: {}", lon);

    println!("Vec: {:?}", coords_vec);

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

        f.render_widget(canv, size);
    })?;
    Ok(())
}