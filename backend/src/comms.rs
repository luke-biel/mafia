use crate::game::action_request::ActionRequest;
use crate::reject::Error;
use serde::Deserialize;
use serde::Serialize;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub enum ResponseKind {
    CheckGoodBadTarget,
    CheckCardTarget,
    HealTarget,
    BlackmailTarget,
    FinishTarget,
    DeathMarkTarget,
    DiabolizationTarget,
    ShootTarget,
    VoteProposal,
    VoteTarget,
}

#[derive(Debug, Clone)]
pub struct MessageIn {
    pub meta: Meta,
    pub body: MessageInBody,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Meta {
    pub guid: Uuid,
    pub response_kind: ResponseKind,
}

#[derive(Debug, Clone, Deserialize)]
pub enum MessageInBody {
    Empty,
}

#[derive(Debug, Clone, Serialize)]
pub enum MessageOut {
    Action(ActionRequest),
}

pub struct UserBuffers {
    buffers: HashMap<Uuid, Arc<Mutex<broadcast::Sender<MessageOut>>>>,
    in_chan: Arc<Mutex<broadcast::Sender<MessageIn>>>,
}

impl Default for UserBuffers {
    fn default() -> Self {
        let (in_send, _) = broadcast::channel(1024);
        Self {
            buffers: Default::default(),
            in_chan: Arc::new(Mutex::new(in_send)),
        }
    }
}

impl UserBuffers {
    pub fn register(&mut self, id: Uuid) -> Result<(), Error> {
        match self.buffers.entry(id) {
            Entry::Occupied(_) => return Err(Error::InternalError),
            Entry::Vacant(ve) => {
                let (send_out, _) = broadcast::channel(1024);
                ve.insert(Arc::new(Mutex::new(send_out)));
            }
        };

        Ok(())
    }

    pub fn out_recv_chan(&self, guid: Uuid) -> Result<broadcast::Receiver<MessageOut>, Error> {
        Ok(self
            .buffers
            .get(&guid)
            .ok_or(Error::UserNotFound)? // TODO: handle missing
            .lock()
            .expect("lock buffers")
            .subscribe())
    }

    pub fn out_send_chan(&self, guid: Uuid) -> Result<broadcast::Sender<MessageOut>, Error> {
        Ok(self
            .buffers
            .get(&guid)
            .ok_or(Error::UserNotFound)? // TODO: handle missing
            .lock()
            .expect("lock buffers")
            .clone())
    }

    pub fn in_recv_chan(&self) -> broadcast::Receiver<MessageIn> {
        self.in_chan.lock().expect("lock in_chan").subscribe()
    }

    pub fn in_send_chan(&self) -> broadcast::Sender<MessageIn> {
        self.in_chan.lock().expect("lock in_chan").clone()
    }
}
