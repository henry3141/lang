pub struct Timer {
    start: std::time::Instant,
}

impl Timer {
    pub fn start() -> Timer {
        Timer {
            start: std::time::Instant::now(),
        }
    }

    pub fn stop(&self) -> std::time::Duration {
        self.start.elapsed()
    }
}
