extern crate quicksilver;

use quicksilver::{
    geom::Rectangle,
    graphics::{Background::Col, Color},
    lifecycle::Window,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Wall {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

impl Wall {
    #[allow(dead_code)]
    pub fn new(id: i32, x: i32, y: i32) -> Wall {
        Wall { id: id, x: x, y: y }
    }

    pub fn draw_debug(&mut self, _window: &mut Window) {
        let (x, y) = (self.x, self.y);
        _window.draw(
            &Rectangle::new((x as u32 * 32, y as u32 * 32), (32, 32)),
            Col(Color::BLUE),
        );
    }
}
