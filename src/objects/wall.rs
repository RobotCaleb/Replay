extern crate quicksilver;

use quicksilver::{
    geom::Rectangle,
    graphics::{Background::Col, Color},
    lifecycle::{State, Window},
    Result,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Wall {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

impl Wall {
    pub fn draw_debug(&mut self, _window: &mut Window) -> Result<()> {
        let (x, y) = (self.x, self.y);
        _window.draw(
            &Rectangle::new((x as u32 * 32, y as u32 * 32), (32, 32)),
            Col(Color::BLUE),
        );
        Ok(())
    }
}

impl State for Wall {
    fn new() -> Result<Wall> {
        Ok(Wall { id: 0, x: 0, y: 0 })
    }

    fn draw(&mut self, _window: &mut Window) -> Result<()> {
        let (x, y) = (self.x, self.y);
        _window.draw(
            &Rectangle::new((x as u32 * 32, y as u32 * 32), (32, 32)),
            Col(Color::BLUE),
        );
        Ok(())
    }
}
