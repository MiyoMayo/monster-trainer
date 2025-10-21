use crate::core::input_manager::State;
use crate::game::scene::{Scene, SceneTransition};
use crate::game::{GameContext, GameCore};
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
        ctx: &mut GameContext,
        core: &GameCore,
    ) -> anyhow::Result<SceneTransition> {
        let e = core.input_manager.get_key_state(KeyCode::Char('a'));
        let s = format!(
            "FPS: {} / ∆: {}\nタイトル : A {:?}",
            core.time_manager.fps(),
            core.time_manager.delta_time(),
            e
        );
        println!("{}", s);

        if matches!(
            core.input_manager.get_key_state(KeyCode::Esc),
            Some(State::Pressed)
        ) {
            return Ok(SceneTransition::Quit);
        }

        Ok(SceneTransition::Continue)
    }
}
