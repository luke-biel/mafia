use crate::game::card::{Blackmailer, Blank, Card, Coquette, Doctor, Mafia, Role};
use crate::game::{ActionRequest, TimeOfDay};

pub const MAFIA_BLACKMAILER: Card<Mafia, Blackmailer> = Card::default();
pub const MAFIA_COQUETTE: Card<Mafia, Coquette> = Card::default();
pub const MAFIA_PAVULON: Card<Mafia, Doctor> = Card::default();
pub const MAFIA_BLANK: Card<Mafia, Blank> = Card::default();

impl Role for Card<Mafia, Blackmailer> {
    fn request_user_action(&self, time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day == 0 {
            vec![ActionRequest::SelectBlackmailed]
        } else {
            Vec::new()
        }
    }
}

impl Role for Card<Mafia, Coquette> {
    fn request_user_action(&self, _time_of_day: TimeOfDay, _day: usize) -> Vec<ActionRequest> {
        Vec::new()
    }
}

impl Role for Card<Mafia, Doctor> {
    fn request_user_action(&self, time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day != 0 {
            vec![ActionRequest::FinishPatient]
        } else {
            Vec::new()
        }
    }
}

impl Role for Card<Mafia, Blank> {
    fn request_user_action(&self, _time_of_day: TimeOfDay, _day: usize) -> Vec<ActionRequest> {
        Vec::new()
    }
}
