#[allow(unused_imports)]
use crate::world::{misc::log, Input, Level};
use std::f32;
use std::result::Result;

use quicksilver::{
    geom::{Rectangle, Transform},
    graphics::{Background::Col, Color},
    lifecycle::{Asset, State, Window},
    Result as QResult,
};

pub struct Loading {
    level_sources: Vec<(Asset<String>, bool)>,
    loaded: bool,
    loaded_frame: f32,
}

impl Loading {
    pub fn is_done_loading(&self) -> bool {
        return self.loaded && self.loaded_frame as u8 == 1;
    }

    pub fn get_levels(&mut self) -> Result<Vec<Level>, &'static str> {
        if !self.loaded && self.loaded_frame as u8 == 1 {
            return Err("Not finished loading yet");
        }

        let mut levels = Vec::new();
        for ls in &mut self.level_sources {
            ls.0.execute(|lvl| {
                levels.push(Level::from_utf8(&lvl.clone()));
                Ok(())
            })
            .expect("Something happened converting from string to level");
        }
        Ok(levels)
    }
}

impl State for Loading {
    fn new() -> QResult<Loading> {
        let mut level_sources = Vec::new();
        for lvl in 1..10 {
            level_sources.push((Level::load(format!("levels/Level{}.toml", lvl)), false));
        }
        let loading = Loading {
            level_sources: level_sources,
            loaded: false,
            loaded_frame: 0.,
        };
        Ok(loading)
    }

    fn update(&mut self, _window: &mut Window) -> QResult<()> {
        if self.loaded == false {
            let mut num_loaded = 0;
            for (ls, loaded) in &mut self.level_sources {
                if *loaded == false {
                    if ls
                        .execute(|_| {
                            *loaded = true;
                            Ok(())
                        })
                        .is_err()
                    {
                        panic!("something broke");
                    }
                }
                if *loaded == true {
                    num_loaded += 1;
                }
            }
            self.loaded = num_loaded == self.level_sources.len();
        } else {
            self.loaded_frame += 1. / 60.;
            self.loaded_frame = self.loaded_frame.min(1.);
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> QResult<()> {
        let mut num_loaded = 0;
        let to_load = self.level_sources.len();
        let width = 440;
        for (_, loaded) in &self.level_sources {
            if *loaded {
                num_loaded += 1;
            }
        }

        let c = Color {
            r: self.loaded_frame,
            g: self.loaded_frame,
            b: self.loaded_frame,
            a: 1.0,
        };

        window.draw_ex(
            &Rectangle::new((96, 212), (4, 40)),
            Col(c),
            Transform::rotate(0),
            0,
        );
        window.draw_ex(
            &Rectangle::new((540, 212), (4, 40)),
            Col(c),
            Transform::rotate(0),
            0,
        );
        window.draw_ex(
            &Rectangle::new((100, 212), (width, 4)),
            Col(c),
            Transform::rotate(0),
            0,
        );
        window.draw_ex(
            &Rectangle::new((100, 248), (width, 4)),
            Col(c),
            Transform::rotate(0),
            0,
        );

        window.draw_ex(
            &Rectangle::new(
                (100, 216),
                (
                    (width as f32 * (num_loaded as f32 / to_load as f32)) as i32,
                    32,
                ),
            ),
            Col(c),
            Transform::rotate(0),
            0,
        );
        Ok(())
    }
}
