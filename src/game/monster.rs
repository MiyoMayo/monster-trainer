pub struct Monster {
    name: String,
}

impl Monster {
    pub fn new() -> Self {
        Self {
            name: String::from("Minty"),
        }
    }
}
