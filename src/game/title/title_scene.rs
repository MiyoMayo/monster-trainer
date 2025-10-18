use crate::game::scene::{Scene, SceneTransition};
use crate::game::GameContext;

pub struct TitleScene {}

impl TitleScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for TitleScene {
    fn update(&mut self, ctx: &mut GameContext) -> anyhow::Result<SceneTransition> {
        println!("タイトル");

        Ok(SceneTransition::Quit)
    }
}
