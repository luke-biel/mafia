use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;

use itertools::Itertools;
use uuid::Uuid;

use crate::comms::{MessageInBody, Meta, ResponseKind};
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
        let requests = construct_requests()
            .into_iter()
            .map(|(id, actions)| actions.into_iter().map(move |action| (id, action)))
            .flatten();

        let mut expected_response_count = 0;
        let mut expected_responses = HashMap::new();

        for (id, action) in requests {
            let comms = PLAYER_COMMS.read().unwrap();
            let chan = comms.out_send_chan(id).unwrap();
            match expected_responses.entry(id) {
                Entry::Vacant(v) => {
                    v.insert(vec![action.expected_response()]);
                }
                Entry::Occupied(mut o) => {
                    o.get_mut().push(action.expected_response());
                }
            }
            expected_response_count += 1;
            chan.send(action.into_message()).unwrap();
        }

        println!("{:?}", expected_responses);

        let mut deltas = HashMap::new();

        let mut recv = {
            let comms = PLAYER_COMMS.read().unwrap();
            comms.in_recv_chan()
        };

        while deltas.len() < expected_response_count && is_sufficient(&deltas, &expected_responses)
        {
            let delta = recv.recv().await.unwrap();

            deltas.insert(delta.meta, delta.body);
        }

        println!("{:?}", deltas)
    }
}

fn is_sufficient(
    delta: &HashMap<Meta, MessageInBody>,
    expected: &HashMap<Uuid, Vec<ResponseKind>>,
) -> bool {
    delta.keys().all(|item| {
        expected
            .get(&item.guid)
            .map(|exists| exists.contains(&item.response_kind))
            .unwrap_or(false)
    })
}

fn construct_requests() -> HashMap<Uuid, Vec<ActionRequest>> {
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

    for (id, function) in alive {
        requests.insert(*id, function.card.request_user_action(time_of_day, day));
    }

    // Allow mafia to shoot
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

    requests
}
