use crate::game::action_request::ActionRequest;
use crate::game::lobby::TimeOfDay;
use itertools::Itertools;
use serde::Serialize;
use strum::IntoEnumIterator;

#[derive(Copy, Clone, strum::EnumIter, Debug, Serialize)]
pub enum Role {
    CityKatani,
    CityEscort,
    CityDoctor,
    CityBlank,
    MafiaBlackmailer,
    MafiaBlank,
    SyndicateAod,
    SyndicateDiaboliser,
    SyndicateBlank,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum Faction {
    City,
    Mafia,
    Syndicate,
    Mason,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    Joker,
    Ace,
    King,
    Queen,
    Jack,
    V10,
    V9,
    V8,
    V7,
    V6,
    V5,
    V4,
    V3,
    V2,
}

impl Role {
    pub fn request_user_action(&self, time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        match self {
            Role::CityKatani => Self::city_katani_action(time_of_day, day),
            Role::CityEscort => Self::city_escort_action(time_of_day, day),
            Role::CityDoctor => Self::city_doctor_action(time_of_day, day),
            Role::CityBlank => Self::city_blank_action(),
            Role::MafiaBlackmailer => Self::mafia_blackmailer_action(time_of_day, day),
            Role::MafiaBlank => Self::mafia_blank_action(),
            Role::SyndicateAod => Self::syndicate_aod_action(time_of_day, day),
            Role::SyndicateDiaboliser => Self::syndicate_diaboliser_action(time_of_day, day),
            Role::SyndicateBlank => Self::syndicate_blank_action(),
        }
    }

    pub fn faction(&self) -> Faction {
        match self {
            Role::CityKatani => Faction::City,
            Role::CityEscort => Faction::City,
            Role::CityDoctor => Faction::City,
            Role::CityBlank => Faction::City,
            Role::MafiaBlackmailer => Faction::Mafia,
            Role::MafiaBlank => Faction::Mafia,
            Role::SyndicateAod => Faction::Syndicate,
            Role::SyndicateDiaboliser => Faction::Syndicate,
            Role::SyndicateBlank => Faction::Syndicate,
        }
    }

    pub fn value(&self) -> Value {
        match self {
            Role::CityKatani => Value::King,
            Role::CityEscort => Value::Queen,
            Role::CityDoctor => Value::Jack,
            Role::CityBlank => Value::V2,
            Role::MafiaBlackmailer => Value::King,
            Role::MafiaBlank => Value::V2,
            Role::SyndicateAod => Value::King,
            Role::SyndicateDiaboliser => Value::Queen,
            Role::SyndicateBlank => Value::V2,
        }
    }

    fn city_katani_action(time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day != 0 {
            vec![ActionRequest::CheckGoodBad]
        } else {
            Vec::new()
        }
    }

    fn city_escort_action(time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day == 0 {
            vec![ActionRequest::CheckCard]
        } else {
            Vec::new()
        }
    }

    fn city_doctor_action(time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day != 0 {
            vec![ActionRequest::Heal]
        } else {
            Vec::new()
        }
    }

    fn city_blank_action() -> Vec<ActionRequest> {
        Vec::new()
    }

    fn mafia_blackmailer_action(time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day == 0 {
            vec![ActionRequest::SelectBlackmailed]
        } else {
            Vec::new()
        }
    }

    fn mafia_blank_action() -> Vec<ActionRequest> {
        Vec::new()
    }

    fn syndicate_aod_action(time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day != 0 {
            vec![ActionRequest::MarkForDeath]
        } else {
            Vec::new()
        }
    }

    fn syndicate_diaboliser_action(time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest> {
        if time_of_day == TimeOfDay::Night && day == 0 {
            vec![ActionRequest::SelectDiabolized]
        } else {
            Vec::new()
        }
    }

    fn syndicate_blank_action() -> Vec<ActionRequest> {
        Vec::new()
    }
}

pub fn print_all_roles() -> String {
    Role::iter()
        .enumerate()
        .map(|(i, role)| format!("{}) {:?}", i, role))
        .join("\n")
}
