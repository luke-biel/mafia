use crate::game::action_request::ActionRequest;
use crate::game::card::{AngelOfDeath, Blank, Card, Diaboliser, Faction, Role, Syndicate, Value};
use crate::game::TimeOfDay;

pub const SYNDICATE_AOD: Card<Syndicate, AngelOfDeath> = Card::default();
pub const SYNDICATE_DIABOLISER: Card<Syndicate, Diaboliser> = Card::default();
pub const SYNDICATE_BLANK: Card<Syndicate, Blank> = Card::default();

impl Role for Card<Syndicate, AngelOfDeath> {
    fn request_user_action(&self, time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day != 0 {
            vec![ActionRequest::MarkForDeath]
        } else {
            Vec::new()
        }
    }

    fn faction(&self) -> Faction {
        Faction::Syndicate
    }

    fn value(&self) -> Value {
        Value::King
    }
}

impl Role for Card<Syndicate, Diaboliser> {
    fn request_user_action(&self, time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day == 0 {
            vec![ActionRequest::SelectDiabolised]
        } else {
            Vec::new()
        }
    }

    fn faction(&self) -> Faction {
        Faction::Syndicate
    }

    fn value(&self) -> Value {
        Value::Queen
    }
}

impl Role for Card<Syndicate, Blank> {
    fn request_user_action(&self, _time_of_day: TimeOfDay, _day: usize) -> Vec<ActionRequest> {
        Vec::new()
    }

    fn faction(&self) -> Faction {
        Faction::Syndicate
    }

    fn value(&self) -> Value {
        Value::V2
    }
}
