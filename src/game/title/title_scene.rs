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
        let s = format!(
            "FPS: {} / ∆: {}\nタイトル : A {:?}",
            ctx.time_manager.fps(),
            ctx.time_manager.delta_time(),
            e
        );
        mut_ctx.console.println(&s);

        if matches!(
            ctx.input_manager.get_key_state(KeyCode::Esc),
            Some(State::Pressed)
        ) {
            return Ok(SceneTransition::Quit);
        }

        Ok(SceneTransition::Continue)
    }
}
