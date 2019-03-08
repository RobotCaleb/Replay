use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum DoorType {
    EastWest,
    NorthSouth,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "state")]
pub enum DoorState {
    Closed,
    Open,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "dir")]
pub enum Direction {
    East,
    North,
    South,
    West,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "clip")]
pub enum ClipType {
    Block,
    Clip,
}

#[derive(Clone, Debug)]
pub enum Entity {
    Door(DoorType),
    Finish,
    Player,
    Shadow,
    Spikeball(ClipType),
    Start,
    Switch,
    Teleport,
    Wall,
}

#[derive(Copy, Clone, Debug)]
pub enum Input {
    Exit,
    Move((i32, i32)),
    NextLevel,
    PrevLevel,
    Restart,
    Undo,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(dead_code)]
pub fn log(str: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        console!(log, str);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("{}", str);
    }
}
