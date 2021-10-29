use crate::game::TimeOfDay;
use crate::GAME_STATE;
use serde::Serialize;
use uuid::Uuid;
use warp::{Rejection, Reply};

#[derive(Serialize)]
pub struct GameStateResponseDTO {
    players: Vec<PlayerDTO>,
    day: usize,
    time_of_day: TimeOfDay,
}

#[derive(Serialize)]
pub struct PlayerDTO {
    id: Uuid,
    name: String,
    alive: bool,
}

pub async fn route_game_state() -> Result<impl Reply, Rejection> {
    let read = GAME_STATE.read().unwrap();

    Ok(warp::reply::json(&GameStateResponseDTO {
        players: read
            .players
            .iter()
            .map(|(id, name)| PlayerDTO {
                id: *id,
                name: name.clone(),
                alive: read.lobby.roles.get(id).map(|p| p.alive).unwrap_or(false),
            })
            .collect(),
        day: read.lobby.day,
        time_of_day: read.lobby.time_of_day,
    }))
}
