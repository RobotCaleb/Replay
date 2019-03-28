#[allow(unused_imports)]
use crate::world::{misc::log, Input, Level, Loading, Replay};

use quicksilver::{
    graphics::Color,
    lifecycle::{State, Window},
    Result,
};

#[derive(PartialEq)]
enum GameState {
    Loading,
    Replay,
}

pub struct Game {
    loading: Loading,
    replay: Replay,
    state: GameState,
}

impl State for Game {
    fn new() -> Result<Game> {
        let game = Game {
            loading: match Loading::new() {
                Ok(l) => l,
                _ => panic!("Couldn't instantiate Loading"),
            },
            replay: match Replay::new() {
                Ok(r) => r,
                _ => panic!("Couldn't instantiate Loading"),
            },
            state: GameState::Loading,
        };
        Ok(game)
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        if self.state == GameState::Loading && self.loading.is_done_loading() {
            log("transitioning");

            self.replay.init(
                self.loading
                    .get_levels()
                    .expect("Weird, it hasn't finish loading but said it had")
                    .as_ref(),
            );
            self.state = GameState::Replay;
        }

        match self.state {
            GameState::Loading => self.loading.update(window),
            GameState::Replay => self.replay.update(window),
        }
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        match self.state {
            GameState::Loading => self.loading.draw(window),
            GameState::Replay => self.replay.draw(window),
        }
    }
}
