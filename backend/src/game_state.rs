use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct GameState {
    players: HashMap<Uuid, String>,
}

impl GameState {
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
