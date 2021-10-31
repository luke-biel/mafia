use crate::GAME_STATE;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rustyline::{error::ReadlineError, Editor};
use std::collections::HashMap;

use crate::game::card::{print_all_roles, ALL_ROLES};
use crate::game::lobby::{Function, GameModifiers, RoleModifiers, TimeOfDay};
use crate::game::start_game;

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
                match line.as_str() {
                    "list" => {
                        let game = GAME_STATE.read().unwrap();
                        println!(
                            "Players ({}): \n{}",
                            game.players.len(),
                            serde_json::to_string_pretty(&game.players).unwrap()
                        )
                    }
                    "h roles" => {
                        println!("Available roles:");

                        print_all_roles()
                    }
                    "h" => {
                        println!("Available commands: h,list,debug");
                    }
                    "debug" => {
                        let game = GAME_STATE.read().unwrap();
                        println!("{:#?}", &*game)
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
                            eprintln!("number of players and roles do not match");
                            continue;
                        }

                        let mut players: Vec<_> = gd.players.keys().collect();
                        players.shuffle(&mut thread_rng());
                        let mut living = HashMap::with_capacity(players.len());
                        for (role, player) in roles.iter().zip(players.iter()) {
                            living.insert(
                                **player,
                                Function {
                                    card: ALL_ROLES[*role],
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
                    }
                    _ => println!("Unknown command"),
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
