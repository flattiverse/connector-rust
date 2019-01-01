
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use std::ops::Add;

use std::time::{SystemTime, UNIX_EPOCH};

use chrono;
use chrono::naive::NaiveDateTime;

use crate::dotnet::TimeSpan;

// FUCK YOU M$
const MILLIS_OFFSET : i64 = -1970 * 365 * 24 * 60 * 60 * 1_000
                               -3 *  30 * 24 * 60 * 60 * 1_000
                                    -22 * 24 * 60 * 60 * 1_000;
const TICKS_PER_MILLI : i64 = 10_000;

#[derive(Copy, Clone)]
pub struct DateTime {
    ticks: i64
}

impl DateTime {
    pub fn from_ticks(ticks: i64) -> DateTime {
        DateTime {
            ticks
        }
    }

    pub fn now() -> DateTime {

        let time   = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let secs_ticks = ((time.as_secs() as i64 * 1_000_i64) - MILLIS_OFFSET) * TICKS_PER_MILLI;
        let nano_ticks = time.subsec_nanos() as i64 / 100_i64;
        DateTime {
            ticks: secs_ticks + nano_ticks
        }
    }

    pub fn ticks(&self) -> i64 {
        self.ticks
    }

    pub fn millis(&self) -> i64 {
        (self.ticks / TICKS_PER_MILLI) + MILLIS_OFFSET
    }

    pub fn naive_date_time(&self) -> NaiveDateTime {
        let secs = self.millis() / 1_000;
        let nano = (self.millis() as u64 * 1_000_000_u64) % 1_000_000_000_u64;
        NaiveDateTime::from_timestamp(secs, nano as u32)
    }

    pub fn elapsed_millis(&self) -> i64 {
        let now = chrono::Local::now().naive_local();
        now.signed_duration_since(self.naive_date_time()).num_milliseconds()
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.naive_date_time().format("%Y-%m-%d %H:%M:%S").fmt(f)
    }
}

impl Add<TimeSpan> for DateTime {
    type Output = DateTime;

    fn add(self, rhs: TimeSpan) -> Self::Output {
        DateTime {
            ticks: self.ticks + rhs.ticks()
        }
    }
}