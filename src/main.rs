use std::env; 
mod game_loader;
mod tui;
mod cli; 

slint::include_modules!();
use slint::{SharedString, ToSharedString};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let use_cli = args.contains(&String::from("--cli"));
    let use_gui = args.contains(&String::from("--gui"));
    for x in args {
        if let Some(custom_conf) = x.strip_prefix("-conf=") {
            game_loader::set_custom_conf_path(custom_conf.to_string());
        }
    }
    if use_cli {
        cli::run_cli();
    } else if use_gui {
        run_slint_gui();
    }
    else {
        let _ = tui::run_tui();
    }
}

fn test() -> SharedString {
    return "skibidi".to_shared_string();
}

fn run_slint_gui() {
    
    let gui = AppWindow::new().unwrap();

    gui.on_test_function(test);


    gui.run().unwrap();
}