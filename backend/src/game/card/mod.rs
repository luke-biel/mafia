use std::fmt;
use std::marker::PhantomData;

use crate::game::action_request::ActionRequest;
use crate::game::card::city::{CITY_BLANK, CITY_DOCTOR, CITY_ESCORT, CITY_GUN_SHOP, CITY_KATANI};
use crate::game::card::mafia::{MAFIA_BLACKMAILER, MAFIA_BLANK, MAFIA_COQUETTE, MAFIA_PAVULON};
use crate::game::card::syndicate::{SYNDICATE_AOD, SYNDICATE_BLANK, SYNDICATE_DIABOLISER};
use crate::game::TimeOfDay;

pub mod city;
pub mod mafia;
pub mod syndicate;

macro_rules! impl_print_static {
    ($typ:ty) => {
        impl PrintStatic for $typ {
            fn fmt(f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", stringify!($typ))
            }
        }
    };
}

pub trait Role: fmt::Debug {
    fn request_user_action(&self, time_of_day: TimeOfDay, day: usize) -> Vec<ActionRequest>;
    fn faction(&self) -> Faction;
    fn value(&self) -> Value;
}

pub trait PrintStatic {
    fn fmt(f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

pub struct City;
impl_print_static!(City);
pub struct Mafia;
impl_print_static!(Mafia);
pub struct Syndicate;
impl_print_static!(Syndicate);

pub struct Blank;
impl_print_static!(Blank);

pub struct GunShop;
impl_print_static!(GunShop);
pub struct Katani;
impl_print_static!(Katani);
pub struct Escort;
impl_print_static!(Escort);
pub struct Doctor;
impl_print_static!(Doctor);

pub struct Blackmailer;
impl_print_static!(Blackmailer);
pub struct Coquette;
impl_print_static!(Coquette);

pub struct AngelOfDeath;
impl_print_static!(AngelOfDeath);
pub struct Diabolizer;
impl_print_static!(Diabolizer);

#[derive(Clone)]
pub struct Card<Suit: PrintStatic, Value: PrintStatic> {
    _phantom: PhantomData<(Suit, Value)>,
}

pub const ALL_ROLES: &[&(dyn Role + Send + Sync)] = &[
    &CITY_GUN_SHOP,
    &CITY_KATANI,
    &CITY_ESCORT,
    &CITY_DOCTOR,
    &CITY_BLANK,
    &MAFIA_BLACKMAILER,
    &MAFIA_COQUETTE,
    &MAFIA_PAVULON,
    &MAFIA_BLANK,
    &SYNDICATE_AOD,
    &SYNDICATE_DIABOLISER,
    &SYNDICATE_BLANK,
];

impl<Suit: PrintStatic, Value: PrintStatic> fmt::Debug for Card<Suit, Value> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Suit::fmt(f)?;
        Value::fmt(f)
    }
}

impl<Suit: PrintStatic + Sized, Value: PrintStatic + Sized> const Default for Card<Suit, Value> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

pub fn print_all_roles() {
    for (i, role) in ALL_ROLES.iter().enumerate() {
        println!("{}) {:?}", i, role)
    }
}
