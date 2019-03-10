extern crate quicksilver;

use quicksilver::{
    geom::{Rectangle, Transform},
    graphics::{Background::Col, Color},
    lifecycle::Window,
};

use crate::world::{ClipType, Direction};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spikeball {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub dir: Direction,
    pub clip: ClipType,
}

impl Spikeball {
    #[allow(dead_code)]
    pub fn new(id: i32, x: i32, y: i32, dir: Direction, clip: ClipType) -> Spikeball {
        Spikeball {
            id: id,
            x: x,
            y: y,
            dir: dir,
            clip: clip,
        }
    }

    pub fn draw_debug(&mut self, window: &mut Window) {
        let (x, y) = (self.x, self.y);
        window.draw_ex(
            &Rectangle::new((x as u32 * 32 + 8, y as u32 * 32 + 8), (16, 16)),
            Col(Color::GREEN),
            Transform::rotate(22.5),
            0,
        );
        window.draw_ex(
            &Rectangle::new((x as u32 * 32 + 8, y as u32 * 32 + 8), (16, 16)),
            Col(Color::GREEN),
            Transform::rotate(67.5),
            0,
        );
    }
}
