extern crate quicksilver;
extern crate toml;
#[macro_use]
extern crate serde_derive;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

mod objects;
mod world;

use crate::world::game::Game;
use quicksilver::{
    geom::Vector,
    graphics::ResizeStrategy,
    lifecycle::{run, Settings},
};

fn main() {
    // let _d = Door::new(0, 0, 0, DoorType::EastWest);
    // let _w = Wall::new(0, 0, 0);
    // let _t = Teleport::new(0, 0, 0, 0, 0);
    // let _s = Switch::new(0, 0, 0, vec![0, 0]);
    // let _b = Spikeball::new(0, 0, 0, Direction::East, ClipType::Clip);
    // let _0 = Start::new(0, 0, 0);
    // let _1 = Finish::new(0, 0, 0);
    // let _p = Player::new(0, 0, 0);
    // let _h = Shadow::new(0, 0, 0);
    // println!("Door:\n{}", toml::to_string(&_d).unwrap());
    // println!("Wall:\n{}", toml::to_string(&_w).unwrap());
    // println!("Teleport:\n{}", toml::to_string(&_t).unwrap());
    // println!("Switch:\n{}", toml::to_string(&_s).unwrap());
    // println!("Spikeball:\n{}", toml::to_string(&_b).unwrap());
    // println!("Start:\n{}", toml::to_string(&_0).unwrap());
    // println!("Finish:\n{}", toml::to_string(&_1).unwrap());
    // println!("Player:\n{}", toml::to_string(&_p).unwrap());
    // println!("Shadow:\n{}", toml::to_string(&_h).unwrap());

    #[cfg(target_arch = "wasm32")]
    {
        console!(log, "in main");
    }

    let mut settings = Settings::default();
    settings.resize = ResizeStrategy::Fill;
    run::<Game>("Game", Vector::new(800, 600), settings);
}
