use crate::game::GameSystem;

mod core;
mod game;

fn main() {
    if let Err(e) = GameSystem::new().run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
