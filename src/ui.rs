use std::io;
use std::io::Stdout;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::canvas::*;
use tui::style::Color;

pub struct TUI {
    term: Terminal<TermionBackend<Stdout>>,
}

impl TUI {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Self {
            term: Terminal::new(TermionBackend::new(io::stdout()))?,
        })
    }

    pub fn draw_map(&mut self) -> Result<(), std::io::Error> {
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

    pub fn draw_dot(&mut self, lat: &f64, lon: &f64, color: &Color) -> Result<(), std::io::Error> {
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

                        color: *color,
                    });
                });

            f.render_widget(canv, size);
        })?;
        Ok(())
    }

    pub fn draw_result(&mut self, coords: &Vec<(f64, f64)>) -> Result<(), std::io::Error> {
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

                    let mut prev_x: f64 = 0.0;
                    let mut prev_y: f64 = 0.0;

                    for c in coords {
                        if prev_x != 0.0 && prev_y != 0.0 {
                            ctx.draw(&Line {
                                x1: c.0,
                                y1: c.1,
                                x2: prev_x,
                                y2: prev_y,

                                color: Color::Blue,
                            });
                        }

                        ctx.draw(&Points {
                            coords: &[*c],
                            color: Color::Red,
                        });

                        prev_x = c.0;
                        prev_y = c.1;
                    }
                });

            f.render_widget(canv, size);
        })?;
        Ok(())
    }

}