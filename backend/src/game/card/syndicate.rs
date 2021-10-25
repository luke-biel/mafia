use crate::game::card::{AngelOfDeath, Blank, Card, Diaboliser, Role, Syndicate};
use crate::game::{ActionRequest, TimeOfDay};

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
}

impl Role for Card<Syndicate, Diaboliser> {
    fn request_user_action(&self, time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day == 0 {
            vec![ActionRequest::SelectDiabolised]
        } else {
            Vec::new()
        }
    }
}

impl Role for Card<Syndicate, Blank> {
    fn request_user_action(&self, _time_of_day: TimeOfDay, _day: usize) -> Vec<ActionRequest> {
        Vec::new()
    }
}
