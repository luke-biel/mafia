use crate::GAME_STATE;
use serde::Serialize;
use std::convert::Infallible;
use uuid::Uuid;
use warp::Reply;

#[derive(Serialize)]
pub struct LobbyResponseDTO {
    players: Vec<PlayerDTO>,
}

#[derive(Serialize)]
pub struct PlayerDTO {
    id: Uuid,
    name: String,
}

pub async fn route_lobby() -> Result<impl Reply, Infallible> {
    let game = GAME_STATE.read().unwrap();
    Ok(warp::reply::json(&LobbyResponseDTO {
        players: game
            .players
            .iter()
            .map(|(id, name)| PlayerDTO {
                id: *id,
                name: name.clone(),
            })
            .collect(),
    }))
}
