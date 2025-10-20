use crate::game::{GameContext, GameCore};

pub enum SceneKind {
    Title,
}

pub enum SceneTransition {
    Quit,
    Continue,
    ChangeScene(SceneKind),
}

pub trait Scene {
    /// シーンの更新
    fn update(&mut self, ctx: &mut GameContext, core: &GameCore) -> anyhow::Result<SceneTransition>;
}
