extern crate quicksilver;

use quicksilver::{
    geom::Circle,
    graphics::{Background::Col, Color},
    lifecycle::{State, Window},
    Result,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Finish {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

impl Finish {
    pub fn draw_debug(&mut self, _window: &mut Window) -> Result<()> {
        let (x, y) = (self.x, self.y);
        _window.draw(
            &Circle::new((x as u32 * 32 + 16, y as u32 * 32 + 16), 12),
            Col(Color::INDIGO),
        );
        Ok(())
    }
}

impl State for Finish {
    fn new() -> Result<Finish> {
        Ok(Finish { id: 0, x: 0, y: 0 })
    }

    fn draw(&mut self, _window: &mut Window) -> Result<()> {
        let (x, y) = (self.x, self.y);
        _window.draw(
            &Circle::new((x as u32 * 32 + 16, y as u32 * 32 + 16), 12),
            Col(Color::INDIGO),
        );
        Ok(())
    }
}
