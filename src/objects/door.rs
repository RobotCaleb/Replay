extern crate quicksilver;

use quicksilver::{
    geom::Rectangle,
    graphics::{Background::Col, Color},
    lifecycle::{State, Window},
    Result,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Door {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub r#type: String,
}

impl Door {
    pub fn draw_debug(&mut self, _window: &mut Window) -> Result<()> {
        let (x, y) = (self.x, self.y);
        _window.draw(
            &Rectangle::new((x as u32 * 32 + 4, y as u32 * 32 + 4), (24, 24)),
            Col(Color::BLACK),
        );
        Ok(())
    }
}

impl State for Door {
    fn new() -> Result<Door> {
        Ok(Door {
            id: 0,
            x: 0,
            y: 0,
            r#type: "".to_string(),
        })
    }

    fn draw(&mut self, _window: &mut Window) -> Result<()> {
        let (x, y) = (self.x, self.y);
        _window.draw(
            &Rectangle::new((x as u32 * 32 + 4, y as u32 * 32 + 4), (24, 24)),
            Col(Color::BLACK),
        );
        Ok(())
    }
}
