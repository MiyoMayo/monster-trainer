use crossterm::event::{Event, KeyCode};
use crossterm::{event, execute, queue, terminal};
use indexmap::{IndexMap, IndexSet};
use std::time::Duration;

#[derive(Debug)]
pub enum State {
    None,
    Pressed,
    Held,
    Released,
}

pub struct InputManager {
    key_states: IndexMap<KeyCode, State>,
    input_key_buffer: IndexSet<KeyCode>,
}

impl InputManager {
    pub fn new() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(std::io::stdout(), terminal::EnterAlternateScreen)?;

        Ok(Self {
            key_states: IndexMap::new(),
            input_key_buffer: IndexSet::with_capacity(8),
        })
    }

    /// 入力状態を更新する
    pub fn update(&mut self) -> std::io::Result<()> {
        // このフレームの入力を全て取得
        self.input_key_buffer.clear();
        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                self.input_key_buffer.insert(key.code);
                self.key_states.entry(key.code).or_insert(State::None);
            }
        }

        // 入力状態を更新
        for (key, state) in self.key_states.iter_mut() {
            *state = if self.input_key_buffer.contains(key) {
                // 今回入力されている
                match state {
                    State::None | State::Released => State::Pressed,
                    State::Pressed | State::Held => State::Held,
                }
            } else {
                // 今回入力されていない
                match state {
                    State::None | State::Released => State::None,
                    State::Pressed | State::Held => State::Released,
                }
            };
        }

        Ok(())
    }

    /// キーの入力状態を取得
    pub fn get_key_state(&self, key: KeyCode) -> Option<&State> {
        self.key_states.get(&key)
    }

    /// ターミナルをクリアする
    pub fn clear_console() -> std::io::Result<()> {
        queue!(
            std::io::stdout(),
            terminal::Clear(terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )
    }
}

impl Drop for InputManager {
    fn drop(&mut self) {
        execute!(std::io::stdout(), terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
