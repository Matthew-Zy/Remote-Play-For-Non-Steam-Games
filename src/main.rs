use std::env; 
mod game_loader;
mod tui;
mod cli; 

#[cfg(target_os = "linux")]
use std::io::{self, IsTerminal};
#[cfg(target_os = "linux")]
use std::process::{self, Command};

fn main() {

    #[cfg(target_os = "linux")]
    {
        check_for_relaunch(); 
    }

    let args: Vec<String> = env::args().skip(1).collect();
    let use_cli = args.contains(&String::from("--cli"));
    for x in args {
        if let Some(custom_conf) = x.strip_prefix("-conf=") {
            game_loader::set_custom_conf_path(custom_conf.to_string());
        }
    }
    if use_cli {
        cli::run_cli();
    } else {
        let _ = tui::run_tui();
    }
}

#[cfg(target_os = "linux")]
fn check_for_relaunch() {
    if let Ok(appimage_path) = env::var("APPIMAGE") {
        if let Some(parent_dir) = std::path::Path::new(&appimage_path).parent() {
            let _ = env::set_current_dir(parent_dir);
        }
    }
    
    if !io::stdout().is_terminal() {
        relaunch_in_terminal();
        process::exit(0); 
    }
}

#[cfg(target_os = "linux")]
fn relaunch_in_terminal() {
    let exe_path = env::var("APPIMAGE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| env::current_exe().expect("Failed to get executable path"));

    let target_dir = exe_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));

    let args: Vec<String> = env::args().skip(1).collect();

    if let Ok(user_term) = env::var("TERMINAL") {
        if Command::new(&user_term)
            .current_dir(target_dir) // Force the terminal to open here
            .arg("-e")
            .arg(&exe_path)
            .args(&args)
            .env_remove("LD_LIBRARY_PATH")
            .env_remove("APPDIR")
            .env_remove("APPIMAGE")
            .spawn()
            .is_ok()
        {
            return;
        }
    }

    let default_terminals = [
        "xdg-terminal-exec",   
        "x-terminal-emulator", 
        "gnome-terminal",      
        "konsole",             
        "xfce4-terminal",      
        "wezterm",             
        "alacritty",
        "kitty",
        "xterm",               
    ];

    for term in default_terminals {
        if Command::new(term)
            .current_dir(target_dir) 
            .arg("-e")
            .arg(&exe_path)
            .args(&args)
            .env_remove("LD_LIBRARY_PATH")
            .env_remove("APPDIR")
            .env_remove("APPIMAGE")
            .spawn()
            .is_ok()
        {
            return;
        }
    }
    
    eprintln!("Could not find a terminal emulator to launch.");
}