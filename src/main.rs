use std::env; 
mod game_loader;
mod tui;
mod cli; 

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    // println!("{:#?}", args);

    let use_cli = args.contains(&String::from("--cli"));

    if use_cli {
        cli::run_cli();
    } else {
        let _ = tui::run_tui();
    }
}

