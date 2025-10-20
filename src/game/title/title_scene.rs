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
        let s = format!("タイトル : A {:?}", e);
        crossterm::queue!(std::io::stdout(), crossterm::style::Print(s))?;

        if matches!(
            core.input_manager.get_key_state(KeyCode::Esc),
            Some(State::Pressed)
        ) {
            return Ok(SceneTransition::Quit);
        }

        Ok(SceneTransition::Continue)
    }
}
