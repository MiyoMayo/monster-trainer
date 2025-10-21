use crate::game::{GameContext, GameMutContext};

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
    fn update(
        &mut self,
        mut_ctx: &mut GameMutContext,
        ctx: &GameContext,
    ) -> anyhow::Result<SceneTransition>;
}
