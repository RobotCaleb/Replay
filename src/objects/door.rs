extern crate quicksilver;

use quicksilver::{
    geom::{Rectangle, Transform},
    graphics::{Background::Col, Color},
    lifecycle::Window,
};

use crate::world::{DoorState, DoorType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Door {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub r#type: DoorType,
    pub state: DoorState,
}

impl Door {
    #[allow(dead_code)]
    pub fn new(id: i32, x: i32, y: i32, r#type: DoorType, state: DoorState) -> Door {
        Door {
            id: id,
            x: x,
            y: y,
            r#type: r#type,
            state: state,
        }
    }

    pub fn draw_debug(&mut self, _window: &mut Window) {
        let (x, y) = (self.x, self.y);
        if self.state == DoorState::Open {
            match self.r#type {
                DoorType::NorthSouth => {
                    _window.draw_ex(
                        &Rectangle::new((x as u32 * 32 + 12, y as u32 * 32 + 4), (8, 8)),
                        Col(Color::BLACK),
                        Transform::rotate(0),
                        0,
                    );
                    _window.draw_ex(
                        &Rectangle::new((x as u32 * 32 + 12, y as u32 * 32 + 24), (8, 8)),
                        Col(Color::BLACK),
                        Transform::rotate(0),
                        0,
                    );
                }
                DoorType::EastWest => {
                    _window.draw_ex(
                        &Rectangle::new((x as u32 * 32 + 4, y as u32 * 32 + 12), (8, 8)),
                        Col(Color::BLACK),
                        Transform::rotate(0),
                        0,
                    );
                    _window.draw_ex(
                        &Rectangle::new((x as u32 * 32 + 24, y as u32 * 32 + 12), (8, 8)),
                        Col(Color::BLACK),
                        Transform::rotate(0),
                        0,
                    );
                }
            }
        } else {
            let tf = if self.r#type == DoorType::EastWest {
                90
            } else {
                0
            };
            _window.draw_ex(
                &Rectangle::new((x as u32 * 32 + 12, y as u32 * 32 + 4), (8, 24)),
                Col(Color::BLACK),
                Transform::rotate(tf),
                0,
            );
        }
    }
}
