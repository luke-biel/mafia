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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Details>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Context {
    #[serde(rename = "msg")]
    Action(ActionRequest),
    #[serde(rename = "msg")]
    Broadcast(Broadcast),
    #[serde(rename = "msg")]
    Notification(Notification),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Details {
    Id(Uuid),
    Role(Role),
    IsGood(bool),
    Faction(Faction),
}

#[derive(Debug, Clone, Serialize)]
pub enum Broadcast {
    GameStart,
    GameEnd,
}

#[derive(Debug, Clone, Serialize)]
pub enum Notification {
    Killed,
    RaisedFromGrave,
    Blackmailed,
    CardCheck,
    FactionCheck,
}

impl MessageOut {
    pub fn new(requires_response: bool, msg: Context, details: Option<Details>) -> Self {
        Self {
            requires_response,
            msg,
            details,
        }
    }
}

impl Broadcast {
    pub fn game_start() -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Broadcast(Broadcast::GameStart),
            details: None,
        }
    }

    pub fn game_end(faction: Faction) -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Broadcast(Broadcast::GameEnd),
            details: Some(Details::Faction(faction)),
        }
    }
}

impl Notification {
    pub fn killed() -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Notification(Notification::Killed),
            details: None,
        }
    }

    pub fn raised_from_grave() -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Notification(Notification::RaisedFromGrave),
            details: None,
        }
    }

    pub fn blackmailed(by_who: Uuid) -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Notification(Notification::Blackmailed),
            details: Some(Details::Id(by_who)),
        }
    }

    pub fn card_check(role: Role) -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Notification(Notification::CardCheck),
            details: Some(Details::Role(role)),
        }
    }

    pub fn faction_check(faction: Faction) -> MessageOut {
        MessageOut {
            requires_response: false,
            msg: Context::Notification(Notification::FactionCheck),
            details: Some(Details::Faction(faction)),
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
    #[test_case(Notification::card_check(Role::MafiaBlackmailer) => r#"{"requiresResponse":false,"msg":"CardCheck","details":{"role":"MafiaBlackmailer"}}"#)]
    #[test_case(Notification::faction_check(Faction::Mafia)      => r#"{"requiresResponse":false,"msg":"FactionCheck","details":{"faction":"Mafia"}}"#)]
    fn serializes_message_out_to_expected_json(message: MessageOut) -> String {
        serde_json::to_string(&message).unwrap()
    }
}
