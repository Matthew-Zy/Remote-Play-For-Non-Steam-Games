use std::process::Command;
use std::fs;

const GAMES_CONF: &str = "games.txt";
#[derive(Debug)]
#[derive(Default)]
pub struct GameInfo {
    path: String,
    arguments: Vec<String>,
    name: String,

}

pub fn spawn_game(game: &GameInfo) -> bool {
    println!("{:#?}", game);
    let match_result = Command::new(&game.path)
        .args(&game.arguments)
        .spawn();
    
    match match_result {
        Ok(_) => {
            println!("Successfully started application {}", game.path);
            true
        }
        Err(e) => {
            eprintln!("Error happened when starting application: {}", e);
            eprintln!("Failed to start application: {}", game.path);
            false
        }
    }
}

pub fn parse_games() -> Vec<GameInfo> {
    let content = fs::read_to_string(GAMES_CONF)
        .expect("Should have been able to read the file");
    let games_list: Vec<&str> = content.split("\n").collect();


    let mut vec: Vec<GameInfo> = Vec::with_capacity(games_list.len()); 
    for game in &games_list {
        if game.trim().is_empty() || game.trim().starts_with("#") {
            continue;
        }
        let mut info: GameInfo = Default::default();
        let contents: Vec<&str> = game.split("|").collect();
        if contents.len() == 0 {
            continue; 
        }
        if contents.len() >= 1 {
            let mut args: Vec<String> = Vec::new();
            let game_launch_info: Vec<String> = split_args(contents[0]);
            for i in 0..game_launch_info.len() {
                if i == 0 {
                    info.path = game_launch_info[0].to_owned();
                } else {
                    args.push(game_launch_info[i].to_owned());
                }
            }
            info.arguments = args.to_owned();
        }
        if contents.len() >= 2 {
            info.name = contents[1].trim().to_owned();
        }
        vec.push(info);
    }
    return vec;
}

pub fn display_games(games: &[GameInfo]) {
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


fn split_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '"' => in_quotes = !in_quotes, // Toggle quote mode
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    args.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    return args;
}