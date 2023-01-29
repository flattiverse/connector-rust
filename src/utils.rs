use std::time::UNIX_EPOCH;

pub fn current_time_millis() -> u64 {
    UNIX_EPOCH.elapsed().unwrap_or_default().as_millis() as u64
}
