use futures::{Stream, StreamExt};
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

use crate::comms::MessageOut;
use crate::reject::Error;
use crate::PLAYER_COMMS;

pub fn route_events(guid: Uuid) -> Result<impl Stream<Item = MessageOut>, Error> {
    let read = PLAYER_COMMS.read().unwrap();
    let recv = read.out_recv_chan(guid)?;
    Ok(BroadcastStream::new(recv).filter_map(|v| async {
        match v {
            Ok(v) => Some(v),
            Err(e) => {
                eprintln!("failed to stream event: {:?}", e);
                None
            }
        }
    }))
}
