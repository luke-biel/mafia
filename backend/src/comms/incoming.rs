use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MessageIn {
    pub meta: Meta,
    pub body: MessageInBody,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Meta {
    pub guid: Uuid,
    pub response_kind: ResponseKind,
}

#[derive(Copy, Debug, Clone, Deserialize)]
pub enum MessageInBody {
    #[serde(rename = "id")]
    Id(Uuid),
}

impl MessageInBody {
    pub fn id(&self) -> Uuid {
        match self {
            MessageInBody::Id(id) => *id,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
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
