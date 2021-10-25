#![feature(const_trait_impl)]

use comms::UserBuffers;
use game::Game;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::RwLock;

pub mod comms;
pub mod game;
pub mod reject;
pub mod routes;

lazy_static! {
    pub static ref PLAYER_COMMS: Arc<RwLock<UserBuffers>> =
        Arc::new(RwLock::new(UserBuffers::default()));
    pub static ref GAME_STATE: Arc<RwLock<Game>> = Arc::new(RwLock::new(Game::default()));
}
