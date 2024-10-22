use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Timer {
    start: Option<Instant>,
    end: Option<Duration>,
}

impl Timer {
    pub fn is_started(&self) -> bool {
        self.start.is_some()
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn end(&mut self) {
        self.end = Some(
            self.start
                .expect("start to have been called before end")
                .elapsed(),
        );
    }

    pub fn duration(&self) -> Duration {
        self.end.expect("end to have been called before duration")
    }
}
