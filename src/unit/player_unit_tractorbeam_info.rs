
use crate::Error;
use crate::net::BinaryReader;


pub struct PlayerUnitTractorbeamInfo {
    direction:     f32,
    range:         f32,
    force:         f32,
    self_affected: bool,
}

impl PlayerUnitTractorbeamInfo {
    pub fn for_reader(reader: &mut BinaryReader) -> Result<PlayerUnitTractorbeamInfo, Error> {
        Ok(PlayerUnitTractorbeamInfo {
            direction:      reader.read_single()?,
            range:          reader.read_single()?,
            force:          reader.read_single()?,
            self_affected:  reader.read_byte()? == 1,
        })
    }

    /// The direction of the tractor-beam. The tractor-beam is
    /// about -15° and +15° the given direction
    pub fn direction(&self) -> f32 {
        self.direction
    }

    /// The range of the tractor-beam. The effective range is
    /// the value + the unit radius
    pub fn range(&self) -> f32 {
        self.range
    }

    /// The effective force of the tractor-beam. A positive value
    /// means the unit is pulling other units towards it. A negative
    /// value means the unit is pushing other units away from it.
    pub fn force(&self) -> f32 {
        self.force
    }

    /// Whether the tractor-beam touched a [Mobility#Steady] or
    /// [Mobility#Still] unit. If so, the unit itself gets moved
    /// towards or away from that unit.
    pub fn self_affected(&self) -> bool {
        self.self_affected
    }
}