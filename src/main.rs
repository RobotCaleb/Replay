extern crate quicksilver;
extern crate toml;
#[macro_use]
extern crate serde_derive;

mod objects;
mod world;

use crate::world::game::Game;
use objects::*;
use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings, State},
};

fn main() {
    let _d = Door::new();
    let _w = Wall::new();
    let _t = Teleport::new();
    let _s = Switch::new();
    let _b = Spikeball::new();
    // let _0 = Start::new();
    let _1 = Finish::new();
    // let _p = Player::new();
    // let _h = Shadow::new();

    run::<Game>("Game", Vector::new(640, 480), Settings::default());
}
