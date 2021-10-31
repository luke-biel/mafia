use crate::comms::incoming::ResponseKind;
use crate::comms::outgoing::{Context, MessageOut};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ActionRequest {
    CheckGoodBad,
    CheckCard,
    Heal,
    SelectBlackmailed,
    FinishPatient,
    MarkForDeath,
    SelectDiabolized,
    Shoot,
    ProposeVote,
    CastVote,
}

impl ActionRequest {
    pub fn into_message_out(self) -> MessageOut {
        MessageOut {
            requires_response: true,
            msg: Context::Action(self),
            details: None,
        }
    }

    pub fn expected_response(&self) -> ResponseKind {
        match self {
            ActionRequest::CheckGoodBad => ResponseKind::CheckGoodBadTarget,
            ActionRequest::CheckCard => ResponseKind::CheckCardTarget,
            ActionRequest::Heal => ResponseKind::HealTarget,
            ActionRequest::SelectBlackmailed => ResponseKind::BlackmailTarget,
            ActionRequest::FinishPatient => ResponseKind::FinishTarget,
            ActionRequest::MarkForDeath => ResponseKind::DeathMarkTarget,
            ActionRequest::SelectDiabolized => ResponseKind::DiabolizationTarget,
            ActionRequest::Shoot => ResponseKind::ShootTarget,
            ActionRequest::ProposeVote => ResponseKind::VoteProposal,
            ActionRequest::CastVote => ResponseKind::VoteTarget,
        }
    }
}
