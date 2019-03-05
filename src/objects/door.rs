#[derive(Debug, Deserialize)]
pub struct Door {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub r#type: String,
}
