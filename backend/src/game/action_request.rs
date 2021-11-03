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
        use ActionRequest::*;
        use ActionResponse::*;

        matches!(
            (self, response),
            (CheckGoodBad, CheckGoodBadTarget { .. })
                | (CheckCard, CheckCardTarget { .. })
                | (Heal, HealTarget { .. })
                | (SelectBlackmailed, BlackmailTarget { .. })
                | (FinishPatient, FinishTarget { .. })
                | (MarkForDeath, DeathMarkTarget { .. })
                | (SelectDiabolized, DiabolizationTarget { .. })
                | (Shoot, ShootTarget { .. })
                | (ProposeVote, VoteProposal { .. })
                | (ProposeVote, VoteSkip)
                | (CastVote, VoteTarget { .. })
        )
    }
}
