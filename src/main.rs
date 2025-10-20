use crate::game::GameSystem;

mod core;
mod game;

fn main() {
    if let Err(e) = GameSystem::new().and_then(|s| s.run()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
