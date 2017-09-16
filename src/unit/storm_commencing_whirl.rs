
use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use unit::any_unit::prelude::*;

pub struct StormCommencingWhirl {
    unit:           UnitData,
    time:           u8,
    active_time:    u8,
    configured_grav:f32,
    hull_damage:    f32,
    shield_damage:  f32,
    energy_damage:  f32,
}

impl StormCommencingWhirl {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<StormCommencingWhirl, Error> {
        Ok(StormCommencingWhirl {
            unit:           UnitData::from_reader(connector, universe_group, packet, reader)?,
            time:           reader.read_unsigned_byte()?,
            active_time:    reader.read_unsigned_byte()?,
            configured_grav:reader.read_single()?,
            hull_damage:    reader.read_single()?,
            shield_damage:  reader.read_single()?,
            energy_damage:  reader.read_single()?,
        })
    }

    /// Time until this [StormCommencingWhirl] to becomes a [StormWhirl]
    pub fn time(&self) -> u8 {
        self.time
    }

    /// Time the [StormWhirl] will be active
    pub fn active_time(&self) -> u8 {
        self.active_time
    }

    pub fn configured_gravity(&self) -> f32 {
        self.configured_grav
    }

    pub fn hull_damage(&self) -> f32 {
        self.hull_damage
    }

    pub fn shield_damage(&self) -> f32 {
        self.shield_damage
    }

    pub fn energy_damage(&self) -> f32 {
        self.energy_damage
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for StormCommencingWhirl {
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
        UnitKind::StormCommencingWhirl
    }
}