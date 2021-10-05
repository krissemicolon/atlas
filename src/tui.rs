use std::io;
use std::io::Stdout;
use termion::raw::RawTerminal;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::widgets::canvas::*;
use tui::layout::{Layout, Constraint, Direction};
use tui::style::Color;

// #[derive(Copy)]
pub struct TUI {
    term: Terminal<TermionBackend<Stdout>>,
}

impl TUI {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Self {
            term: Terminal::new(TermionBackend::new(io::stdout()))?,
        })
    }

    pub fn draw_map(&mut self) -> Result<(), std::io::Error>{
        self.term.draw(|f| {
            let size = f.size();
            let canv = Canvas::default()
                .x_bounds([-180.0, 180.0])
                .y_bounds([-90.0, 90.0])
                .paint(|ctx| {
                    ctx.draw(&Map {
                        resolution: MapResolution::High,

                        color: Color::White
                    });
                });

            f.render_widget(canv, size);
        })?;
        Ok(())
    }

    pub fn draw_dot(&mut self, lat: &f64, lon: &f64) -> Result<(), std::io::Error> {
        self.term.draw(|f| {
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
}