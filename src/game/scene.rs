pub enum SceneKind {
    Title,
}

pub enum SceneTransition {
    Quit,
    ChangeScene(SceneKind),
}

pub trait Scene {
    /// シーンの更新
    fn update(&mut self) -> anyhow::Result<SceneTransition>;
}
