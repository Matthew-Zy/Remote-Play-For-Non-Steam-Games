use crate::game_loader::{self, GameInfo};
use std::sync::OnceLock;

slint::include_modules!();
use slint::{Model, ModelRc, SharedString, ToSharedString};


static GAME_INFORMATIONS: OnceLock<Vec<GameInfo>> = OnceLock::new();

impl From<&GameInfo> for GameInformation {
    fn from(game: &GameInfo) -> Self {
        let args: Vec<SharedString> = game.arguments
            .iter()
            .map(|x| x.as_str().into())
            .collect();

        GameInformation {
            display_name: if !game.name.is_empty() { 
                game.name.as_str().into() 
            } else { 
                game.path.as_str().into() 
            },
            path: game.path.as_str().into(),
            args: ModelRc::new(slint::VecModel::from(args)),
        }
    }
}

fn test() -> SharedString {
    return "skibidi".to_shared_string();
}



fn fetch_games() -> GameInformationStatus {
    if GAME_INFORMATIONS.get().is_none() {
        match game_loader::parse_games() {
            Ok(games) => {
                GAME_INFORMATIONS.set(games).unwrap();
            },
            Err(e) => {
                return GameInformationStatus {
                    success: false,
                    game_infos: ModelRc::default(),
                    error: e.into()
                }
            }
        }
    }

    let games: &Vec<GameInfo> = GAME_INFORMATIONS.get().unwrap();

    let slint_games: Vec<GameInformation> = games.iter().map(GameInformation::from).collect();

    GameInformationStatus {
        success: true,
        game_infos: ModelRc::new(ModelRc::new(slint::VecModel::from(slint_games))),
        error: "cmon MANNN".into(),
    }

}


pub fn run_slint_gui() {
    
    let gui = AppWindow::new().unwrap();

    gui.on_test_function(test);

    gui.on_fetch_games(fetch_games);

    gui.on_test_struct_function(|| {
        LaunchStatus {
            success: false,
            error: SharedString::from("Totally real error message"),
        }
    });


    gui.run().unwrap();
}