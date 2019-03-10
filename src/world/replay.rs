#[allow(unused_imports)]
use crate::world::{misc::log, Input, Level};

use quicksilver::{
    geom::Line,
    graphics::{Background::Col, Color},
    input::{ButtonState, Key},
    lifecycle::{Asset, State, Window},
    Result,
};

pub struct Replay {
    levels: Vec<Vec<Level>>,
    cur_level: usize,
    draw_debug: bool,
    intermediate_level: Level,
}

impl Replay {
    pub fn init(&mut self, levels: &Vec<Level>) {
        for lvl in levels {
            self.levels.push(vec![lvl.clone()]);
        }
    }
}

impl State for Replay {
    fn new() -> Result<Replay> {
        let replay = Replay {
            levels: Vec::new(),
            cur_level: 0,
            draw_debug: true,
            intermediate_level: Level::new(),
        };
        Ok(replay)
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        // only one type of input per frame, sorry
        let mut input = None;
        if window.keyboard()[Key::Right] == ButtonState::Pressed {
            input = Some(Input::Move((1, 0)));
        } else if window.keyboard()[Key::Left] == ButtonState::Pressed {
            input = Some(Input::Move((-1, 0)));
        } else if window.keyboard()[Key::Up] == ButtonState::Pressed {
            input = Some(Input::Move((0, -1)));
        } else if window.keyboard()[Key::Down] == ButtonState::Pressed {
            input = Some(Input::Move((0, 1)));
        } else if window.keyboard()[Key::Space] == ButtonState::Pressed {
            input = Some(Input::Move((0, 0)));
        } else if window.keyboard()[Key::R] == ButtonState::Pressed {
            input = Some(Input::Restart);
        } else if window.keyboard()[Key::Back] == ButtonState::Pressed {
            input = Some(Input::Undo);
        } else if window.keyboard()[Key::Space] == ButtonState::Pressed {
            input = Some(Input::Move((0, 0)));
        } else if window.keyboard()[Key::Escape] == ButtonState::Pressed {
            input = Some(Input::Exit);
        } else if window.keyboard()[Key::LBracket] == ButtonState::Pressed {
            input = Some(Input::PrevLevel);
        } else if window.keyboard()[Key::RBracket] == ButtonState::Pressed {
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

        if self.levels.len() == 0 {
            return Ok(());
        }

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
                            Col(Color::RED),
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
