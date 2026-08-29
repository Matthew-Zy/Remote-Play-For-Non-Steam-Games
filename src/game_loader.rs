use std::process::Command;
use std::fs;
use std::collections::HashMap;
use std::path::Path;
use serde::Deserialize;

const GAMES_CONF_TXT: &str = "games.txt";
const GAMES_CONF_TOML: &str = "games.toml";

#[derive(Debug)]
#[derive(Default)]
#[derive(Deserialize)]
pub struct GameInfo {
    pub path: String,

    #[serde(default)]
    pub arguments: Vec<String>,

    #[serde(default)]
    pub env_variables: Vec<(String, String)>,

    #[serde(default)]
    pub name: String,

}

pub fn spawn_game(game: &GameInfo) -> Result<(), String> {

    let game_file = Path::new(&game.path);
    let working_dir = game_file.parent().unwrap_or_else(|| Path::new("."));
    let match_result = Command::new(game_file)
        .args(&game.arguments)
        .envs(game.env_variables.clone())
        .current_dir(working_dir)
        .spawn();
    match match_result {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {

            Err(e.to_string())
        }
    }
}


pub fn parse_games() -> Result<Vec<GameInfo>, String> {
    match fs::read_to_string(GAMES_CONF_TXT) {
        Ok(_) => return Ok(parse_games_txt()), // File exists and is readable
        
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            if Path::new(GAMES_CONF_TOML).exists() {
                return Ok(parse_games_toml())
            } else {
                Err(format!("No file {GAMES_CONF_TXT} or {GAMES_CONF_TOML} was found"))
            }
        }
        
        Err(e) => Err(format!("Error reading {GAMES_CONF_TXT}: {e}")),
    }
}


pub fn parse_games_toml() -> Vec<GameInfo> {
    let toml_str = fs::read_to_string(GAMES_CONF_TOML).unwrap();

    let mut map: HashMap<String, Vec<GameInfo>> = toml::from_str(&toml_str).unwrap();
    
    // Extract the vector directly from the map
    let games: Vec<GameInfo> = map.remove("game").unwrap_or_default();

    println!("{:#?}", games);

    return games;
}

fn parse_games_txt() -> Vec<GameInfo> {
    let content = fs::read_to_string(GAMES_CONF_TXT)
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