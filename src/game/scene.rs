use crate::game::GameContext;

pub enum SceneKind {
    Title,
}

pub enum SceneTransition {
    Quit,
    ChangeScene(SceneKind),
}

pub trait Scene {
    /// シーンの更新
    fn update(&mut self, ctx: &mut GameContext) -> anyhow::Result<SceneTransition>;
}
