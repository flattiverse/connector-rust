
use crate::Error;
use crate::Connector;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::GravityWell;
use crate::unit::any_unit::prelude::*;

pub struct BlackHole {
    unit:  UnitData,
    wells: Vec<GravityWell>
}

impl BlackHole {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<BlackHole, Error> {
        Ok(BlackHole {
            unit:  UnitData::from_reader(connector, universe_group, packet, reader)?,
            wells: {
                let mut vec = Vec::new();
                let count = reader.read_unsigned_byte()?;
                for _ in 0..count {
                    vec.push(GravityWell::from_reader(reader)?);
                }
                vec
            },
        })
    }

    pub fn gravity_wells(&self) -> &Vec<GravityWell> {
        &self.wells
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for BlackHole {
    fn name(&self) -> &str {
        self.unit.name()
    }

    fn position(&self) -> &Vector {
        self.unit.position()
    }

    fn movement(&self) -> &Vector {
        self.unit.movement()
    }

    fn radius(&self) -> f32 {
        self.unit.radius()
    }

    fn gravity(&self) -> f32 {
        self.unit.gravity()
    }

    fn team(&self) -> &Weak<Team> {
        self.unit.team()
    }

    fn is_solid(&self) -> bool {
        self.unit.is_solid()
    }

    fn is_masking(&self) -> bool {
        self.unit.is_masking()
    }

    fn is_visible(&self) -> bool {
        self.unit.is_visible()
    }

    fn is_orbiting(&self) -> bool {
        self.unit.is_orbiting()
    }

    fn orbiting_center(&self) -> &Option<Vector> {
        self.unit.orbiting_center()
    }

    fn orbiting_states(&self) -> &Option<Vec<OrbitingState>> {
        self.unit.orbiting_states()
    }

    fn mobility(&self) -> Mobility {
        self.unit.mobility()
    }

    fn connector(&self) -> &Weak<Connector> {
        self.unit.connector()
    }

    fn kind(&self) -> UnitKind {
        UnitKind::BlackHole
    }
}