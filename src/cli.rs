use std::process;
use std::io;
use std::io::{Read, Write};

use crate::game_loader::GameInfo;
use crate::game_loader;


const INTRO_HEADER: &str = "----------------------------------
|        Steam Remote Play       |
----------------------------------";
fn display_games(games: &[GameInfo]) {
    println!("{INTRO_HEADER}");
    
    for (i, game) in games.iter().enumerate() {
        // Fallback to path if name is empty
        let display_name = if game.name.is_empty() {
            &game.path
        } else {
            &game.name
        };

        println!("  {i}. {display_name}");
    }
}

pub fn run_cli() {
    let games = match game_loader::parse_games() {
        Ok(value) => value,
        Err(error) => {
            println!("Error occured: {}", error);
            println!("Hit enter to exit program");
            let _ = io::stdin().read(&mut [0u8]);
            process::exit(0);
            
        },
    };

    display_games(&games);
    println!("Enter a game to play 0-{} (inclusive)\nEnter q to quit.", games.len()-1);
    let mut input = String::new();
    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim().to_lowercase() == "q" {
            println!("Exiting application.");
            process::exit(0);
        }
        else if input.trim().to_lowercase() == "c" {
            // clear screen, scroll and reset cursor to top left.
            print!("\x1b[2J\x1b[3J\x1b[H");
            io::stdout().flush().unwrap();
            display_games(&games);
            println!("Enter a game to play 0-{} (inclusive)\nEnter q to quit or c to clear & reprint the terminal", games.len()-1);
            continue;
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
                    process::exit(0);
                }
                Err(e) => {
                    println!("Error when opening application: {}", e);
                    println!("Hit enter to exit program");
                    let _ = io::stdin().read(&mut [0u8]);
                    break;
                }
            }
        }
    }
}
