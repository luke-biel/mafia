use crate::game::action_request::ActionRequest;
use crate::game::card::{Blank, Card, City, Doctor, Escort, Faction, GunShop, Katani, Role, Value};
use crate::game::TimeOfDay;

pub const CITY_GUN_SHOP: Card<City, GunShop> = Card::default();
pub const CITY_KATANI: Card<City, Katani> = Card::default();
pub const CITY_ESCORT: Card<City, Escort> = Card::default();
pub const CITY_DOCTOR: Card<City, Doctor> = Card::default();
pub const CITY_BLANK: Card<City, Blank> = Card::default();

impl Role for Card<City, GunShop> {
    fn request_user_action(&self, _time_of_day: TimeOfDay, _day: usize) -> Vec<ActionRequest> {
        Vec::new()
    }

    fn faction(&self) -> Faction {
        Faction::City
    }

    fn value(&self) -> Value {
        Value::Ace
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

    fn faction(&self) -> Faction {
        Faction::City
    }

    fn value(&self) -> Value {
        Value::King
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

    fn faction(&self) -> Faction {
        Faction::City
    }

    fn value(&self) -> Value {
        Value::Queen
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

    fn faction(&self) -> Faction {
        Faction::City
    }

    fn value(&self) -> Value {
        Value::Jack
    }
}

impl Role for Card<City, Blank> {
    fn request_user_action(&self, _time_of_day: TimeOfDay, _day: usize) -> Vec<ActionRequest> {
        Vec::new()
    }

    fn faction(&self) -> Faction {
        Faction::City
    }

    fn value(&self) -> Value {
        Value::V2
    }
}
