extern crate quicksilver;

use quicksilver::{
    geom::Rectangle,
    graphics::{Background::Col, Color},
    lifecycle::Window,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shadow {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

impl Shadow {
    #[allow(dead_code)]
    pub fn new(id: i32, x: i32, y: i32) -> Shadow {
        Shadow { id: id, x: x, y: y }
    }

    pub fn draw_debug(&mut self, window: &mut Window) {
        let (x, y) = (self.x, self.y);
        window.draw(
            &Rectangle::new((x as u32 * 32 + 11, y as u32 * 32 + 8), (10, 16)),
            Col(Color::ORANGE),
        );
    }
}
