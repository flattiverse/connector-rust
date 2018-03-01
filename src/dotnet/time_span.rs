
use std::cmp::Ordering;

use Error;
use net::BinaryReader;

use atomic;
use atomic::Atomic;

#[derive(Debug)]
pub struct TimeSpan {
    ticks: Atomic<i64>
}

impl TimeSpan {
    pub fn new(ticks: i64) -> TimeSpan {
        TimeSpan {
            ticks: Atomic::new(ticks)
        }
    }

    pub fn from_dhmsm(days: i32, hours: i32, minutes: i32, seconds: i32, millis: i32) -> TimeSpan {
        Self::new(
            days            as i64 * 864_000_000_000_i64 // * ticks per day
                + hours     as i64 *  36_000_000_000_i64 // * ticks per hour
                + minutes   as i64 *     600_000_000_i64 // * ticks per minute
                + seconds   as i64 *      10_000_000_i64 // * ticks per seconds
                + millis    as i64 *          10_000_i64 // * ticks per milliseconds
        )
    }

    pub fn from_reader(reader: &mut BinaryReader) -> Result<TimeSpan, Error> {
        let time_span = Self::default();
        time_span.update(reader)?;
        Ok(time_span)
    }

    pub fn from_millis(millis: i64) -> TimeSpan {
        // 1ms * 1000 -> us * 10 -> 100ns
        Self::new(millis * 100_000_i64)
    }

    pub fn from_seconds(seconds: i64) -> TimeSpan {
        // 1s * 1000 -> ms * 1000 -> us * 10 -> 100ns
        Self::new(seconds * 10_000_000_i64)
    }

    pub fn ticks(&self) -> i64 {
        self.ticks.load(atomic::Ordering::Relaxed)
    }

    pub fn seconds(&self) -> i64 {
        // 100ns / 10 -> 1us / 1000 -> 1ms / 1000 -> 1s
        self.ticks() / 10_000_000_i64
    }

    pub fn seconds_exact(&self) -> f64 {
        // 100ns / 10 -> 1us / 1000 -> 1ms / 1000 -> 1s
        self.ticks() as f64 / 10_000_000_f64
    }

    pub fn millis(&self) -> u64 {
        (self.seconds_exact() * 1_000f64) as u64
    }

    pub(crate) fn update(&self, reader: &mut BinaryReader) -> Result<(), Error> {
        self.ticks.store(reader.read_u32()? as i64, atomic::Ordering::Relaxed);
        Ok(())
    }
}

impl PartialEq<TimeSpan> for TimeSpan {
    fn eq(&self, other: &TimeSpan) -> bool {
        let ticks_me = self.ticks();
        let ticks_ot = other.ticks();
        ticks_me == ticks_ot
    }
}

impl PartialOrd<TimeSpan> for TimeSpan {
    fn partial_cmp(&self, other: &TimeSpan) -> Option<Ordering> {
        let ticks_me = self.ticks();
        let ticks_ot = other.ticks();
        ticks_me.partial_cmp(&ticks_ot)
    }
}

impl Default for TimeSpan {
    fn default() -> Self {
        Self::new(0)
    }
}