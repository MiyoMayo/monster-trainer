use crate::core::console::Console;
use crate::core::input_manager::InputManager;
use crate::core::time::TimeManager;
use crate::game::monster::Monster;
use crate::game::scene::{SceneKind, SceneTransition};
use crate::game::scene_management::SceneController;

mod enums;
mod monster;
mod scene;
mod scene_management;
mod title;

pub struct GameContext {
    input_manager: InputManager,
    time_manager: TimeManager,
}

impl GameContext {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            input_manager: InputManager::new()?,
            time_manager: TimeManager::new(),
        })
    }

    pub fn update(&mut self) {
        self.time_manager.update();
        self.input_manager.update();
    }
}

pub struct GameMutContext {
    monster: Monster,
    console: Console,
}

impl GameMutContext {
    pub fn new() -> Self {
        Self {
            monster: Monster::new(),
            console: Console::new(),
        }
    }
}

pub struct GameSystem {
    scene_controller: SceneController,

    game_mut_context: GameMutContext,
    game_context: GameContext,
}

impl GameSystem {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            scene_controller: SceneController::new(SceneKind::Title),
            game_mut_context: GameMutContext::new(),
            game_context: GameContext::new()?,
        })
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        loop {
            self.game_context.update();

            let transition = self
                .scene_controller
                .update(&mut self.game_mut_context, &self.game_context)?;

            self.game_mut_context.console.flush()?;

            self.game_context.time_manager.frame_sleep();

            match transition {
                SceneTransition::Quit => return Ok(()),
                SceneTransition::Continue => (),
                SceneTransition::ChangeScene(s) => {
                    self.scene_controller.change_scene(s);
                }
            }
        }
    }
}
