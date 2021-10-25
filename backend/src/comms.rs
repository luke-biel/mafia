use crate::game::action_request::ActionRequest;
use crate::reject::Error;
use serde::Serialize;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum MessageIn {}

#[derive(Debug, Clone, Serialize)]
pub enum MessageOut {
    Action(ActionRequest),
}

pub struct CommunicationBuffer {
    pub send_in: Arc<Mutex<broadcast::Sender<MessageIn>>>,
    pub send_out: Arc<Mutex<broadcast::Sender<MessageOut>>>,
}

#[derive(Default)]
pub struct UserBuffers {
    buffers: HashMap<Uuid, CommunicationBuffer>,
}

impl CommunicationBuffer {
    pub fn new(
        send_in: broadcast::Sender<MessageIn>,
        send_out: broadcast::Sender<MessageOut>,
    ) -> Self {
        Self {
            send_in: Arc::new(Mutex::new(send_in)),
            send_out: Arc::new(Mutex::new(send_out)),
        }
    }
}

impl UserBuffers {
    pub fn register(&mut self, id: Uuid) -> Result<(), Error> {
        match self.buffers.entry(id) {
            Entry::Occupied(_) => return Err(Error::InternalError),
            Entry::Vacant(ve) => {
                let (send_in, _) = broadcast::channel(1024);
                let (send_out, _) = broadcast::channel(1024);
                ve.insert(CommunicationBuffer::new(send_in, send_out));
            }
        };

        Ok(())
    }

    pub fn out_recv_chan(&self, guid: Uuid) -> Result<broadcast::Receiver<MessageOut>, Error> {
        Ok(self
            .buffers
            .get(&guid)
            .ok_or(Error::UserNotFound)? // TODO: handle missing
            .send_out
            .lock()
            .unwrap()
            .subscribe())
    }

    pub fn out_send_chan(&self, guid: Uuid) -> Result<broadcast::Sender<MessageOut>, Error> {
        Ok(self
            .buffers
            .get(&guid)
            .ok_or(Error::UserNotFound)? // TODO: handle missing
            .send_out
            .lock()
            .unwrap()
            .clone())
    }
}
