use crate::game::action_request::ActionRequest;
use crate::game::card::{Faction, Role};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageOut {
    pub requires_response: bool,
    #[serde(flatten)]
    pub msg: Context,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Context {
    Action(ActionRequest),
    Broadcast(Broadcast),
    Notification(Notification),
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "msg", content = "details")]
pub enum Broadcast {
    GameStart,
    GameEnd { faction: Faction },
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "msg", content = "details")]
pub enum Notification {
    Killed,
    RaisedFromGrave,
    Blackmailed { id: Uuid },
    CardCheck { card: Role },
    FactionCheck { good: bool },
}

impl Broadcast {
    pub fn game_start() -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Broadcast(Broadcast::GameStart),
        }
    }

    pub fn game_end(faction: Faction) -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Broadcast(Broadcast::GameEnd { faction }),
        }
    }
}

impl Notification {
    pub fn killed() -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Notification(Notification::Killed),
        }
    }

    pub fn raised_from_grave() -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Notification(Notification::RaisedFromGrave),
        }
    }

    pub fn blackmailed(id: Uuid) -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Notification(Notification::Blackmailed { id }),
        }
    }

    pub fn card_check(card: Role) -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Notification(Notification::CardCheck { card }),
        }
    }

    pub fn faction_check(good: bool) -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Notification(Notification::FactionCheck { good }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Broadcast, Notification};
    use crate::comms::outgoing::MessageOut;
    use crate::game::action_request::ActionRequest;
    use crate::game::card::{Faction, Role};
    use test_case::test_case;
    use uuid::Uuid;

    #[test_case(ActionRequest::Shoot.into_message_out()          => r#"{"requiresResponse":true,"msg":"Shoot"}"#)]
    #[test_case(Broadcast::game_start()                          => r#"{"requiresResponse":false,"msg":"GameStart"}"#)]
    #[test_case(Broadcast::game_end(Faction::Mafia)              => r#"{"requiresResponse":false,"msg":"GameEnd","details":{"faction":"Mafia"}}"#)]
    #[test_case(Notification::killed()                           => r#"{"requiresResponse":false,"msg":"Killed"}"#)]
    #[test_case(Notification::raised_from_grave()                => r#"{"requiresResponse":false,"msg":"RaisedFromGrave"}"#)]
    #[test_case(Notification::blackmailed(Uuid::nil())           => r#"{"requiresResponse":false,"msg":"Blackmailed","details":{"id":"00000000-0000-0000-0000-000000000000"}}"#)]
    #[test_case(Notification::card_check(Role::MafiaBlackmailer) => r#"{"requiresResponse":false,"msg":"CardCheck","details":{"card":"MafiaBlackmailer"}}"#)]
    #[test_case(Notification::faction_check(true)                => r#"{"requiresResponse":false,"msg":"FactionCheck","details":{"good":true}}"#)]
    fn serializes_message_out_to_expected_json(message: MessageOut) -> String {
        serde_json::to_string(&message).unwrap()
    }
}
