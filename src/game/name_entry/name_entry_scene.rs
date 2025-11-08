use crate::core::rx::Subscriptions;
use crate::game::scene::{Scene, SceneTransition};
use crate::game::{def, GameContext, GameMutContext};
use crossterm::event::KeyCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub struct NameEntryScene {
    name_buffer: Arc<Mutex<String>>,
    subs: Subscriptions,
    accept: Arc<AtomicBool>,
}

impl NameEntryScene {
    pub fn new() -> Self {
        Self {
            name_buffer: Arc::new(Mutex::new(String::new())),
            subs: Subscriptions::new(),
            accept: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Scene for NameEntryScene {
    fn start(&mut self, mut_ctx: &mut GameMutContext, _ctx: &GameContext) -> anyhow::Result<()> {
        let name = self.name_buffer.clone();
        let accept = self.accept.clone();

        self.subs.add(mut_ctx.input_event.subscribe_any(move |key| {
            let Ok(mut name) = name.lock() else {
                return;
            };

            match key {
                KeyCode::Char(c) if name.len() < def::NAME_MAX => name.push(*c),
                KeyCode::Backspace if !name.is_empty() => {
                    name.pop();
                }
                KeyCode::Enter if !name.is_empty() => accept.store(true, Ordering::Release),
                _ => (),
            };
        }));

        Ok(())
    }

    fn update(
        &mut self,
        mut_ctx: &mut GameMutContext,
        _ctx: &GameContext,
    ) -> anyhow::Result<SceneTransition> {
        if self.accept.load(Ordering::Acquire)
            && let Ok(name) = self.name_buffer.lock()
        {
            mut_ctx.monster.init_name(name.as_str())?;
            return Ok(SceneTransition::Quit);
        }

        mut_ctx.console.println(format!(
            "モンスターの名前を入力してください(半角{}文字以内)",
            def::NAME_MAX
        ));
        if let Ok(name) = self.name_buffer.lock() {
            mut_ctx.console.println(name.as_str());
        }
        mut_ctx.console.new_line();
        mut_ctx.console.println("エンターキーで確定");

        Ok(SceneTransition::Continue)
    }
}
