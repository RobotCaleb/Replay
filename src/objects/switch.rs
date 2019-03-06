extern crate quicksilver;

use quicksilver::{
    geom::Circle,
    graphics::{Background::Col, Color},
    lifecycle::{State, Window},
    Result,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Switch {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub toggles: Vec<i32>,
}

impl Switch {
    pub fn draw_debug(&mut self, _window: &mut Window) -> Result<()> {
        let (x, y) = (self.x, self.y);
        _window.draw(
            &Circle::new((x as u32 * 32 + 16, y as u32 * 32 + 16), 12),
            Col(Color::RED),
        );
        Ok(())
    }
}

impl State for Switch {
    fn new() -> Result<Switch> {
        Ok(Switch {
            id: 0,
            x: 0,
            y: 0,
            toggles: Vec::new(),
        })
    }

    fn draw(&mut self, _window: &mut Window) -> Result<()> {
        let (x, y) = (self.x, self.y);
        _window.draw(
            &Circle::new((x as u32 * 32 + 16, y as u32 * 32 + 16), 12),
            Col(Color::RED),
        );
        Ok(())
    }
}
