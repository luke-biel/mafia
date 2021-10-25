use crate::game::card::{Blank, Card, City, Doctor, Escort, GunShop, Katani, Role};
use crate::game::{ActionRequest, Lobby, TimeOfDay};

pub const CITY_GUN_SHOP: Card<City, GunShop> = Card::default();
pub const CITY_KATANI: Card<City, Katani> = Card::default();
pub const CITY_ESCORT: Card<City, Escort> = Card::default();
pub const CITY_DOCTOR: Card<City, Doctor> = Card::default();
pub const CITY_BLANK: Card<City, Blank> = Card::default();

impl Role for Card<City, GunShop> {
    fn request_user_action(&self, _time_of_day: TimeOfDay, _day: usize) -> Vec<ActionRequest> {
        Vec::new()
    }
}

impl Role for Card<City, Katani> {
    fn request_user_action(&self, time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day != 0 {
            vec![ActionRequest::CheckGoodBad]
        } else {
            Vec::new()
        }
    }
}

impl Role for Card<City, Escort> {
    fn request_user_action(&self, time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day == 0 {
            vec![ActionRequest::CheckCard]
        } else {
            Vec::new()
        }
    }
}

impl Role for Card<City, Doctor> {
    fn request_user_action(&self, time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day != 0 {
            vec![ActionRequest::Heal]
        } else {
            Vec::new()
        }
    }
}
