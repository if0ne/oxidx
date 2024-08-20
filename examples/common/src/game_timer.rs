use std::time::Instant;

#[derive(Copy, Clone, Debug)]
pub struct GameTimer {
    timer: Instant,

    stopped: bool,

    base_time: f64,
    paused_time: f64,
    stop_time: f64,

    delta_time: f64,
}

impl Default for GameTimer {
    fn default() -> Self {
        Self {
            timer: Instant::now(),
            stopped: Default::default(),
            base_time: Default::default(),
            paused_time: Default::default(),
            stop_time: Default::default(),
            delta_time: -1.0,
        }
    }
}

impl GameTimer {
    pub fn game_time(&self) -> f32 {
        0.0
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time as f32
    }

    pub fn reset(&mut self) {}

    pub fn start(&mut self) {}

    pub fn stop(&mut self) {}

    pub fn tick(&mut self) {
        if self.stopped {
            self.delta_time = 0.0;
            return;
        }

        self.delta_time = self.timer.elapsed().as_secs_f64() * 1000.0;
        self.timer = Instant::now();

        if self.delta_time < 0.0 {
            self.delta_time = 0.0;
        }
    }
}
