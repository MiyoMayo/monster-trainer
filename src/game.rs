use crate::game::scene::{SceneKind, SceneTransition};
use crate::game::scene_management::SceneController;

mod enums;
mod scene;
mod scene_management;
mod title;

pub struct GameSystem {
    scene_controller: SceneController,
}

impl GameSystem {
    pub fn new() -> Self {
        Self {
            scene_controller: SceneController::new(SceneKind::Title),
        }
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        loop {
            let t = self.scene_controller.update()?;

            match t {
                SceneTransition::Quit => return Ok(()),
                SceneTransition::ChangeScene(s) => {
                    self.scene_controller.change_scene(s);
                }
            }
        }
    }
}
