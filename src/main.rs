use std::env; 
mod game_loader;
mod tui;
mod cli; 

fn main() {
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
