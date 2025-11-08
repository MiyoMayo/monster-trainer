use std::cell::OnceCell;

pub struct Monster {
    name: OnceCell<String>,
}

impl Monster {
    pub fn new() -> Self {
        Self {
            name: OnceCell::new(),
        }
    }

    /// 名前を初期化
    /// 2回目以降はErr
    pub fn init_name(&mut self, name: impl Into<String>) -> anyhow::Result<()> {
        self.name
            .set(name.into())
            .map_err(|_| anyhow::anyhow!("名前は初期化済みです"))
    }

    /// 名前を取得
    pub fn get_name(&self) -> Option<&str> {
        self.name.get().map(|s| s.as_str())
    }
}
