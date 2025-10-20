pub struct Time{
    delta_time: f64,
}

impl Time {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
        }
    }
}
