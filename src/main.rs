use std::process;
use std::io;
use std::io::{Write, Read};
mod game_loader;

const INTRO_HEADER: &str = "
-------------------------
|   Steam Remote Play   |
-------------------------";

fn main() {

    let games = game_loader::parse_games();
    // println!("{:#?}", x);
    println!("{INTRO_HEADER}");
    game_loader::display_games(&games);
    let mut input = String::new();
    println!("Enter a game to play [0-{}]\nenter q or Q to quit.", games.len()-1);
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
            let success: bool = game_loader::spawn_game(&games[num_input]);
            if !success {
                // don't exit immediately so the user can still see the error msg..
                println!("Hit enter to continue");
                let _ = io::stdin().read(&mut [0u8]);
            }
            break;
        }
    }
    
}
