use crate::game_loader::{self, GameInfo};
use std::sync::OnceLock;
use std::sync::{Arc, Mutex};
slint::include_modules!();
use slint::{Model, ModelRc, SharedString, ToSharedString};

static GAME_INFORMATIONS: OnceLock<Vec<GameInfo>> = OnceLock::new();

impl From<&GameInfo> for GameInformation {
    fn from(game: &GameInfo) -> Self {
        let args: Vec<SharedString> = game.arguments.iter().map(|x| x.as_str().into()).collect();

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

struct GuiSlint {
    gui: AppWindow,
    games: Arc<Mutex<Vec<GameInfo>>>,
}

impl Default for GuiSlint {
    fn default() -> Self {
        Self {
            gui: AppWindow::new().unwrap(),
            games: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

// todo: make all methods implementations of a struct for better state control
impl GuiSlint {
    pub fn run_slint_gui(&self) {
        self.gui.run().unwrap();
    }

    fn set_games(&mut self) {
        match game_loader::parse_games() {
            Ok(games) => {
                let slint_games = self.update_and_read(games);
                self.gui.set_game_infos(ModelRc::new(slint::VecModel::from(slint_games)));
            }
            Err(e) => {
                open_error_window(e);
            }
        }
    }

    pub fn update_and_read(&self, new_games: Vec<GameInfo>) -> Vec<GameInformation> {
        let mut games_ref = self.games.lock().unwrap();
        
        *games_ref = new_games; 
        
        let slint_games: Vec<GameInformation> = games_ref
            .iter()
            .map(GameInformation::from)
            .collect();

        slint_games
    }

    pub fn launch_game(&self, x: i32) {
        let games_ref = self.games.lock().unwrap();
        
        if let Some(game) = games_ref.get(x as usize) {
            println!("Launching game from struct state!");
            match game_loader::spawn_game(game) {
                Ok(_) => {},
                Err(e) => {},
            }
        } else {
            
        }
    }
}
fn test() -> SharedString {
    return "skibidi".to_shared_string();
}

fn launch_game(x: i32) -> LaunchStatus {
    match game_loader::spawn_game(&GAME_INFORMATIONS.get().unwrap()[x as usize]) {
        Ok(_) => LaunchStatus {
            success: true,
            error: "".into(),
        },
        Err(e) => LaunchStatus {
            success: false,
            error: e.into(),
        },
    }
}

fn open_error_window(e: String) {
    let error_window = ErrorWindow::new().unwrap();
    error_window.set_error_msg(e.into());
    let _ = error_window.show();
}

fn set_games(app: &AppWindow) {
    match game_loader::parse_games() {
        Ok(games) => {
            let slint_games: Vec<GameInformation> = games.iter().map(GameInformation::from).collect();
            app.set_game_infos(ModelRc::new(slint::VecModel::from(slint_games)));
        }
        Err(e) => {
            open_error_window(e);
        }
    }
}

pub fn run_slint_gui() {
    let gui = AppWindow::new().unwrap();

    let _ = gui.show();
    

    gui.on_launch_game(launch_game);

    {
        // test junk stuff yeah
        gui.on_test_function(test);

        gui.on_test_struct_function(|| LaunchStatus {
            success: false,
            error: SharedString::from("Totally real error message"),
        });
    }
    // gui.set_game_info(fetched_status);
    set_games(&gui);
    slint::run_event_loop().unwrap();
}
