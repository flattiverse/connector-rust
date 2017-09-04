
use Error;

#[repr(u8)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum CrystalKind {
    /// The crystal has a low grade, 60% of all crystals produced are
    /// low grade, meaning the crystal has some major negative effects
    LowGrade = 0,

    /// This crystal has a regular quality, 20% of all crystals produced are
    /// regular quality. Regular crystals have minor negative effects.
    Regular = 1,

    /// This crystal is pure, 15% of all crystals produced are pure.
    /// Pure grade crystals have no negative effects.
    Pure = 2,

    /// Mastery grade crystals have a chance of nearly 5% to be produced.
    /// These crystals have increased positive main effects.
    Mastery = 3,

    /// Divine grade crystals have a chance of 0.1% to be produced.
    /// These crystals have increased positive main effects and major
    /// secondary effects.
    Divine = 4,

    /// A special crystal. These crystals cannot be renamed and
    /// have a special set of abilities.
    Special = 5,
}

impl CrystalKind {
    pub fn from_id(id: u8) -> Result<CrystalKind, Error> {
        Ok(match id {
            0 => CrystalKind::LowGrade,
            1 => CrystalKind::Regular,
            2 => CrystalKind::Pure,
            3 => CrystalKind::Mastery,
            4 => CrystalKind::Divine,
            5 => CrystalKind::Special,
            id@_ => return Err(Error::InvalidCrystalKind(id))
        })
    }
}