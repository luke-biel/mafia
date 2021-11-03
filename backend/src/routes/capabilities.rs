use crate::game::action_request::ActionRequest;
use crate::game::card::{Faction, Role};
use crate::reject::Error;
use crate::GAME_STATE;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{Rejection, Reply};

#[derive(Deserialize)]
pub struct ActionDTO {
    #[serde(flatten)]
    request: ActionRequest,
}

#[derive(Serialize)]
pub struct CapabilitiesResponseDTO {
    players: Vec<Uuid>,
}

pub async fn route_capabilities(guid: Uuid, action: ActionDTO) -> Result<impl Reply, Rejection> {
    let role = {
        let read = GAME_STATE.read().unwrap();
        read.lobby
            .roles
            .get(&guid)
            .ok_or_else(|| warp::reject::custom(Error::UserNotFound))?
            .clone()
    };
    let mut players = match (role.card, action.request) {
        (Role::CityKatani, ActionRequest::CheckGoodBad) => alive_players_except_me(guid),
        (Role::CityEscort, ActionRequest::CheckCard) => alive_players_except_me(guid),
        (Role::CityDoctor, ActionRequest::Heal) => all_alive_players(),
        (Role::MafiaBlackmailer, ActionRequest::SelectBlackmailed) => alive_players_except_mafia(),
        (Role::MafiaBlank, ActionRequest::FinishPatient) => alive_players_except_mafia(),
        (_, ActionRequest::Shoot) if role.card.faction() == Faction::Mafia => {
            alive_players_except_mafia()
        }
        (Role::SyndicateAod, ActionRequest::MarkForDeath) => all_alive_players(),
        (Role::SyndicateDiaboliser, ActionRequest::SelectDiabolized) => all_alive_players(),
        (_, ActionRequest::ProposeVote) => all_alive_players(),
        (_, ActionRequest::CastVote) => all_alive_players(),
        _ => return Err(warp::reject::custom(Error::UnsupportedAction)),
    };

    if let Some(blackmailer) = role.modifiers.blackmailed_by {
        if let Some((idx, _)) = players.iter().find_position(|item| **item == blackmailer) {
            players.remove(idx);
        }
    }

    Ok(warp::reply::json(&CapabilitiesResponseDTO { players }))
}

fn alive_players_except_me(guid: Uuid) -> Vec<Uuid> {
    let read = GAME_STATE.read().unwrap();
    read.lobby
        .roles
        .iter()
        .filter(|(key, value)| **key != guid && value.alive)
        .map(|(key, _)| key)
        .cloned()
        .collect()
}

fn all_alive_players() -> Vec<Uuid> {
    let read = GAME_STATE.read().unwrap();
    read.lobby
        .roles
        .iter()
        .filter(|(_, value)| value.alive)
        .map(|(key, _)| key)
        .cloned()
        .collect()
}

fn alive_players_except_mafia() -> Vec<Uuid> {
    let read = GAME_STATE.read().unwrap();
    read.lobby
        .roles
        .iter()
        .filter(|(_, value)| value.card.faction() != Faction::Mafia)
        .map(|(key, _)| key)
        .cloned()
        .collect()
}
