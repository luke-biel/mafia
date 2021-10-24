use crate::reject::Error;
use crate::routes::MAFIA_GUID_COOKIE_NAME;
use crate::{GAME_STATE, PLAYER_COMMS};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::http::header::SET_COOKIE;
use warp::http::HeaderValue;
use warp::{Rejection, Reply};

#[derive(Deserialize)]
pub struct RegisterDTO {
    name: String,
}

#[derive(Serialize)]
struct RegisterReplyDTO {
    guid: Uuid,
}

pub async fn route_register(body: RegisterDTO) -> Result<impl Reply, Rejection> {
    let game_state = GAME_STATE.clone();
    let mut game_state = game_state.write().unwrap();
    let guid = game_state.register(body.name);
    let player_comms = PLAYER_COMMS.clone();
    let mut player_comms = player_comms.write().unwrap();
    if let Err(e) = player_comms.register(guid) {
        game_state.remove(guid);
        return Err(warp::reject::custom(e));
    }
    let mut resp = warp::reply::json(&RegisterReplyDTO { guid }).into_response();
    let headers = resp.headers_mut();

    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("{}={}", MAFIA_GUID_COOKIE_NAME, guid)).map_err(|e| {
            eprintln!("failed to set header {}", e); // TODO: tracing
            warp::reject::custom(Error::InternalError)
        })?,
    );

    Ok(resp)
}
