use crate::comms::{MessageIn, MessageInBody, Meta, ResponseKind};
use crate::reject::Error;
use crate::PLAYER_COMMS;
use serde::Deserialize;
use uuid::Uuid;
use warp::{Rejection, Reply};

#[derive(Deserialize)]
pub struct ActionDTO {
    kind: ResponseKind,
    body: MessageInBody,
}

pub async fn route_action(guid: Uuid, action: ActionDTO) -> Result<impl Reply, Rejection> {
    let write = PLAYER_COMMS.write().unwrap();
    write
        .in_send_chan()
        .send(MessageIn {
            meta: Meta {
                guid,
                response_kind: action.kind,
            },
            body: action.body,
        })
        .map_err(|e| {
            eprintln!("failed to ack action: {:?}", e);
            warp::reject::custom(Error::InternalError)
        })?;

    Ok(warp::reply::json(&()))
}
