use crate::core::input_manager::State;
use crate::game::scene::{Scene, SceneTransition};
use crate::game::{GameContext, GameMutContext};
use crossterm::event::KeyCode;

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
        let e = ctx.input_manager.get_key_state(KeyCode::Char('a'));
        mut_ctx.console.println(format!(
            "FPS: {} / ∆: {}",
            ctx.time_manager.fps(),
            ctx.time_manager.delta_time()
        ));
        mut_ctx.console.println(format!("タイトル: A {:?}", e));

        if matches!(
            ctx.input_manager.get_key_state(KeyCode::Esc),
            Some(State::Pressed)
        ) {
            return Ok(SceneTransition::Quit);
        }

        Ok(SceneTransition::Continue)
    }
}
