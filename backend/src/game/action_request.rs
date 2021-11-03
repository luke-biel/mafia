use crate::comms::incoming::ActionResponse;
use crate::comms::outgoing::{Context, MessageOut};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "msg")]
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
        }
    }

    pub fn is_sufficient(&self, response: &ActionResponse) -> bool {
        matches!(
            (self, response),
            (
                ActionRequest::CheckGoodBad,
                ActionResponse::CheckGoodBadTarget { .. }
            ) | (
                ActionRequest::CheckCard,
                ActionResponse::CheckCardTarget { .. }
            ) | (ActionRequest::Heal, ActionResponse::HealTarget { .. })
                | (
                    ActionRequest::SelectBlackmailed,
                    ActionResponse::BlackmailTarget { .. }
                )
                | (
                    ActionRequest::FinishPatient,
                    ActionResponse::FinishTarget { .. }
                )
                | (
                    ActionRequest::MarkForDeath,
                    ActionResponse::DeathMarkTarget { .. }
                )
                | (
                    ActionRequest::SelectDiabolized,
                    ActionResponse::DiabolizationTarget { .. }
                )
                | (ActionRequest::Shoot, ActionResponse::ShootTarget { .. })
                | (
                    ActionRequest::ProposeVote,
                    ActionResponse::VoteProposal { .. }
                )
                | (ActionRequest::CastVote, ActionResponse::VoteTarget { .. })
        )
    }
}
