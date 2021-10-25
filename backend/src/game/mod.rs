use crate::game::card::Role;
use std::collections::HashMap;
use uuid::Uuid;

pub mod card;

#[derive(Default)]
pub struct Game {
    players: HashMap<Uuid, String>,
    lobby: Option<Lobby>,
}

pub struct Lobby {
    roles: HashMap<Uuid, Function>,
    time_of_day: TimeOfDay,
    day: usize,
    modifiers: GameModifiers,
}

pub struct Function {
    card: Box<dyn Role + Send + Sync + 'static>,
    alive: bool,
    modifiers: RoleModifiers,
}

#[derive(PartialEq)]
pub enum TimeOfDay {
    Day,
    Dusk,
    Night,
}

pub struct GameModifiers {
    pub is_gun_shop_dead_during_day: bool,
}

pub struct RoleModifiers {
    diabolised: bool,
    marked_by_aod: bool,
}

pub enum ActionRequest {
    CheckGoodBad,
    CheckCard,
    Heal,
}

impl Game {
    pub fn register(&mut self, name: String) -> Uuid {
        let id = Uuid::new_v4();
        self.players.insert(id, name);
        id
    }

    pub fn remove(&mut self, guid: Uuid) {
        self.players.remove(&guid);
    }

    pub fn find(&self, guid: Uuid) -> Option<String> {
        self.players.get(&guid).cloned()
    }
}
