use crate::TimeSpan;
use std::time::Instant;

pub struct StopWatch {
    start: Instant,
    stop: Instant,
}

impl Default for StopWatch {
    fn default() -> Self {
        StopWatch {
            start: Instant::now(),
            stop: Instant::now(),
        }
    }
}

impl StopWatch {
    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    pub fn stop(&mut self) {
        self.stop = Instant::now()
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
        self.stop = self.start;
    }

    pub fn end(&self) -> Instant {
        if self.stop < self.start {
            Instant::now()
        } else {
            self.stop
        }
    }

    pub fn ticks(&self) -> i64 {
        // ticks are measured in 100ns bundles
        let duration = self.end().duration_since(self.start);
        i64::from(duration.subsec_nanos()) / 100_i64 + (duration.as_secs() as i64 * 10_000_000_i64)
    }

    pub fn elapsed(&self) -> TimeSpan {
        TimeSpan::new(self.ticks())
    }
}
