use crate::game::name_entry::name_entry_scene::NameEntryScene;
use crate::game::scene::{Scene, SceneKind, SceneTransition};
use crate::game::title::title_scene::TitleScene;
use crate::game::{GameContext, GameMutContext};

pub struct SceneController {
    current_scene: Box<dyn Scene>,
    completed_start: bool,
}

impl SceneController {
    pub fn new(kind: SceneKind) -> Self {
        Self {
            current_scene: Self::create_scene(kind),
            completed_start: false,
        }
    }

    /// シーンを変更する
    pub fn change_scene(&mut self, kind: SceneKind) {
        self.current_scene = Self::create_scene(kind);
        self.completed_start = false;
    }

    /// シーンを作成する
    fn create_scene(kind: SceneKind) -> Box<dyn Scene> {
        match kind {
            SceneKind::Title => Box::new(TitleScene::new()),
            SceneKind::NameEntry => Box::new(NameEntryScene::new()),
        }
    }

    /// シーンを更新する
    pub fn update(
        &mut self,
        mut_ctx: &mut GameMutContext,
        ctx: &GameContext,
    ) -> anyhow::Result<SceneTransition> {
        if !self.completed_start {
            self.current_scene.start(mut_ctx, ctx)?;
            self.completed_start = true;
        }

        self.current_scene.update(mut_ctx, ctx)
    }
}
