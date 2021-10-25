use crate::game::{ActionRequest, Lobby, TimeOfDay};
use std::marker::PhantomData;

pub mod city;
pub mod mafia;
pub mod syndicate;

pub trait Role {
    fn request_user_action(&self, time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest>;
}

pub struct Card<Suit, Value> {
    _phantom: PhantomData<(Suit, Value)>,
}

impl<Suit, Value> const Default for Card<Suit, Value> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

pub struct City;
pub struct Mafia;
pub struct Syndicate;

pub struct Blank;

pub struct GunShop;
pub struct Katani;
pub struct Escort;
pub struct Doctor;

pub struct Blackmailer;
pub struct Coquette;

pub struct AngelOfDeath;
pub struct Diaboliser;
