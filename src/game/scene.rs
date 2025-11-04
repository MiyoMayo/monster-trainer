use crate::game::{GameContext, GameMutContext};

#[derive(Debug, Copy, Clone)]
pub enum SceneKind {
    Title,
}

pub enum SceneTransition {
    Quit,
    Continue,
    ChangeScene(SceneKind),
}

pub trait Scene {
    /// シーンの初期化
    fn start(&mut self, _mut_ctx: &mut GameMutContext, _ctx: &GameContext) -> anyhow::Result<()> {
        Ok(())
    }

    /// シーンの更新
    fn update(
        &mut self,
        _mut_ctx: &mut GameMutContext,
        _ctx: &GameContext,
    ) -> anyhow::Result<SceneTransition> {
        Ok(SceneTransition::Continue)
    }
}
