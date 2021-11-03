use serde::Deserialize;
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

#[derive(Copy, Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
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
    #[serde(rename_all = "camelCase")]
    VoteTarget {
        id: Uuid,
        vote_kind: VoteKind,
    },
}

impl ActionResponse {
    pub fn id(&self) -> Uuid {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comms::incoming::ActionResponse;
    use test_case::test_case;

    macro_rules! parse {
        ($e:expr) => {
            $e.parse().unwrap()
        };
    }

    // HealTarget,
    // BlackmailTarget,
    // FinishTarget,
    // DeathMarkTarget,
    // DiabolizationTarget,
    // ShootTarget,
    // VoteProposal,
    // VoteTarget,

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
    fn deserializes_jsons_with_expected_format(s: &str) -> ActionResponse {
        serde_json::from_str(s).unwrap()
    }
}
