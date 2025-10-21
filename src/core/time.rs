use std::thread;
use std::time::{Duration, Instant};

/// 目標FPS
pub const FPS: u32 = 60;
/// 1フレームあたりの秒数
pub const FRAME_SECS: f64 = 1.0 / FPS as f64;

pub struct TimeManager {
    latest_delta_time: f64,
    latest_fps: u32,

    previous_time: Instant,
    fps_start_time: Instant,
    fps_count: u32,
}

impl TimeManager {
    pub fn new() -> Self {
        Self {
            latest_delta_time: 0.0,
            latest_fps: 0,
            previous_time: Instant::now(),
            fps_start_time: Instant::now(),
            fps_count: 0,
        }
    }

    /// 時間情報を更新する
    pub fn update(&mut self) {
        self.update_delta_time();
        self.update_fps();

        self.previous_time = Instant::now();
    }

    /// FPSを調整するためにスレッドを止める
    pub fn frame_sleep(&self) {
        let sleep_secs = FRAME_SECS - self.latest_delta_time;
        if sleep_secs.is_sign_positive() {
            thread::sleep(Duration::from_secs_f64(sleep_secs));
        }
    }

    fn update_delta_time(&mut self) {
        let duration = Instant::now() - self.previous_time;
        self.latest_delta_time = duration.as_secs_f64();
    }

    fn update_fps(&mut self) {
        self.fps_count += 1;

        let now = Instant::now();
        let duration = now - self.fps_start_time;

        if duration.as_secs() >= 1 {
            self.latest_fps = self.fps_count;
            self.fps_count = 0;
            self.fps_start_time = now;
        }
    }

    pub fn delta_time(&self) -> f64 {
        self.latest_delta_time
    }

    pub fn fps(&self) -> u32 {
        self.latest_fps
    }
}
