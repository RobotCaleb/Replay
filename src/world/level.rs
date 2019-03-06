use crate::objects::*;

#[derive(Clone, Debug, Deserialize)]
pub struct Level {
    pub doors: Option<Vec<Door>>,
    pub spikeballs: Option<Vec<Spikeball>>,
    pub switches: Option<Vec<Switch>>,
    pub walls: Option<Vec<Wall>>,
    pub teleports: Option<Vec<Teleport>>,
    pub start: Start,
    pub finish: Finish,
}

impl Level {
    pub fn new() -> Level {
        Level {
            doors: None,
            spikeballs: None,
            switches: None,
            walls: None,
            teleports: None,
            start: Start { x: 0, y: 0, id: 0 },
            finish: Finish { x: 0, y: 0, id: 0 },
        }
    }

    pub fn get_position(&mut self, id: i32) -> Option<(i32, i32)> {
        if let Some(ns) = self.doors.as_mut() {
            for n in ns {
                if n.id == id {
                    return Some((n.x, n.y));
                }
            }
        }
        if let Some(ns) = self.spikeballs.as_mut() {
            for n in ns {
                if n.id == id {
                    return Some((n.x, n.y));
                }
            }
        }
        if let Some(ns) = self.switches.as_mut() {
            for n in ns {
                if n.id == id {
                    return Some((n.x, n.y));
                }
            }
        }
        if let Some(ns) = self.walls.as_mut() {
            for n in ns {
                if n.id == id {
                    return Some((n.x, n.y));
                }
            }
        }
        if let Some(ns) = self.teleports.as_mut() {
            for n in ns {
                if n.id == id {
                    return Some((n.x, n.y));
                }
            }
        }
        if self.start.id == id {
            return Some((self.start.x, self.start.y));
        }
        if self.finish.id == id {
            return Some((self.finish.x, self.finish.y));
        }
        return None;
    }

    pub fn change_position(&mut self, id: i32, x: i32, y: i32) {
        if let Some(ns) = self.doors.as_mut() {
            for n in ns {
                if n.id == id {
                    n.x = x;
                    n.y = y;
                    return;
                }
            }
        }
        if let Some(ns) = self.spikeballs.as_mut() {
            for n in ns {
                if n.id == id {
                    n.x = x;
                    n.y = y;
                    return;
                }
            }
        }
        if let Some(ns) = self.switches.as_mut() {
            for n in ns {
                if n.id == id {
                    n.x = x;
                    n.y = y;
                    return;
                }
            }
        }
        if let Some(ns) = self.walls.as_mut() {
            for n in ns {
                if n.id == id {
                    n.x = x;
                    n.y = y;
                    return;
                }
            }
        }
        if let Some(ns) = self.teleports.as_mut() {
            for n in ns {
                if n.id == id {
                    n.x = x;
                    n.y = y;
                    return;
                }
            }
        }
        if self.start.id == id {
            self.start.x = x;
            self.start.y = y;
            return;
        }
        if self.finish.id == id {
            self.finish.x = x;
            self.finish.y = y;
            return;
        }
    }
}
