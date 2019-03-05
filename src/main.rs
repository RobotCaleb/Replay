extern crate toml;
#[macro_use]
extern crate serde_derive;

mod objects;

use std::fs;

use objects::*;

#[derive(Debug, Deserialize)]
struct World {
    level: Option<Level>,
}

#[derive(Debug, Deserialize)]
struct Level {
    doors: Option<Vec<Door>>,
    spikeballs: Option<Vec<Spikeball>>,
    switches: Option<Vec<Switch>>,
    walls: Vec<Wall>,
    teleports: Option<Vec<Teleport>>,
    start: Start,
    finish: Finish,
}

fn main() {
    let _d = Door {
        id: 0,
        x: 0,
        y: 0,
        r#type: "".to_string(),
    };
    let _w = Wall { id: 1, x: 0, y: 0 };
    let _t = Teleport {
        id: 2,
        x: 0,
        y: 0,
        tx: 0,
        ty: 0,
    };
    let _s = Switch {
        id: 3,
        x: 0,
        y: 0,
        toggles: Vec::new(),
    };
    let _b = Spikeball {
        id: 4,
        x: 0,
        y: 0,
        r#type: "".to_string(),
    };
    let _0 = Start { id: 6, x: 0, y: 0 };
    let _1 = Finish { id: 7, x: 0, y: 0 };
    let _p = Player {};
    let _h = Shadow {};

    for lvl in 1..10 {
        let level_string = fs::read_to_string(format!("levels/Level{}.toml", lvl))
            .expect("Something went wrong reading the file");

        let level: Level = toml::from_str(&level_string).unwrap();
        println!("Level {} \n{:#?}\nLevel {}\n", lvl, level, lvl);
    }
}
