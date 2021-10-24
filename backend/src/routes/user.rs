use crate::GAME_STATE;
use uuid::Uuid;
use warp::http::{Response, StatusCode};
use warp::hyper::Body;
use warp::{Rejection, Reply};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserReplyDTO {
    name: String,
}

#[derive(Serialize)]
pub struct EmptyUserReply;

// TODO: Change this route to accept Uuid from header / rename it to "refresh"?
pub async fn route_user(guid: Uuid) -> Result<Response<Body>, Rejection> {
    let gd = GAME_STATE.read().unwrap();
    match gd.find(guid) {
        Some(name) => Ok(warp::reply::json(&UserReplyDTO { name }).into_response()),
        None => Ok(warp::reply::with_status(
            warp::reply::json(&EmptyUserReply),
            StatusCode::NO_CONTENT,
        )
        .into_response()),
    }
}
