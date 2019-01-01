
use crate::Error;
use crate::Connector;
use crate::UniverseGroup;

use crate::unit::UnitData;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::any_unit::prelude::*;

pub trait AiUnit : Unit {

    fn hull(&self) -> f32;

    fn hull_max(&self) -> f32;

    fn hull_armor(&self) -> f32;

    fn shield(&self) -> f32;

    fn shield_max(&self) -> f32;

    fn shield_armor(&self) -> f32;
}

pub(crate) struct AiUnitData {
    unit:           UnitData,
    hull:           f32,
    hull_max:       f32,
    hull_armor:     f32,
    shield:         f32,
    shield_max:     f32,
    shield_armor:   f32,
}

impl AiUnitData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<AiUnitData, Error> {
        Ok(AiUnitData {
            unit: UnitData::from_reader(connector, universe_group, packet, reader)?,
            hull:           reader.read_single()?,
            hull_max:       reader.read_single()?,
            hull_armor:     reader.read_single()?,
            shield:         reader.read_single()?,
            shield_max:     reader.read_single()?,
            shield_armor:   reader.read_single()?,
        })
    }
}

impl AiUnit for AiUnitData {
    fn hull(&self) -> f32 {
        self.hull
    }

    fn hull_max(&self) -> f32 {
        self.hull_max
    }

    fn hull_armor(&self) -> f32 {
        self.hull_armor
    }

    fn shield(&self) -> f32 {
        self.shield
    }

    fn shield_max(&self) -> f32 {
        self.shield_max
    }

    fn shield_armor(&self) -> f32 {
        self.shield_armor
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for AiUnitData {
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
        self.unit.kind()
    }
}