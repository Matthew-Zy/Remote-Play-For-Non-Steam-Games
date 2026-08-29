use std::io;
use std::io::{Read};
use std::env; 
mod game_loader;
mod tui;
mod cli; 

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("{:#?}", args);

    let use_cli = args.contains(&String::from("--cli"));


    let games = match game_loader::parse_games() {
        Ok(value) => value,
        Err(error) => {
            println!("Error occured: {}", error);
            let _ = io::stdin().read(&mut [0u8]);
            panic!("Closing program")
        },
    };

    if use_cli {
        cli::run_cli(&games);
    } else {
        let _ = tui::run_tui(games);
    }
}

