use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct MessageIn {
    pub meta: Meta,
    pub msg: ActionResponse,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub guid: Uuid,
}

#[derive(Copy, Debug, Clone, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VoteKind {
    Check,
    Kill,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
#[serde(tag = "kind", content = "details")]
pub enum ActionResponse {
    CheckGoodBadTarget {
        id: Uuid,
    },
    CheckCardTarget {
        id: Uuid,
    },
    HealTarget {
        id: Uuid,
    },
    BlackmailTarget {
        id: Uuid,
    },
    FinishTarget {
        id: Uuid,
    },
    DeathMarkTarget {
        id: Uuid,
    },
    DiabolizationTarget {
        id: Uuid,
    },
    ShootTarget {
        id: Uuid,
    },
    #[serde(rename_all = "camelCase")]
    VoteProposal {
        id: Uuid,
        vote_kind: VoteKind,
    },
    VoteSkip,
    #[serde(rename_all = "camelCase")]
    VoteTarget {
        id: Uuid,
        vote_kind: VoteKind,
    },
}

impl ActionResponse {
    pub fn target_id(&self) -> Uuid {
        match self {
            ActionResponse::CheckGoodBadTarget { id }
            | ActionResponse::CheckCardTarget { id }
            | ActionResponse::HealTarget { id }
            | ActionResponse::BlackmailTarget { id }
            | ActionResponse::FinishTarget { id }
            | ActionResponse::DeathMarkTarget { id }
            | ActionResponse::DiabolizationTarget { id }
            | ActionResponse::ShootTarget { id }
            | ActionResponse::VoteProposal { id, .. }
            | ActionResponse::VoteTarget { id, .. } => *id,
            ActionResponse::VoteSkip => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comms::incoming::{ActionResponse, VoteKind};
    use test_case::test_case;

    macro_rules! parse {
        ($e:expr) => {
            $e.parse().unwrap()
        };
    }

    #[test_case(
        r#"{"kind":"CheckGoodBadTarget","details":{"id":"2d3c34ae-36e6-410d-823c-2884c2127134"}}"# =>
        ActionResponse::CheckGoodBadTarget { id: parse!("2d3c34ae-36e6-410d-823c-2884c2127134") }
    )]
    #[test_case(
        r#"{"kind":"CheckCardTarget","details":{"id":"6a13ccc1-c1f9-495e-a9fe-93592ac7cacf"}}"# =>
        ActionResponse::CheckCardTarget { id: parse!("6a13ccc1-c1f9-495e-a9fe-93592ac7cacf") }
    )]
    #[test_case(
        r#"{"kind":"HealTarget","details":{"id":"27224456-1f10-4728-a71e-df068fc10e78"}}"# =>
        ActionResponse::HealTarget { id: parse!("27224456-1f10-4728-a71e-df068fc10e78") }
    )]
    #[test_case(
        r#"{"kind":"VoteTarget","details":{"id":"27224456-1f10-4728-a71e-df068fc10e78","voteKind":"kill"}}"# =>
        ActionResponse::VoteTarget { id: parse!("27224456-1f10-4728-a71e-df068fc10e78"), vote_kind: VoteKind::Kill }
    )]
    #[test_case(
        r#"{"kind":"VoteProposal","details":{"id":"27224456-1f10-4728-a71e-df068fc10e78","voteKind":"check"}}"# =>
        ActionResponse::VoteProposal { id: parse!("27224456-1f10-4728-a71e-df068fc10e78"), vote_kind: VoteKind::Check }
    )]
    #[test_case(
        r#"{"kind":"VoteSkip"}"# =>
        ActionResponse::VoteSkip
    )]
    fn deserializes_jsons_with_expected_format(s: &str) -> ActionResponse {
        serde_json::from_str(s).unwrap()
    }
}
