extern crate quicksilver;

use quicksilver::{
    geom::Circle,
    graphics::{Background::Col, Color},
    lifecycle::Window,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Finish {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

impl Finish {
    #[allow(dead_code)]
    pub fn new(id: i32, x: i32, y: i32) -> Finish {
        Finish { id: id, x: x, y: y }
    }

    pub fn draw_debug(&mut self, window: &mut Window) {
        let (x, y) = (self.x, self.y);
        window.draw(
            &Circle::new((x as u32 * 32 + 16, y as u32 * 32 + 16), 12),
            Col(Color::INDIGO),
        );
    }
}
