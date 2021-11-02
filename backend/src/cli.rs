use crate::comms::UserBuffers;
use crate::{GAME_STATE, PLAYER_COMMS};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rustyline::{error::ReadlineError, Editor};
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::game::card::{print_all_roles, Role};
use crate::game::lobby::{Function, GameModifiers, RoleModifiers, TimeOfDay};
use crate::game::{start_game, Game};

pub async fn handle_admin() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("./.mafia_cli_history").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match handle_user_command(&line) {
                    Ok(text) => println!("{}", text),
                    Err(text) => eprintln!("error!\n{}", text),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("./.mafia_cli_history").unwrap();
}

pub fn handle_user_command(line: &str) -> Result<String, &'static str> {
    match line {
        "list" => {
            let game = GAME_STATE.read().unwrap();
            Ok(format!(
                "Players ({}): \n{}",
                game.players.len(),
                serde_json::to_string_pretty(&game.players).unwrap()
            ))
        }
        "h roles" => Ok(print_all_roles()),
        "h" => Ok("Available commands: h,list,debug".to_string()),
        "debug" => {
            let game = GAME_STATE.read().unwrap();
            Ok(format!("{:#?}", &*game))
        }
        "reset" => {
            let mut gs = GAME_STATE.write().unwrap();
            *gs = Game::default();
            let mut pc = PLAYER_COMMS.write().unwrap();
            *pc = UserBuffers::default();

            Ok("done".to_string())
        }
        _ if line.starts_with("roles") => {
            let (_, role_list) = line.split_once(' ').unwrap();
            let roles = role_list
                .split(',')
                .map(|i| i.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let mut gd = GAME_STATE.write().unwrap();
            if roles.len() != gd.players.len() {
                return Err("number of players and roles do not match");
            }

            let mut players: Vec<_> = gd.players.keys().collect();
            players.shuffle(&mut thread_rng());
            let mut living = HashMap::with_capacity(players.len());
            let all_roles: Vec<Role> = Role::iter().collect();

            for (role, player) in roles.iter().zip(players.iter()) {
                living.insert(
                    **player,
                    Function {
                        card: all_roles[*role],
                        modifiers: RoleModifiers {
                            diabolised: false,
                            blackmailed_by: None,
                            marked_by_aod: false,
                            blackmails: None,
                        },
                        alive: true,
                    },
                );
            }

            gd.lobby.roles = living;
            gd.lobby.time_of_day = TimeOfDay::Night;
            gd.lobby.day = 0;
            gd.lobby.modifiers = GameModifiers {
                is_gun_shop_dead_during_day: false,
            };

            tokio::spawn(start_game());
            Ok("game started".to_string())
        }
        _ => Err("unknown command"),
    }
}
