use crate::world::Level;

use quicksilver::{
    geom::Line,
    graphics::{Background::Col, Color},
    input::{ButtonState, Key},
    lifecycle::{State, Window},
    Result,
};
use std::fs;

pub struct Game {
    pub levels: Vec<Level>,
    pub level: usize,
    pub draw_debug: bool,
}

impl State for Game {
    fn new() -> Result<Game> {
        let mut levels = Vec::new();
        for lvl in 1..10 {
            println!("Reading levels/Level{}.toml", lvl);
            let level_string = fs::read_to_string(format!("levels/Level{}.toml", lvl)).expect(
                &format!("Something went wrong reading levels/Level{}.toml", lvl),
            );

            levels.push(toml::from_str(&level_string).unwrap());
            // uncomment to dump level data as it's loaded
            //println!("Level {} \n{:#?}\nLevel {}\n", lvl, levels.last(), lvl);
        }
        let game = Game {
            levels: levels.clone(),
            level: 0,
            draw_debug: true,
        };
        Ok(game)
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        if _window.keyboard()[Key::LBracket] == ButtonState::Pressed {
            if self.level == 0 {
                self.level = self.levels.len();
            }
            self.level -= 1;
        }
        if _window.keyboard()[Key::RBracket] == ButtonState::Pressed {
            self.level += 1;
            if self.level >= self.levels.len() {
                self.level = 0;
            }
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // this drawing is wrong!
        // I think what needs to happen is assign a z value to everything that
        // can be drawn
        // draw them with draw_ex using z
        // and draw everything one row at a time
        // starting at the top of the screen
        // That seems like it should do the right thing with the updated graphics
        window.clear(Color::WHITE)?;

        let level = &mut self.levels[self.level];

        if self.draw_debug {
            if let Some(walls) = level.walls.as_mut() {
                for wall in walls {
                    wall.draw_debug(window)
                        .expect("Something happened drawing walls");
                }
            }

            if let Some(doors) = level.doors.as_mut() {
                for door in doors {
                    door.draw_debug(window)
                        .expect("Something happened drawing doors");
                }
            }

            if let Some(switches) = level.switches.as_mut() {
                let sws = switches.clone();
                for mut switch in sws {
                    let (sx, sy) = (switch.x, switch.y);
                    let mut ox = 0;
                    let mut oy = 0;
                    let mut found = false;
                    for toggle in &switch.toggles {
                        if let Some(other_pos) = level.get_position(*toggle) {
                            ox = other_pos.0;
                            oy = other_pos.1;
                            found = true;
                        }
                        if found {
                            window.draw(
                                &Line::new(
                                    (sx as u32 * 32 + 16, sy as u32 * 32 + 16),
                                    (ox as u32 * 32 + 16, oy as u32 * 32 + 16),
                                )
                                .with_thickness(1.0),
                                Col(Color::MAGENTA),
                            );
                        }
                    }
                    switch
                        .draw_debug(window)
                        .expect("Something happened drawing switches");
                }
            }

            if let Some(spikeballs) = level.spikeballs.as_mut() {
                for spikeball in spikeballs {
                    spikeball
                        .draw_debug(window)
                        .expect("Something happened drawing spikeballs");
                }
            }

            if let Some(teleports) = level.teleports.as_mut() {
                for teleport in teleports {
                    teleport
                        .draw_debug(window)
                        .expect("Something happened drawing teleports");
                }
            }

            level
                .start
                .draw_debug(window)
                .expect("Something happened drawing start");
            level
                .finish
                .draw_debug(window)
                .expect("Something happened drawing finish");
        }

        Ok(())
    }
}
