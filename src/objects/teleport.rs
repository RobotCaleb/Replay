extern crate quicksilver;

use quicksilver::{
    geom::{Circle, Line},
    graphics::{Background::Col, Color},
    lifecycle::{State, Window},
    Result,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Teleport {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub tx: i32,
    pub ty: i32,
}

impl Teleport {
    pub fn draw_debug(&mut self, _window: &mut Window) -> Result<()> {
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
        Ok(())
    }
}

impl State for Teleport {
    fn new() -> Result<Teleport> {
        Ok(Teleport {
            id: 0,
            x: 0,
            y: 0,
            tx: 0,
            ty: 0,
        })
    }

    fn draw(&mut self, _window: &mut Window) -> Result<()> {
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
        Ok(())
    }
}
