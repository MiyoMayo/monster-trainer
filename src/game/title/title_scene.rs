use crate::game::scene::{Scene, SceneTransition};
use crate::game::{GameContext, GameMutContext};

pub struct TitleScene {}

impl TitleScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for TitleScene {
    fn update(
        &mut self,
        mut_ctx: &mut GameMutContext,
        ctx: &GameContext,
    ) -> anyhow::Result<SceneTransition> {
        mut_ctx.console.println(format!(
            "FPS: {} / ∆: {}",
            ctx.time_manager.fps(),
            ctx.time_manager.delta_time()
        ));

        Ok(SceneTransition::Continue)
    }
}
