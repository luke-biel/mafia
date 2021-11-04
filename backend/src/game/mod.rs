use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;

use itertools::Itertools;
use uuid::Uuid;

use crate::comms::incoming::ActionResponse;
use crate::comms::incoming::Meta;
use crate::comms::outgoing::{Broadcast, MessageOut, Notification};
use action_request::ActionRequest;
use lobby::{Lobby, TimeOfDay};

use crate::game::card::Faction;
use crate::GAME_STATE;
use crate::PLAYER_COMMS;

pub mod action_request;
pub mod card;
pub mod lobby;

#[derive(Debug, Default)]
pub struct Game {
    pub players: HashMap<Uuid, String>,
    pub lobby: Lobby,
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
    {
        let comms = PLAYER_COMMS.read().unwrap();
        let gd = GAME_STATE.read().unwrap();

        for (id, sender) in comms.out_send_all() {
            if let Err(e) = sender.send(Broadcast::game_start()) {
                eprintln!("failed to send GameStart to {}: {}", id, e);
            }
            let faction = gd.lobby.roles.get(&id).unwrap();
            if let Err(e) = sender.send(Notification::role_assignment(faction.card)) {
                eprintln!("failed to send GameStart to {}: {}", id, e);
            }
        }
    }

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
                    v.insert(vec![action]);
                }
                Entry::Occupied(mut o) => {
                    o.get_mut().push(action);
                }
            }
            expected_response_count += 1;
            chan.send(action.into_message_out()).unwrap();
        }

        let mut deltas = HashMap::new();

        let mut recv = {
            let comms = PLAYER_COMMS.read().unwrap();
            comms.in_recv_chan()
        };

        while deltas.len() < expected_response_count && is_sufficient(&deltas, &expected_responses)
        {
            let delta = recv.recv().await.unwrap();

            deltas.insert(delta.meta, delta.msg);
        }

        for (id, message) in calculate_new_game_state(deltas) {
            let comms = PLAYER_COMMS.read().unwrap();
            let chan = comms.out_send_chan(id).unwrap();
            chan.send(message).unwrap();
        }
    }
}

fn calculate_new_game_state(deltas: HashMap<Meta, ActionResponse>) -> Vec<(Uuid, MessageOut)> {
    let mut lobby = {
        let gd = GAME_STATE.read().unwrap();
        gd.lobby.clone()
    };
    let notifications = lobby.update(deltas);
    let mut gd = GAME_STATE.write().unwrap();
    gd.lobby = lobby;
    notifications
}

fn is_sufficient(
    delta: &HashMap<Meta, ActionResponse>,
    expected: &HashMap<Uuid, Vec<ActionRequest>>,
) -> bool {
    delta.iter().all(|(meta, item)| {
        expected
            .get(&meta.guid)
            .map(|exists| exists.iter().any(|req| req.is_sufficient(item)))
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
