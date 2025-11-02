use crate::core::rx::{Observable, Subject, Subscription};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::{event, execute, terminal};
use indexmap::IndexMap;
use std::time::Duration;

/// キー入力があった時にイベントを送信する
pub struct InputEvent {
    input_subject: IndexMap<KeyCode, Subject<()>>,
}

impl InputEvent {
    pub fn new() -> std::io::Result<Self> {
        let mut input = Self {
            input_subject: IndexMap::new(),
        };
        input.initialize()?;

        Ok(input)
    }

    /// 入力状態を更新する
    pub fn update(&mut self) -> std::io::Result<()> {
        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if let Some(subject) = self.input_subject.get_mut(&key.code) {
                    subject.emit(&());
                }
            }
        }

        Ok(())
    }

    /// キー入力があった時に呼び出されるクロージャを登録する
    pub fn subscribe(
        &mut self,
        key_code: KeyCode,
        mut f: impl FnMut() + 'static,
    ) -> Subscription<()> {
        self.input_subject
            .entry(key_code)
            .or_insert(Subject::new())
            .subscribe(move |_| f())
    }

    fn initialize(&mut self) -> std::io::Result<()> {
        let mut stdout = std::io::stdout();
        execute!(stdout, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        Ok(())
    }

    fn finalize() {
        let mut stdout = std::io::stdout();
        terminal::disable_raw_mode().unwrap();
        execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    }
}

impl Drop for InputEvent {
    fn drop(&mut self) {
        Self::finalize();
    }
}
