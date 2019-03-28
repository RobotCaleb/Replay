use crate::objects::*;
use crate::world::{ClipType, Direction, DoorState, Entity};
use futures::Future;
use quicksilver::{
    geom::Line,
    graphics::{Background::Col, Color},
    lifecycle::{Asset, Window},
    load_file,
};
use std::collections::{HashMap, VecDeque};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Level {
    pub name: String,
    pub doors: Vec<Door>,
    pub spikeballs: Vec<Spikeball>,
    pub switches: Vec<Switch>,
    pub walls: Vec<Wall>,
    pub teleports: Vec<Teleport>,
    pub start: Start,
    pub finish: Finish,
    pub player: Player,
    pub shadow: Shadow,
    pub shadow_moves: VecDeque<(i32, i32)>,
    entities: HashMap<i32, Entity>,
    follow_distance: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigLevel {
    pub name: String,
    pub doors: Option<Vec<Door>>,
    pub spikeballs: Option<Vec<Spikeball>>,
    pub switches: Option<Vec<Switch>>,
    pub walls: Option<Vec<Wall>>,
    pub teleports: Option<Vec<Teleport>>,
    pub start: Start,
    pub finish: Finish,
    pub follow_distance: usize,
}

impl Level {
    pub fn new() -> Level {
        Level {
            name: "".to_string(),
            doors: Vec::new(),
            spikeballs: Vec::new(),
            switches: Vec::new(),
            walls: Vec::new(),
            teleports: Vec::new(),
            start: Start { x: 0, y: 0, id: 0 },
            finish: Finish { x: 0, y: 0, id: 0 },
            player: Player { x: 0, y: 0, id: 0 },
            shadow: Shadow { x: 0, y: 0, id: 0 },
            entities: HashMap::new(),
            shadow_moves: VecDeque::new(),
            follow_distance: 4,
        }
    }

    pub fn load(path: impl AsRef<Path> + 'static) -> Asset<String> {
        Asset::new(
            load_file(path).and_then(|contents| {
                Ok(String::from_utf8(contents).expect("The file must be UTF-8"))
            }),
        )
    }

    pub fn from_utf8(data: &String) -> Level {
        let cfg: ConfigLevel = toml::from_str(data).unwrap();
        Level::from_config_level(&cfg)
    }

    pub fn from_config_level(config_level: &ConfigLevel) -> Level {
        let mut level = Level::new();
        let cl = config_level.clone();
        level.start = cl.start;
        level.finish = cl.finish;
        level.doors = cl.doors.unwrap_or_else(Vec::new);
        level.spikeballs = cl.spikeballs.unwrap_or_else(Vec::new);
        level.switches = cl.switches.unwrap_or_else(Vec::new);
        level.walls = cl.walls.unwrap_or_else(Vec::new);
        level.teleports = cl.teleports.unwrap_or_else(Vec::new);
        level.follow_distance = cl.follow_distance;

        // store all entity ids in type lookup
        for n in &level.doors {
            level.entities.insert(n.id, Entity::Door(n.r#type.clone()));
        }
        for n in &level.spikeballs {
            level
                .entities
                .insert(n.id, Entity::Spikeball(n.clip.clone()));
        }
        for n in &level.switches {
            level.entities.insert(n.id, Entity::Switch);
        }
        for n in &level.walls {
            level.entities.insert(n.id, Entity::Wall);
        }
        for n in &level.teleports {
            level.entities.insert(n.id, Entity::Teleport);
        }
        level.entities.insert(level.start.id, Entity::Start);
        level.entities.insert(level.finish.id, Entity::Finish);

        level.player = Player {
            x: level.start.x,
            y: level.start.y,
            id: level.entities.keys().max().unwrap() + 1,
        };
        level.shadow = Shadow {
            x: level.start.x,
            y: level.start.y,
            id: level.player.id + 1,
        };

        level.entities.insert(level.player.id, Entity::Player);
        level.entities.insert(level.shadow.id, Entity::Shadow);

        return level;
    }

    #[allow(dead_code)]
    pub fn get_entity_pos_by_id(&self, id: i32) -> Option<(i32, i32)> {
        for n in &self.doors {
            if n.id == id {
                return Some((n.x, n.y));
            }
        }
        for n in &self.spikeballs {
            if n.id == id {
                return Some((n.x, n.y));
            }
        }
        for n in &self.switches {
            if n.id == id {
                return Some((n.x, n.y));
            }
        }
        for n in &self.walls {
            if n.id == id {
                return Some((n.x, n.y));
            }
        }
        for n in &self.teleports {
            if n.id == id {
                return Some((n.x, n.y));
            }
        }
        if self.start.id == id {
            return Some((self.start.x, self.start.y));
        }
        if self.finish.id == id {
            return Some((self.finish.x, self.finish.y));
        }
        if self.player.id == id {
            return Some((self.player.x, self.player.y));
        }
        if self.shadow.id == id {
            return Some((self.shadow.x, self.shadow.y));
        }
        return None;
    }

    #[allow(dead_code)]
    pub fn set_entity_pos_by_id(&mut self, id: i32, new_pos: (i32, i32)) {
        for n in &mut self.doors {
            if n.id == id {
                n.x = new_pos.0;
                n.y = new_pos.1;
                return;
            }
        }
        for n in &mut self.spikeballs {
            if n.id == id {
                n.x = new_pos.0;
                n.y = new_pos.1;
                return;
            }
        }
        for n in &mut self.switches {
            if n.id == id {
                n.x = new_pos.0;
                n.y = new_pos.1;
                return;
            }
        }
        for n in &mut self.walls {
            if n.id == id {
                n.x = new_pos.0;
                n.y = new_pos.1;
                return;
            }
        }
        for n in &mut self.teleports {
            if n.id == id {
                n.x = new_pos.0;
                n.y = new_pos.1;
                return;
            }
        }
        if self.start.id == id {
            self.start.x = new_pos.0;
            self.start.y = new_pos.1;
            return;
        }
        if self.finish.id == id {
            self.finish.x = new_pos.0;
            self.finish.y = new_pos.1;
            return;
        }
        if self.player.id == id {
            self.player.x = new_pos.0;
            self.player.y = new_pos.1;
            return;
        }
        if self.shadow.id == id {
            self.shadow.x = new_pos.0;
            self.shadow.y = new_pos.1;
            return;
        }
    }

    #[allow(dead_code)]
    pub fn set_spikeball_dir_by_id(&mut self, id: i32, new_dir: &Direction) {
        match self.get_entity_by_id(id) {
            Some(Entity::Spikeball(_)) => (),
            _ => panic!("Not a spikeball"),
        }

        for mut n in &mut self.spikeballs {
            if n.id == id {
                n.dir = new_dir.clone();
                return;
            }
        }

        panic!("Something is wrong. Spikeball {} can't be found", id);
    }

    #[allow(dead_code)]
    pub fn get_entity_by_id(&self, id: i32) -> Option<&Entity> {
        self.entities.get(&id)
    }

    #[allow(dead_code)]
    pub fn get_entity_id_by_pos(&self, pos: (i32, i32)) -> Option<i32> {
        for n in &self.doors {
            if (n.x, n.y) == pos {
                return Some(n.id);
            }
        }
        for n in &self.spikeballs {
            if (n.x, n.y) == pos {
                return Some(n.id);
            }
        }
        for n in &self.switches {
            if (n.x, n.y) == pos {
                return Some(n.id);
            }
        }
        for n in &self.walls {
            if (n.x, n.y) == pos {
                return Some(n.id);
            }
        }
        for n in &self.teleports {
            if (n.x, n.y) == pos {
                return Some(n.id);
            }
        }
        if (self.start.x, self.start.y) == pos {
            return Some(self.start.id);
        }
        if (self.finish.x, self.finish.y) == pos {
            return Some(self.finish.id);
        }
        if (self.player.x, self.player.y) == pos {
            return Some(self.player.id);
        }
        if (self.shadow.x, self.shadow.y) == pos {
            return Some(self.shadow.id);
        }
        return None;
    }

    pub fn is_door_open(&self, id: i32) -> bool {
        for door in &self.doors {
            if door.id == id {
                return door.state == DoorState::Open;
            }
        }
        panic!("huh, didn't find the {} door", id);
    }

    pub fn process_switches(&mut self) {
        // handle switches opening doors
        let mut poses = Vec::new();
        poses.push((self.player.x, self.player.y));
        poses.push((self.shadow.x, self.shadow.y));
        for sp in &self.spikeballs {
            poses.push((sp.x, sp.y));
        }

        let mut doors = Vec::new();
        for sw in &self.switches {
            for pos in &poses {
                if *pos == (sw.x, sw.y) {
                    doors.extend_from_slice(&sw.toggles);
                }
            }
        }
        for door in &mut self.doors {
            door.state = DoorState::Closed;
        }
        for d in doors {
            for door in &mut self.doors {
                if door.id == d {
                    door.state = DoorState::Open;
                }
            }
        }
    }

    fn process_teleports(&mut self) {
        // handle teleports teleporting the entity standing on
        let mut new_player: Player = self.player.clone();
        let mut new_shadow: Shadow = self.shadow.clone();
        let mut new_spikes: Vec<Spikeball> = Vec::new();

        for t in &self.teleports {
            if (new_player.x, new_player.y) == (t.x, t.y) {
                new_player.x = t.tx;
                new_player.y = t.ty;
            }
            if (new_shadow.x, new_shadow.y) == (t.x, t.y) {
                new_shadow.x = t.tx;
                new_shadow.y = t.ty;
            }

            for sp in &self.spikeballs {
                if (sp.x, sp.y) == (t.x, t.y) {
                    let mut s = sp.clone();
                    s.x = t.tx;
                    s.y = t.ty;
                    new_spikes.push(s);
                }
            }
        }
        self.player.x = new_player.x;
        self.player.y = new_player.y;

        self.shadow.x = new_shadow.x;
        self.shadow.y = new_shadow.y;

        for sp in &mut self.spikeballs {
            for s in &new_spikes {
                if sp.id == s.id {
                    sp.x = s.x;
                    sp.y = s.y;
                }
            }
        }
    }

    fn is_dead(&self) -> bool {
        let pp = &mut self.get_entity_pos_by_id(self.player.id);
        let sp = &mut self.get_entity_pos_by_id(self.shadow.id);
        for spike in &self.spikeballs {
            if Some((spike.x, spike.y)) == *pp || Some((spike.x, spike.y)) == *sp {
                return true;
            }
        }
        for door in &self.doors {
            if door.state == DoorState::Closed
                && (Some((door.x, door.y)) == *pp || Some((door.x, door.y)) == *sp)
            {
                return true;
            }
        }
        false
    }

    fn can_move(&self, id: i32, new_pos: (i32, i32)) -> bool {
        let a = self.get_entity_by_id(id);
        let bid = self.get_entity_id_by_pos((new_pos.0, new_pos.1));
        let b = self.get_entity_by_id(bid.unwrap_or(-1));

        // can't move out of bounds
        if a.is_none() || new_pos.0 < 0 || new_pos.0 >= 20 || new_pos.1 < 0 || new_pos.1 >= 15 {
            return false;
        }

        // nothing is allowed to move if
        // there is a spikeball on player or on shadow
        // or if player or shadow is on a closed door
        if self.is_dead() {
            return false;
        }

        match a.unwrap() {
            Entity::Door(_) => return false,
            Entity::Finish => return false,
            Entity::Start => return false,
            Entity::Switch => return false,
            Entity::Teleport => return false,
            Entity::Wall => return false,
            Entity::Spikeball(ClipType::Clip) => match b {
                Some(Entity::Wall) => return false,
                _ => return true,
            },
            Entity::Spikeball(ClipType::Block) => match b {
                Some(Entity::Wall) => return false,
                Some(Entity::Door(_)) => {
                    if !self.is_door_open(bid.unwrap()) {
                        return false;
                    }
                }
                _ => return true,
            },
            Entity::Player => match b {
                Some(Entity::Wall) => return false,
                Some(Entity::Door(_)) => {
                    if !self.is_door_open(bid.unwrap()) {
                        return false;
                    }
                }
                _ => return true,
            },
            // Shadows can always move. invalid moves will die, though
            Entity::Shadow => return true,
        }

        true
    }

    #[allow(dead_code)]
    pub fn process(level: &Level, (x, y): (i32, i32)) -> Option<(Level, Option<Vec<Level>>)> {
        let mut lvl = level.clone();
        let mut intermediates = Vec::new();

        if lvl.is_dead() {
            return None;
        }

        // first the player moves, triggering any teleports or switches
        // then the spikeballs move, triggering any teleports or switches
        // then the shadow moves, triggering any teleports or switches
        let player = lvl.player.clone();
        let new_pos = (player.x + x, player.y + y);
        if lvl.can_move(player.id, new_pos) {
            // player move
            {
                lvl.set_entity_pos_by_id(player.id, new_pos);
                lvl.shadow_moves.push_front(new_pos);
                lvl.process_switches();
                lvl.process_teleports();
                intermediates.push(lvl.clone());
                if lvl.is_dead() {
                    return Some((lvl, None));
                }
            }

            // spikeball move
            {
                let mut sp_move: (i32, i32);
                let sps = lvl.spikeballs.clone();
                for sp in &sps {
                    match sp.dir {
                        Direction::East => sp_move = (1, 0),
                        Direction::North => sp_move = (0, -1),
                        Direction::South => sp_move = (0, 1),
                        Direction::West => sp_move = (-1, 0),
                    }
                    let mut new_pos = (sp_move.0 + sp.x, sp_move.1 + sp.y);
                    let mut new_dir = &sp.dir;
                    if !lvl.can_move(sp.id, new_pos) {
                        match sp.dir {
                            Direction::East => new_dir = &Direction::West,
                            Direction::North => new_dir = &Direction::South,
                            Direction::South => new_dir = &Direction::North,
                            Direction::West => new_dir = &Direction::East,
                        }
                        lvl.set_spikeball_dir_by_id(sp.id, &new_dir);
                    }
                    match &new_dir {
                        Direction::East => sp_move = (1, 0),
                        Direction::North => sp_move = (0, -1),
                        Direction::South => sp_move = (0, 1),
                        Direction::West => sp_move = (-1, 0),
                    }
                    new_pos = (sp_move.0 + sp.x, sp_move.1 + sp.y);
                    if lvl.can_move(sp.id, new_pos) {
                        lvl.set_entity_pos_by_id(sp.id, new_pos);
                    }
                }
                lvl.process_switches();
                lvl.process_teleports();
                intermediates.push(lvl.clone());
                if lvl.is_dead() {
                    return Some((lvl, None));
                }
            }

            // shadow move
            {
                if lvl.shadow_moves.len() > lvl.follow_distance {
                    let sm = lvl.shadow_moves.pop_back().unwrap();
                    lvl.set_entity_pos_by_id(lvl.shadow.id, sm);
                    lvl.process_switches();
                    if lvl.is_dead() {
                        return Some((lvl, None));
                    }
                }
                intermediates.push(lvl.clone());
            }

            return Some((lvl, Some(intermediates)));
        }

        return None;
    }

    // this drawing is wrong!
    // I think what needs to happen is assign a z value to everything that
    // can be drawn
    // draw them with draw_ex using z
    // and draw everything one row at a time
    // starting at the top of the screen
    // That seems like it should do the right thing with the updated graphics
    pub fn draw_debug(&self, window: &mut Window) {
        for wall in &self.walls {
            wall.draw_debug(window);
        }
        for door in &self.doors {
            door.draw_debug(window);
        }
        let sws = self.switches.clone();
        for switch in sws {
            let (sx, sy) = (switch.x, switch.y);
            let mut ox = 0;
            let mut oy = 0;
            let mut found = false;
            for toggle in &switch.toggles {
                if let Some(other_pos) = self.get_entity_pos_by_id(*toggle) {
                    ox = other_pos.0;
                    oy = other_pos.1;
                    found = true;
                }
                if found {
                    window.draw(
                        &Line::new(
                            (sx as u32 * 32 + 16, sy as u32 * 32 + 16),
                            (ox as u32 * 32 + 16, oy as u32 * 32 + 16),
                        )
                        .with_thickness(1.0),
                        Col(Color::RED),
                    );
                }
            }
            switch.draw_debug(window);
        }
        for spikeball in &self.spikeballs {
            spikeball.draw_debug(window);
        }
        for teleport in &self.teleports {
            teleport.draw_debug(window);
        }

        self.start.draw_debug(window);
        self.finish.draw_debug(window);

        self.player.draw_debug(window);
        self.shadow.draw_debug(window);
    }
}
