use crate::GAME_STATE;
use rustyline::{error::ReadlineError, Editor};

use crate::game::card::print_all_roles;

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
                    _ if line.starts_with("roles") => {}
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
