use crate::comms::incoming::ActionResponse;
use crate::comms::incoming::{MessageIn, Meta};
use crate::reject::Error;
use crate::PLAYER_COMMS;
use uuid::Uuid;
use warp::{Rejection, Reply};

pub async fn route_action(guid: Uuid, action: ActionResponse) -> Result<impl Reply, Rejection> {
    let write = PLAYER_COMMS.write().unwrap();
    write
        .in_send_chan()
        .send(MessageIn {
            meta: Meta { guid },
            msg: action,
        })
        .map_err(|e| {
            eprintln!("failed to ack action: {:?}", e);
            warp::reject::custom(Error::InternalError)
        })?;

    Ok(warp::reply::json(&()))
}
