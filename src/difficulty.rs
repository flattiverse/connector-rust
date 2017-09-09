
use Error;


#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Difficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2,
    Insane = 3,
}

impl Difficulty {
    pub fn from_id(id: u8) -> Result<Difficulty, Error> {
        match id {
            0 => Ok(Difficulty::Easy),
            1 => Ok(Difficulty::Medium),
            2 => Ok(Difficulty::Hard),
            3 => Ok(Difficulty::Insane),
            _ => Err(Error::InvalidDifficulty(id))
        }
    }
}