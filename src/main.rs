use std::process;
use std::io;
use std::io::{Write, Read};
use std::env; 
mod game_loader;
mod tui;

use game_loader::GameInfo;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("{:#?}", args);

    let use_cli = args.contains(&String::from("-cli"));


    let games = match game_loader::parse_games() {
        Ok(value) => value,
        Err(error) => {
            println!("Error occured: {}", error);
            let _ = io::stdin().read(&mut [0u8]);
            panic!("Closing program")
        },
    };

    if use_cli {
        run_cli(&games);
    } else {
        let _ = tui::run_tui(games);
    }
}



const INTRO_HEADER: &str = "
-------------------------
|   Steam Remote Play   |
-------------------------";
fn display_games(games: &[GameInfo]) {
    println!("{INTRO_HEADER}");
    
    for (i, game) in games.iter().enumerate() {
        // Fallback to path if name is empty
        let display_name = if game.name.is_empty() {
            &game.path
        } else {
            &game.name
        };

        if game.arguments.is_empty() {
            println!("{i}. {display_name}");
        } else {
            println!("{i}. {display_name}");
            // println!("{i}. {display_name} (arguments: {})", game.arguments.join(" "));
        }
    }
}

fn run_cli(games: &[GameInfo]) {
    display_games(games);
    let mut input = String::new();
    println!("Enter a game to play 0-{} (inclusive)\nenter q or Q to quit.", games.len()-1);
    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim().to_uppercase() == "Q" {
            println!("Exiting application...");
            process::exit(0);
        }
        let num_input: usize = match input.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
                eprintln!("Error: Please enter a valid number!");
                continue
            }
        };

        if num_input >= games.len() {
            println!("Please enter a number between 0 and {}", games.len()-1)
        } else {
            println!("Game Launch Params:\n{:#?}", &games[num_input]);
            match game_loader::spawn_game(&games[num_input]) {
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    println!("Error when opening application: {}", e);
                    println!("Hit enter to continue");
                    let _ = io::stdin().read(&mut [0u8]);
                }
            }
        }
    }
}