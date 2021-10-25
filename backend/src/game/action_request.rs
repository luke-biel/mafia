use crate::comms::MessageOut;
use serde::Serialize;

#[derive(Copy, Clone, Debug, Serialize)]
pub enum ActionRequest {
    CheckGoodBad,
    CheckCard,
    Heal,
    SelectBlackmailed,
    FinishPatient,
    MarkForDeath,
    SelectDiabolised,
    Shoot,
    ProposeVote,
    CastVote,
}

impl ActionRequest {
    pub fn into_message(self) -> MessageOut {
        MessageOut::Action(self)
    }

    pub fn blocking(&self) -> bool {
        !matches!(self, Self::ProposeVote | Self::SelectBlackmailed)
    }
}
