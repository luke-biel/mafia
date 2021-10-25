use crate::game::card::Role;
use std::collections::HashMap;
use std::fmt::Debug;
use uuid::Uuid;

pub mod card;

#[derive(Debug, Default)]
pub struct Game {
    pub(crate) players: HashMap<Uuid, String>,
    lobby: Option<Lobby>,
}

#[derive(Debug)]
pub struct Lobby {
    roles: HashMap<Uuid, Function>,
    time_of_day: TimeOfDay,
    day: usize,
    modifiers: GameModifiers,
}

#[derive(Debug)]
pub struct Function {
    card: Box<dyn Role + Send + Sync + 'static>,
    alive: bool,
    modifiers: RoleModifiers,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TimeOfDay {
    Day,
    Dusk,
    Night,
}

#[derive(Debug)]
pub struct GameModifiers {
    pub is_gun_shop_dead_during_day: bool,
}

#[derive(Debug)]
pub struct RoleModifiers {
    diabolised: bool,
    marked_by_aod: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum ActionRequest {
    CheckGoodBad,
    CheckCard,
    Heal,
    SelectBlackmailed,
    FinishPatient,
    MarkForDeath,
    SelectDiabolised,
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
