use crate::core::input_manager::InputManager;
use crate::core::time::TimeManager;
use crate::game::monster::Monster;
use crate::game::scene::{SceneKind, SceneTransition};
use crate::game::scene_management::SceneController;
use std::io::Write;

mod enums;
mod monster;
mod scene;
mod scene_management;
mod title;

pub struct GameCore {
    input_manager: InputManager,
    time_manager: TimeManager,
}

impl GameCore {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            input_manager: InputManager::new()?,
            time_manager: TimeManager::new(),
        })
    }

    pub fn update(&mut self) -> std::io::Result<()> {
        self.time_manager.update();
        self.input_manager.update()?;

        Ok(())
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
            self.game_core.update()?;

            let transition = self
                .scene_controller
                .update(&mut self.game_context, &self.game_core)?;

            std::io::stdout().flush()?;

            match transition {
                SceneTransition::Quit => return Ok(()),
                SceneTransition::Continue => (),
                SceneTransition::ChangeScene(s) => {
                    self.scene_controller.change_scene(s);
                }
            }

            self.game_core.time_manager.frame_sleep();
        }
    }
}
