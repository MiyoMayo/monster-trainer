use crate::core::rx::Subscriptions;
use crate::game::scene::{Scene, SceneKind, SceneTransition};
use crate::game::{GameContext, GameMutContext};
use crossterm::event::KeyCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct TitleScene {
    need_next_scene: Arc<AtomicBool>,
    subs: Subscriptions,
}

impl TitleScene {
    pub fn new() -> Self {
        Self {
            need_next_scene: Arc::new(AtomicBool::new(false)),
            subs: Subscriptions::new(),
        }
    }
}

impl Scene for TitleScene {
    fn start(&mut self, mut_ctx: &mut GameMutContext, _ctx: &GameContext) -> anyhow::Result<()> {
        let c = self.need_next_scene.clone();
        self.subs.add(
            mut_ctx
                .input_event
                .subscribe(KeyCode::Char(' '), move || c.store(true, Ordering::Release)),
        );

        Ok(())
    }

    fn update(
        &mut self,
        mut_ctx: &mut GameMutContext,
        _ctx: &GameContext,
    ) -> anyhow::Result<SceneTransition> {
        if self.need_next_scene.load(Ordering::Acquire) {
            return Ok(SceneTransition::ChangeScene(SceneKind::NameEntry));
        }

        mut_ctx.console.println("スペースキーではじめる");
        mut_ctx.console.new_line();
        mut_ctx.console.println("エスケープキーでいつでも終了");

        Ok(SceneTransition::Continue)
    }
}
