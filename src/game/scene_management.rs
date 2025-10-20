use crate::game::scene::{Scene, SceneKind, SceneTransition};
use crate::game::title::title_scene::TitleScene;
use crate::game::{GameContext, GameCore};

pub struct SceneController {
    current_scene: Box<dyn Scene>,
}

impl SceneController {
    pub fn new(kind: SceneKind) -> Self {
        Self {
            current_scene: Self::create_scene(kind),
        }
    }

    /// シーンを変更する
    pub fn change_scene(&mut self, kind: SceneKind) {
        self.current_scene = Self::create_scene(kind);
    }

    /// シーンを作成する
    fn create_scene(kind: SceneKind) -> Box<dyn Scene> {
        match kind {
            SceneKind::Title => Box::new(TitleScene::new()),
        }
    }

    /// シーンを更新する
    pub fn update(&mut self, ctx: &mut GameContext, core: &GameCore) -> anyhow::Result<SceneTransition> {
        self.current_scene.update(ctx, core)
    }
}
