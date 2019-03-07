extern crate quicksilver;

use quicksilver::{
    geom::{Circle, Line},
    graphics::{Background::Col, Color},
    lifecycle::Window,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Teleport {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub tx: i32,
    pub ty: i32,
}

impl Teleport {
    #[allow(dead_code)]
    pub fn new(id: i32, x: i32, y: i32, tx: i32, ty: i32) -> Teleport {
        Teleport {
            id: id,
            x: x,
            y: y,
            tx: tx,
            ty: ty,
        }
    }

    pub fn draw_debug(&mut self, _window: &mut Window) {
        let (x, y) = (self.x, self.y);
        let (tx, ty) = (self.tx, self.ty);

        _window.draw(
            &Circle::new((x as u32 * 32 + 16, y as u32 * 32 + 16), 12),
            Col(Color::MAGENTA),
        );
        _window.draw(
            &Line::new(
                (x as u32 * 32 + 16, y as u32 * 32 + 16),
                (tx as u32 * 32 + 16, ty as u32 * 32 + 16),
            )
            .with_thickness(1.0),
            Col(Color::MAGENTA),
        );
    }
}
