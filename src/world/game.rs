use crate::world::{ConfigLevel, Input, Level};

use quicksilver::{
    geom::Line,
    graphics::{Background::Col, Color},
    input::{ButtonState, Key},
    lifecycle::{State, Window},
    Result,
};
use std::fs;

pub struct Game {
    pub levels: Vec<Vec<Level>>,
    pub cur_level: usize,
    pub sub_level: usize,
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

            let config_level: ConfigLevel = toml::from_str(&level_string).unwrap();
            levels.push(vec![Level::from_config_level(&config_level)]);
            // uncomment to dump level data as it's loaded
            //println!("Level {} \n{:#?}\nLevel {}\n", lvl, levels.last(), lvl);
        }
        let game = Game {
            levels: levels.clone(),
            cur_level: 0,
            sub_level: 0,
            draw_debug: true,
        };
        Ok(game)
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        // only one type of input per frame, sorry
        let mut input = None;
        if _window.keyboard()[Key::Right] == ButtonState::Pressed {
            input = Some(Input::Move((1, 0)));
        } else if _window.keyboard()[Key::Left] == ButtonState::Pressed {
            input = Some(Input::Move((-1, 0)));
        } else if _window.keyboard()[Key::Up] == ButtonState::Pressed {
            input = Some(Input::Move((0, -1)));
        } else if _window.keyboard()[Key::Down] == ButtonState::Pressed {
            input = Some(Input::Move((0, 1)));
        } else if _window.keyboard()[Key::Space] == ButtonState::Pressed {
            input = Some(Input::Move((0, 0)));
        } else if _window.keyboard()[Key::R] == ButtonState::Pressed {
            input = Some(Input::Restart);
        } else if _window.keyboard()[Key::Back] == ButtonState::Pressed {
            input = Some(Input::Undo);
        } else if _window.keyboard()[Key::Space] == ButtonState::Pressed {
            input = Some(Input::Move((0, 0)));
        } else if _window.keyboard()[Key::Escape] == ButtonState::Pressed {
            input = Some(Input::Exit);
        } else if _window.keyboard()[Key::LBracket] == ButtonState::Pressed {
            input = Some(Input::PrevLevel);
        } else if _window.keyboard()[Key::RBracket] == ButtonState::Pressed {
            input = Some(Input::NextLevel);
        }

        match input {
            Some(Input::NextLevel) => {
                self.cur_level += 1;
                if self.cur_level >= self.levels.len() {
                    self.cur_level = 0;
                }
                // self.levels[self.cur_level].resize(1, Level::new());
                return Ok(());
            }
            Some(Input::PrevLevel) => {
                if self.cur_level == 0 {
                    self.cur_level = self.levels.len();
                }
                self.cur_level -= 1;
                // self.levels[self.cur_level].resize(1, Level::new());
                return Ok(());
            }
            Some(Input::Restart) => {
                self.levels[self.cur_level].resize(1, Level::new());
                return Ok(());
            }
            Some(Input::Undo) => {
                let c = self.levels[self.cur_level].len();
                if c > 1 {
                    self.levels[self.cur_level].resize(c - 1, Level::new());
                }
                return Ok(());
            }
            Some(Input::Exit) => std::process::exit(0),
            _ => (),
        }

        if !input.is_none() {
            if let Some(l) = Level::process(
                &self.levels[self.cur_level].last().unwrap(),
                &input.unwrap(),
            ) {
                self.levels[self.cur_level].push(l);
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

        let mut level = self.levels[self.cur_level].last().unwrap().clone();

        if self.draw_debug {
            for wall in &mut level.walls {
                wall.draw_debug(window);
            }
            for door in &mut level.doors {
                door.draw_debug(window);
            }
            let sws = level.switches.clone();
            for mut switch in sws {
                let (sx, sy) = (switch.x, switch.y);
                let mut ox = 0;
                let mut oy = 0;
                let mut found = false;
                for toggle in &switch.toggles {
                    if let Some(other_pos) = level.get_entity_pos_by_id(*toggle) {
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
                switch.draw_debug(window);
            }
            for spikeball in &mut level.spikeballs {
                spikeball.draw_debug(window);
            }
            for teleport in &mut level.teleports {
                teleport.draw_debug(window);
            }

            level.start.draw_debug(window);
            level.finish.draw_debug(window);

            level.player.draw_debug(window);
            level.shadow.draw_debug(window);
        }

        Ok(())
    }
}
