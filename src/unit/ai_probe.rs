
use crate::Error;
use crate::Connector;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::any_ai_unit::prelude::*;

pub struct AiProbe {
    pub(crate) unit: AiUnitData,
}

impl AiProbe {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<AiProbe, Error> {
        Ok(AiProbe {
            unit: AiUnitData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for AiProbe {
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
        UnitKind::AiProbe
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl AiUnit for AiProbe {
    fn hull(&self) -> f32 {
        self.unit.hull()
    }

    fn hull_max(&self) -> f32 {
        self.unit.hull_max()
    }

    fn hull_armor(&self) -> f32 {
        self.unit.hull_armor()
    }

    fn shield(&self) -> f32 {
        self.unit.shield()
    }

    fn shield_max(&self) -> f32 {
        self.unit.shield_max()
    }

    fn shield_armor(&self) -> f32 {
        self.unit.shield_armor()
    }
}