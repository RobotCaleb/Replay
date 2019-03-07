extern crate quicksilver;

use quicksilver::{
    geom::Circle,
    graphics::{Background::Col, Color},
    lifecycle::Window,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Switch {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub toggles: Vec<i32>,
}

impl Switch {
    #[allow(dead_code)]
    pub fn new(id: i32, x: i32, y: i32, toggles: Vec<i32>) -> Switch {
        Switch {
            id: id,
            x: x,
            y: y,
            toggles: toggles,
        }
    }

    pub fn draw_debug(&mut self, _window: &mut Window) {
        let (x, y) = (self.x, self.y);
        _window.draw(
            &Circle::new((x as u32 * 32 + 16, y as u32 * 32 + 16), 12),
            Col(Color::RED),
        );
    }
}
