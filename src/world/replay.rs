#[allow(unused_imports)]
use crate::world::{misc::log, Input, Level};

use quicksilver::{
    graphics::Color,
    input::{ButtonState, Key},
    lifecycle::{State, Window},
    Result,
};

pub struct Replay {
    levels: Vec<Vec<Level>>,
    cur_level: usize,
    intermediate_levels: Option<Vec<Level>>,
    input_timer: f64,
    input_time: f64,
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
            intermediate_levels: None,
            input_timer: 0.,
            input_time: 1. / 5.,
        };
        Ok(replay)
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        // only one type of input per frame, sorry
        let mut input = None;
        if window.keyboard()[Key::Right].is_down() {
            input = Some(Input::Move((1, 0)));
        } else if window.keyboard()[Key::Left].is_down() {
            input = Some(Input::Move((-1, 0)));
        } else if window.keyboard()[Key::Up].is_down() {
            input = Some(Input::Move((0, -1)));
        } else if window.keyboard()[Key::Down].is_down() {
            input = Some(Input::Move((0, 1)));
        } else if window.keyboard()[Key::Space].is_down() {
            input = Some(Input::Move((0, 0)));
        } else if window.keyboard()[Key::R] == ButtonState::Pressed {
            input = Some(Input::Restart);
        } else if window.keyboard()[Key::Back].is_down() {
            input = Some(Input::Undo);
        } else if window.keyboard()[Key::Escape] == ButtonState::Pressed {
            input = Some(Input::Exit);
        } else if window.keyboard()[Key::LBracket] == ButtonState::Pressed {
            input = Some(Input::PrevLevel);
        } else if window.keyboard()[Key::RBracket] == ButtonState::Pressed {
            input = Some(Input::NextLevel);
        }

        if self.input_timer > 0. {
            self.input_timer = 0f64.max(self.input_timer - 1. / window.current_fps());
        }

        match input {
            Some(Input::NextLevel) => {
                self.cur_level += 1;
                if self.cur_level >= self.levels.len() {
                    self.cur_level = 0;
                }
                // the following line will clear the level being changed to
                // resetting it back to the beginning. probably a good thing.
                // self.levels[self.cur_level].resize(1, Level::new());
                // return Ok(());
            }
            Some(Input::PrevLevel) => {
                if self.cur_level == 0 {
                    self.cur_level = self.levels.len();
                }
                self.cur_level -= 1;
                // the following line will clear the level being changed to
                // resetting it back to the beginning. probably a good thing.
                // self.levels[self.cur_level].resize(1, Level::new());
                // return Ok(());
            }
            Some(Input::Restart) => {
                self.levels[self.cur_level].resize(1, Level::new());
                // return Ok(());
            }
            Some(Input::Undo) => {
                if self.input_timer == 0. {
                    self.input_timer = self.input_time;
                    let c = self.levels[self.cur_level].len();
                    if c > 1 {
                        self.levels[self.cur_level].resize(c - 1, Level::new());
                    }
                }
                // return Ok(());
            }
            Some(Input::Exit) => std::process::exit(0),
            _ => (),
        }

        match input {
            Some(Input::Move(m)) => {
                if self.input_timer == 0. {
                    self.input_timer = self.input_time;
                    if let Some((l, ils)) =
                        Level::process(&self.levels[self.cur_level].last().unwrap(), m)
                    {
                        self.intermediate_levels = ils;
                        self.levels[self.cur_level].push(l);
                    }
                }
            }
            _ => self.intermediate_levels = None,
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        if self.levels.len() == 0 {
            return Ok(());
        }

        let mut im: Option<usize> = None;
        let i1 = self.input_time / 3.;
        let i2 = i1 * 2.;

        if let Some(lvl) = &self.intermediate_levels {
            if self.input_timer >= i2 {
                im = Some(0);
            }
            if self.input_timer < i2 && self.input_timer >= i1 {
                im = Some(1);
            }
            if self.input_timer < i1 && self.input_timer >= 0. {
                im = Some(2);
            }

            lvl[im.unwrap()].draw_debug(window);
        } else {
            self.levels[self.cur_level]
                .last()
                .unwrap()
                .draw_debug(window);
        }

        Ok(())
    }
}
