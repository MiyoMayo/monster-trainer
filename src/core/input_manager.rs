use crossterm::{execute, terminal};
use indexmap::{IndexMap, IndexSet};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

#[derive(Debug)]
pub enum State {
    None,
    Pressed,
    Held,
    Released,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum KeyCode {
    Char(char),
    Esc,
    Space,
    Up,
    Down,
    Right,
    Left,
}

pub struct InputManager {
    key_states: IndexMap<KeyCode, State>,
    input_key_buffer: Arc<Mutex<IndexSet<KeyCode>>>,
}

impl InputManager {
    pub fn new() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(std::io::stdout(), terminal::EnterAlternateScreen)?;

        let input = Self {
            key_states: IndexMap::new(),
            input_key_buffer: Default::default(),
        };
        let input_key_buffer = input.input_key_buffer.clone();
        spawn(move || Self::begin_update(&input_key_buffer));

        Ok(input)
    }

    /// 入力状態を更新する
    fn begin_update(input_key_buffer: &Mutex<IndexSet<KeyCode>>) {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let mut buffer = [0u8; 16];

        loop {
            match handle.read(&mut buffer) {
                Ok(0) => break,
                Ok(_) => {
                    let byte = buffer[0];
                    // エスケープシーケンスの場合
                    if byte == 0x1B {
                        let s = String::from_utf8_lossy(&buffer).to_string();
                        let key_code = match s.as_str() {
                            "\x1b" => KeyCode::Esc,
                            "\x1b[A" => KeyCode::Up,
                            "\x1b[B" => KeyCode::Down,
                            "\x1b[C" => KeyCode::Right,
                            "\x1b[D" => KeyCode::Left,
                            _ => continue,
                        };

                        input_key_buffer.lock().unwrap().insert(key_code);
                    } else if byte == 0x20 {
                        input_key_buffer.lock().unwrap().insert(KeyCode::Space);
                    }
                    // 通常のキー
                    else {
                        input_key_buffer
                            .lock()
                            .unwrap()
                            .insert(KeyCode::Char(byte as char));
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => panic!("{}", e),
            };
        }
    }

    pub fn update(&mut self) {
        let mut input = self.input_key_buffer.lock().unwrap();

        // 新規のキーコードがあれば登録する
        for key_code in input.iter() {
            self.key_states
                .entry(key_code.clone())
                .or_insert(State::None);
        }

        // 入力状態を更新
        for (key, state) in self.key_states.iter_mut() {
            *state = if input.contains(key) {
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
            }
        }

        input.clear();
    }

    /// キーの入力状態を取得
    pub fn get_key_state(&self, key: KeyCode) -> Option<&State> {
        self.key_states.get(&key)
    }
}

impl Drop for InputManager {
    fn drop(&mut self) {
        execute!(std::io::stdout(), terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
