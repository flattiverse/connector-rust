
use crate::Error;


#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum PerformanceRequirement {
    No = 0,
    Yes = 1,
    High = 2,
}

impl PerformanceRequirement {
    pub fn from_id(id: u8) -> Result<PerformanceRequirement, Error> {
        match id {
            0 => Ok(PerformanceRequirement::No),
            1 => Ok(PerformanceRequirement::Yes),
            2 => Ok(PerformanceRequirement::High),
            _ => Err(Error::InvalidPerformanceRequirement(id))
        }
    }
}