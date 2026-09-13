use crate::game_loader::{self, GameInfo};
use std::sync::{Arc, Mutex};
slint::include_modules!();
use slint::{Model, ModelRc, SharedString, ToSharedString};

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
    error_window: ErrorWindow,
    games: Arc<Mutex<Vec<GameInfo>>>,
}

impl Default for GuiSlint {
    fn default() -> Self {
        Self {
            gui: AppWindow::new().unwrap(),
            error_window: ErrorWindow::new().unwrap(),
            games: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

// todo: make all methods implementations of a struct for better state control
impl GuiSlint {
    pub fn new() -> Self {
        let gui = AppWindow::new().unwrap();
        // this ensures that the main app window appears before any error windows.
        gui.show().unwrap();


        
        
        Self {
            gui: gui, 
            error_window: ErrorWindow::new().unwrap(),
            games: Arc::new(Mutex::new(Vec::new())),
        }
    }


    pub fn run(&self) {
        slint::run_event_loop().unwrap();
    }


    fn set_games(&mut self) {
        match game_loader::parse_games() {
            Ok(games) => {
                let slint_games = self.update_and_read(games);
                self.gui.set_game_infos(ModelRc::new(slint::VecModel::from(slint_games)));
            }
            Err(e) => {
                self.open_error_window(e);
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
                Err(e) => {
                    self.open_error_window(e)
                },
            }
        } else {
            
        }
    }

    fn open_error_window(&self, e: String) {
        // todo: unfuck this
        open_error_window(e);
    }
}

fn open_error_window(e: String) {
    let error_window = ErrorWindow::new().unwrap();
    error_window.set_error_msg(e.into());
    let _ = error_window.show();
}


pub fn run_slint_gui() {
    let mut app = GuiSlint::new();
    app.set_games();

    let weak_gui = app.gui.as_weak();

    app.gui.on_launch_game(move |x: i32| {
        let binding = app.games.clone();
        let games = binding.lock().unwrap();


    });

    {
        app.gui.on_test_function(|| {"Test".into()});
    }



    app.run();
}

