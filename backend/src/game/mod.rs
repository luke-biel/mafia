use std::collections::HashMap;
use std::fmt::Debug;

use itertools::Itertools;
use uuid::Uuid;

use action_request::ActionRequest;

use crate::game::card::{Faction, Role};
use crate::GAME_STATE;
use crate::PLAYER_COMMS;

pub mod action_request;
pub mod card;

#[derive(Debug, Default)]
pub struct Game {
    pub players: HashMap<Uuid, String>,
    pub lobby: Lobby,
}

#[derive(Debug, Default)]
pub struct Lobby {
    pub roles: HashMap<Uuid, Function>,
    pub time_of_day: TimeOfDay,
    pub day: usize,
    pub modifiers: GameModifiers,
}

#[derive(Debug)]
pub struct Function {
    pub card: &'static (dyn Role + Send + Sync),
    pub alive: bool,
    pub modifiers: RoleModifiers,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TimeOfDay {
    Day,
    Dusk,
    Night,
}

#[derive(Debug, Default)]
pub struct GameModifiers {
    pub is_gun_shop_dead_during_day: bool,
}

#[derive(Debug, Default)]
pub struct RoleModifiers {
    pub diabolized: bool,
    pub marked_by_aod: bool,
    pub blackmailed: bool,
}

impl const Default for TimeOfDay {
    fn default() -> Self {
        Self::Night
    }
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

    pub fn top_value_living_mafia(&self) -> Option<Uuid> {
        self.lobby
            .roles
            .iter()
            .filter(|(_, f)| f.card.faction() == Faction::Mafia)
            .sorted_by(|(_, a), (_, b)| a.card.value().cmp(&b.card.value()))
            .map(|(id, _)| *id)
            .next()
    }
}

pub async fn start_game() {
    loop {
        let (time_of_day, day, top_mafia, mut requests) = unpack_current_game_state();

        push_reoccuring_action_requests(time_of_day, day, top_mafia, &mut requests);

        let mut blocking = requests
            .into_iter()
            .map(|(id, actions)| actions.into_iter().map(move |action| (id, action)))
            .flatten()
            .sorted_by(|(_, a), (_, b)| a.blocking().cmp(&b.blocking()));

        let non_blocking: Vec<_> = blocking.take_while_ref(|(_, a)| !a.blocking()).collect();

        for (id, action) in non_blocking {
            let comms = PLAYER_COMMS.read().unwrap();
            let chan = comms.out_send_chan(id).unwrap();
            chan.send(action.into_message()).unwrap();
        }

        let mut deltas = Vec::new();

        for (id, action) in blocking {
            let (send, mut recv) = {
                let comms = PLAYER_COMMS.read().unwrap();
                let send = comms.out_send_chan(id).unwrap();
                let recv = comms.in_recv_chan(id).unwrap();
                (send, recv)
            };

            send.send(action.into_message()).unwrap();

            let msg = recv.recv().await.unwrap();

            deltas.push((id, msg));
        }
    }
}

fn push_reoccuring_action_requests(time_of_day: TimeOfDay, day: usize, top_mafia: Option<Uuid>, requests: &mut HashMap<Uuid, Vec<ActionRequest>>) {
    // Let mafia shoot
    if time_of_day == TimeOfDay::Night && day != 0 {
        if let Some(top_mafia) = top_mafia {
            let item = requests.get_mut(&top_mafia).unwrap();
            item.push(ActionRequest::Shoot);
        }
    }

    // Let city members add voting proposals
    if time_of_day == TimeOfDay::Day {
        requests
            .iter_mut()
            .for_each(|(_, actions)| actions.push(ActionRequest::ProposeVote))
    }

    // Let city members cast a vote
    if time_of_day == TimeOfDay::Dusk {
        requests
            .iter_mut()
            .for_each(|(_, actions)| actions.push(ActionRequest::CastVote))
    }
}

fn unpack_current_game_state() -> (TimeOfDay, usize, Option<Uuid>, HashMap<Uuid, Vec<ActionRequest>>) {
    let read = GAME_STATE.read().unwrap();
    let time_of_day = read.lobby.time_of_day;
    let day = read.lobby.day;
    let alive = read
        .lobby
        .roles
        .iter()
        .filter(|(_, f)| f.alive)
        .collect::<Vec<_>>();
    let top_mafia = read.top_value_living_mafia();
    let mut requests = HashMap::new();

    // Every role has it's own actions added here
    for (id, function) in alive {
        requests.insert(*id, function.card.request_user_action(time_of_day, day));
    }
    (time_of_day, day, top_mafia, requests)
}
