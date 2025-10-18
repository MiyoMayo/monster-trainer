use crate::game::monster::Monster;
use crate::game::scene::{SceneKind, SceneTransition};
use crate::game::scene_management::SceneController;

mod enums;
mod monster;
mod scene;
mod scene_management;
mod title;

#[derive(Default)]
pub struct GameContext {
    monster: Monster,
}

pub struct GameSystem {
    scene_controller: SceneController,
    game_context: GameContext,
}

impl GameSystem {
    pub fn new() -> Self {
        Self {
            scene_controller: SceneController::new(SceneKind::Title),
            game_context: GameContext::default(),
        }
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        loop {
            let t = self.scene_controller.update(&mut self.game_context)?;

            match t {
                SceneTransition::Quit => return Ok(()),
                SceneTransition::ChangeScene(s) => {
                    self.scene_controller.change_scene(s);
                }
            }
        }
    }
}
