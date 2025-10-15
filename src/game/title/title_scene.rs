use crate::game::scene::{Scene, SceneTransition};

pub struct TitleScene {}

impl TitleScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for TitleScene {
    fn update(&mut self) -> anyhow::Result<SceneTransition> {
        println!("タイトル");

        Ok(SceneTransition::Quit)
    }
}
