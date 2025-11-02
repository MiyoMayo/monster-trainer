use crate::core::console::Console;
use crate::core::input::InputEvent;
use crate::core::time::TimeManager;
use crate::game::monster::Monster;
use crate::game::scene::{SceneKind, SceneTransition};
use crate::game::scene_management::SceneController;
use crossterm::event::KeyCode;

mod enums;
mod monster;
mod scene;
mod scene_management;
mod title;

pub struct GameContext {
    time_manager: TimeManager,
}

impl GameContext {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            time_manager: TimeManager::new(),
        })
    }

    pub fn update(&mut self) -> std::io::Result<()> {
        self.time_manager.update();

        Ok(())
    }
}

pub struct GameMutContext {
    input_event: InputEvent,
    monster: Monster,
    console: Console,
}

impl GameMutContext {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            input_event: InputEvent::new()?,
            monster: Monster::new(),
            console: Console::new(),
        })
    }

    fn update(&mut self) -> std::io::Result<()> {
        self.input_event.update()?;

        Ok(())
    }
}

pub struct GameSystem {
    scene_controller: SceneController,

    game_mut_context: GameMutContext,
    game_context: GameContext,

    continue_game: bool,
}

impl GameSystem {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            scene_controller: SceneController::new(SceneKind::Title),
            game_mut_context: GameMutContext::new()?,
            game_context: GameContext::new()?,
            continue_game: true,
        })
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        let s = self
            .game_mut_context
            .input_event
            .subscribe(KeyCode::Esc, move || self.continue_game = false);

        while self.continue_game {
            self.game_context.update()?;
            self.game_mut_context.update()?;

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

        Ok(())
    }
}
