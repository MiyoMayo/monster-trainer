use crossterm::style::Print;
use crossterm::{execute, terminal};
use std::fmt::Display;
use std::io::{Stdout, Write};

pub struct Console {
    print_buffer: String,
    stdout: Stdout,
}

impl Console {
    pub fn new() -> Self {
        Self {
            print_buffer: String::new(),
            stdout: std::io::stdout(),
        }
    }

    /// コンソールをクリアする
    fn clear(&mut self) -> std::io::Result<()> {
        execute!(
            self.stdout,
            terminal::Clear(terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )
    }

    /// 1行出力する
    /// 本当はバッファに追加しているだけ
    pub fn println<T: Display>(&mut self, s: T) {
        if !self.print_buffer.is_empty() {
            self.print_buffer.push_str("\n\r");
        }

        self.print_buffer.push_str(format!("{}", s).as_str());
    }

    /// バッファの内容を出力する
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.clear()?;
        execute!(self.stdout, Print(self.print_buffer.as_str()))?;
        self.stdout.flush()?;
        self.print_buffer.clear();

        Ok(())
    }
}
