
use std::time::Instant;


use TimeSpan;

pub struct StopWatch {
    start: Instant,
    stop:  Instant,
}

impl StopWatch {
    pub fn new() -> StopWatch {
        StopWatch {
            start: Instant::now(),
            stop:  Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    pub fn stop(&mut self) {
        self.stop = Instant::now()
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
        self.stop  = self.start.clone();
    }

    pub fn end(&self) -> Instant {
        if self.stop < self.start {
            Instant::now()
        } else {
            self.stop.clone()
        }
    }

    pub fn ticks(&self) -> i64 {
        // ticks are measured in 100ns bundles
        let duration = self.end().duration_since(self.start);
        (duration.subsec_nanos() as i64 / 100_i64)
            + (duration.as_secs() as i64 * 10_000_000_i64)
    }

    pub fn elapsed(&self) -> TimeSpan {
        TimeSpan::new(self.ticks())
    }
}