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

#[derive(Clone, Debug)]
pub enum Input {
    Exit,
    Move((i32, i32)),
    NextLevel,
    PrevLevel,
    Restart,
    Undo,
}
