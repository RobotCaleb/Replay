extern crate quicksilver;

use quicksilver::{
    geom::{Rectangle, Transform},
    graphics::{Background::Col, Color},
    lifecycle::{State, Window},
    Result,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Spikeball {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub r#type: String,
}

impl Spikeball {
    pub fn draw_debug(&mut self, _window: &mut Window) -> Result<()> {
        let (x, y) = (self.x, self.y);
        _window.draw_ex(
            &Rectangle::new((x as u32 * 32 + 8, y as u32 * 32 + 8), (16, 16)),
            Col(Color::GREEN),
            Transform::rotate(22.5),
            0,
        );
        _window.draw_ex(
            &Rectangle::new((x as u32 * 32 + 8, y as u32 * 32 + 8), (16, 16)),
            Col(Color::GREEN),
            Transform::rotate(67.5),
            0,
        );
        Ok(())
    }
}

impl State for Spikeball {
    fn new() -> Result<Spikeball> {
        Ok(Spikeball {
            id: 0,
            x: 0,
            y: 0,
            r#type: "".to_string(),
        })
    }

    fn draw(&mut self, _window: &mut Window) -> Result<()> {
        let (x, y) = (self.x, self.y);
        _window.draw_ex(
            &Rectangle::new((x as u32 * 32 + 8, y as u32 * 32 + 8), (16, 16)),
            Col(Color::GREEN),
            Transform::rotate(22.5),
            0,
        );
        _window.draw_ex(
            &Rectangle::new((x as u32 * 32 + 8, y as u32 * 32 + 8), (16, 16)),
            Col(Color::GREEN),
            Transform::rotate(67.5),
            0,
        );
        Ok(())
    }
}
