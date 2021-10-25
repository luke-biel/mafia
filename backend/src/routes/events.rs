use futures::{Stream, TryStreamExt};
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

use crate::comms::MessageOut;
use crate::PLAYER_COMMS;
use crate::reject::Error;

pub fn route_events(guid: Uuid) -> Result<impl Stream<Item=Result<MessageOut, ()>>, Error> {
    let read = PLAYER_COMMS.read().unwrap();
    let recv = read.out_recv_chan(guid)?;
    Ok(BroadcastStream::new(recv).map_err(|_| ()))
}
