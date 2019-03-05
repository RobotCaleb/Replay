#[derive(Debug, Deserialize)]
pub struct Switch {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub toggles: Vec<i32>,
}
