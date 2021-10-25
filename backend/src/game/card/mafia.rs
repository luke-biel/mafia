use crate::game::card::{Blackmailer, Blank, Card, Coquette, Doctor, Mafia};

pub const MAFIA_BLACKMAILER: Card<Mafia, Blackmailer> = Card::default();
pub const MAFIA_COQUETTE: Card<Mafia, Coquette> = Card::default();
pub const MAFIA_PAVULON: Card<Mafia, Doctor> = Card::default();
pub const MAFIA_BLANK: Card<Mafia, Blank> = Card::default();
