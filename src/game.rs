use crate::core::input_manager::InputManager;
use crate::game::monster::Monster;
use crate::game::scene::{SceneKind, SceneTransition};
use crate::game::scene_management::SceneController;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

mod enums;
mod monster;
mod scene;
mod scene_management;
mod title;

pub struct GameCore {
    input_manager: InputManager,
}

impl GameCore {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            input_manager: InputManager::new()?,
        })
    }
}

pub struct GameContext {
    monster: Monster,
}

impl GameContext {
    pub fn new() -> Self {
        Self {
            monster: Monster::new(),
        }
    }
}

pub struct GameSystem {
    scene_controller: SceneController,

    game_context: GameContext,
    game_core: GameCore,
}

impl GameSystem {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            scene_controller: SceneController::new(SceneKind::Title),
            game_context: GameContext::new(),
            game_core: GameCore::new()?,
        })
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        loop {
            InputManager::clear_console()?;

            self.game_core.input_manager.update()?;

            let t = self
                .scene_controller
                .update(&mut self.game_context, &self.game_core)?;

            std::io::stdout().flush()?;

            match t {
                SceneTransition::Quit => return Ok(()),
                SceneTransition::Continue => continue,
                SceneTransition::ChangeScene(s) => {
                    self.scene_controller.change_scene(s);
                }
            }

            sleep(Duration::from_nanos(16_666_667));
        }
    }
}
