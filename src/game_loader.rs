use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

static CUSTOM_CONF_LOCATION: OnceLock<String> = OnceLock::new();
const GAMES_CONF_TXT: &str = "games.txt";
const GAMES_CONF_TOML: &str = "games.toml";

#[derive(Debug, Default, Deserialize)]
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
        Ok(_) => Ok(()),
        Err(e) => {
            let error_msg = format!(
                "Error running application: {}\n{}",
                game.path,
                e.to_string()
            );
            Err(error_msg)
        }
    }
}

pub fn set_custom_conf_path(path: String) {
    CUSTOM_CONF_LOCATION.set(path).unwrap();
}

pub fn parse_games() -> Result<Vec<GameInfo>, String> {
    if CUSTOM_CONF_LOCATION.get().is_some() {
        let custom_conf = CUSTOM_CONF_LOCATION.get().unwrap();
        match fs::read_to_string(custom_conf) {
            Ok(_) => {
                if custom_conf.ends_with(".toml") {
                    match parse_games_toml(custom_conf) {
                        Ok(games) => return Ok(games),
                        Err(e) => return Err(e),
                    }

                } else {
                    return Ok(parse_games_txt(custom_conf));
                }
            }
            Err(e) => {
                return Err(format!(
                    "Could not find custom conf file: {custom_conf}\n{e}"
                ));
            }
        }
    }

    match fs::read_to_string(GAMES_CONF_TOML) {
        Ok(_) => match parse_games_toml(GAMES_CONF_TOML) {
            Ok(games) => return Ok(games),
            Err(e) => return Err(e),
        },

        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            if Path::new(GAMES_CONF_TOML).exists() {
                return Ok(parse_games_txt(GAMES_CONF_TXT));
            } else {
                Err(format!(
                    "No file {GAMES_CONF_TXT} or {GAMES_CONF_TOML} file was found.\nPlease Ensure a {GAMES_CONF_TXT} or {GAMES_CONF_TOML} exists and are placed in the same directory as this executable."
                ))
            }
        }

        Err(e) => Err(format!("Error reading {GAMES_CONF_TOML}: {e}")),
    }
}


fn parse_games_toml(path: &str) -> Result<Vec<GameInfo>, String> {
    let toml_str = fs::read_to_string(path).unwrap();

    let mut map: HashMap<String, Vec<GameInfo>> = toml::from_str(&toml_str)
        .map_err(|e| e.to_string())?;

    let games = map.remove("game")
        .ok_or_else(|| "Missing 'game' key in TOML file".to_string())?;

    Ok(games)
}

fn parse_games_txt(path: &str) -> Vec<GameInfo> {
    let content = fs::read_to_string(path).expect("Should have been able to read the file");
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
