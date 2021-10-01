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
    let cords_asciivector = api::api(&host);
    let cords_s = String::from_utf8_lossy(&cords_asciivector);
    let cords: Vec<&str> = cords_s.split("\n").collect();

    println!("lon: {}", cords[0]);
    println!("lat: {}", cords[1]);

    println!("Vec: {:?}", cords);

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let canv = Canvas::default()
            .block(Block::default().title("Canvas").borders(Borders::ALL))
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .paint(|ctx| {
                ctx.draw(&Map {
                    resolution: MapResolution::High,
                    color: Color::White
                });
                ctx.layer();
                ctx.draw(&Line {
                    x1: 0.0,
                    y1: 10.0,
                    x2: 10.0,
                    y2: 10.0,
                    color: Color::White,
                });
                ctx.draw(&Rectangle {
                    x: 10.0,
                    y: 20.0,
                    width: 10.0,
                    height: 10.0,
                    color: Color::Red
                });
            });

        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);

        // f.render_widget(canv, size); TUI_G
    })
}