use crate::comms::MessageOut;
use crate::PLAYER_COMMS;
use futures::{Stream, TryStreamExt};
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

pub fn route_events(guid: Uuid) -> impl Stream<Item = Result<MessageOut, ()>> {
    let read = PLAYER_COMMS.read().unwrap();
    let recv = read.recv_out(guid);
    BroadcastStream::new(recv).map_err(|_| ())
}
