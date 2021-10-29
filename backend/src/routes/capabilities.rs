use crate::game::action_request::ActionRequest;
use crate::game::card::{Faction, Value};
use crate::reject::Error;
use crate::GAME_STATE;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{Rejection, Reply};

#[derive(Deserialize)]
pub struct ActionDTO {
    request: ActionRequest,
}

#[derive(Serialize)]
pub struct CapabilitiesResponseDTO {
    players: Vec<Uuid>,
}

pub async fn route_capabilities(guid: Uuid, action: ActionDTO) -> Result<impl Reply, Rejection> {
    let card = {
        let read = GAME_STATE.read().unwrap();
        read.lobby
            .roles
            .get(&guid)
            .ok_or_else(|| warp::reject::custom(Error::UserNotFound))?
            .card
    };
    match (card.faction(), card.value(), action.request) {
        (Faction::City, Value::King, ActionRequest::CheckGoodBad) => {
            Ok(warp::reply::json(&alive_players_except_me(guid)))
        }
        (Faction::City, Value::Queen, ActionRequest::CheckCard) => {
            Ok(warp::reply::json(&alive_players_except_me(guid)))
        }
        (Faction::City, Value::Jack, ActionRequest::Heal) => {
            Ok(warp::reply::json(&all_alive_players()))
        }
        (Faction::Mafia, Value::King, ActionRequest::SelectBlackmailed) => {
            Ok(warp::reply::json(&alive_players_except_mafia()))
        }
        (Faction::Mafia, Value::Jack, ActionRequest::FinishPatient) => {
            Ok(warp::reply::json(&alive_players_except_mafia()))
        }
        (Faction::Mafia, _, ActionRequest::Shoot) => {
            Ok(warp::reply::json(&alive_players_except_mafia()))
        }
        (Faction::Syndicate, Value::King, ActionRequest::MarkForDeath) => {
            Ok(warp::reply::json(&all_alive_players()))
        }
        (Faction::Syndicate, Value::Queen, ActionRequest::SelectDiabolized) => {
            Ok(warp::reply::json(&all_alive_players()))
        }
        (_, _, ActionRequest::ProposeVote) => Ok(warp::reply::json(&all_alive_players())),
        (_, _, ActionRequest::CastVote) => Ok(warp::reply::json(&all_alive_players())),
        _ => Err(warp::reject::custom(Error::UnsupportedAction)),
    }
}

fn alive_players_except_me(guid: Uuid) -> CapabilitiesResponseDTO {
    let read = GAME_STATE.read().unwrap();
    CapabilitiesResponseDTO {
        players: read
            .lobby
            .roles
            .iter()
            .filter(|(key, value)| **key != guid && value.alive)
            .map(|(key, _)| key)
            .cloned()
            .collect(),
    }
}

fn all_alive_players() -> CapabilitiesResponseDTO {
    let read = GAME_STATE.read().unwrap();
    CapabilitiesResponseDTO {
        players: read
            .lobby
            .roles
            .iter()
            .filter(|(_, value)| value.alive)
            .map(|(key, _)| key)
            .cloned()
            .collect(),
    }
}

fn alive_players_except_mafia() -> CapabilitiesResponseDTO {
    let read = GAME_STATE.read().unwrap();
    CapabilitiesResponseDTO {
        players: read
            .lobby
            .roles
            .iter()
            .filter(|(_, value)| value.card.faction() != Faction::Mafia)
            .map(|(key, _)| key)
            .cloned()
            .collect(),
    }
}
