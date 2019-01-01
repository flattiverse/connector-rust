
use crate::Error;
use crate::Connector;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::any_unit::prelude::*;

pub struct Storm {
    unit: UnitData,
    max_whirls: u8,
    child_min_announcement_time:    u8,
    child_max_announcement_time:    u8,
    child_min_active_time:          u8,
    child_max_active_time:          u8,
    child_min_size:     f32,
    child_max_size:     f32,
    child_min_speed:    f32,
    child_max_speed:    f32,
    child_min_gravity:  f32,
    child_max_gravity:  f32,
    min_hull_damage:    f32,
    max_hull_damage:    f32,
    min_shield_damage:  f32,
    max_shield_damage:  f32,
    min_energy_damage:  f32,
    max_energy_damage:  f32,
}

impl Storm {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<Storm, Error> {
        Ok(Storm {
            unit: UnitData::from_reader(connector, universe_group, packet, reader)?,
            max_whirls:                     reader.read_unsigned_byte()?,
            child_min_announcement_time:    reader.read_unsigned_byte()?,
            child_max_announcement_time:    reader.read_unsigned_byte()?,
            child_min_active_time:          reader.read_unsigned_byte()?,
            child_max_active_time:          reader.read_unsigned_byte()?,
            child_min_size:                 reader.read_single()?,
            child_max_size:                 reader.read_single()?,
            child_min_speed:                reader.read_single()?,
            child_max_speed:                reader.read_single()?,
            child_min_gravity:              reader.read_single()?,
            child_max_gravity:              reader.read_single()?,
            min_hull_damage:                reader.read_single()?,
            max_hull_damage:                reader.read_single()?,
            min_shield_damage:              reader.read_single()?,
            max_shield_damage:              reader.read_single()?,
            min_energy_damage:              reader.read_single()?,
            max_energy_damage:              reader.read_single()?,
        })
    }

    pub fn max_whirls(&self) -> u8 {
        self.max_whirls
    }

    pub fn child_min_announcement_time(&self) -> u8 {
        self.child_min_announcement_time
    }

    pub fn child_max_announcement_time(&self) -> u8 {
        self.child_max_announcement_time
    }

    pub fn child_min_active_time(&self) -> u8 {
        self.child_min_active_time
    }

    pub fn child_max_active_time(&self) -> u8 {
        self.child_max_active_time
    }

    pub fn child_min_size(&self) -> f32 {
        self.child_min_size
    }

    pub fn child_max_size(&self) -> f32 {
        self.child_max_size
    }

    pub fn child_min_speed(&self) -> f32 {
        self.child_min_speed
    }

    pub fn child_max_speed(&self) -> f32 {
        self.child_max_speed
    }

    pub fn child_min_gravity(&self) -> f32 {
        self.child_min_gravity
    }

    pub fn child_max_gravity(&self) -> f32 {
        self.child_max_gravity
    }

    pub fn min_hull_damage(&self) -> f32 {
        self.min_hull_damage
    }

    pub fn max_hull_damage(&self) -> f32 {
        self.max_hull_damage
    }

    pub fn min_shield_damage(&self) -> f32 {
        self.min_shield_damage
    }

    pub fn max_shield_damage(&self) -> f32 {
        self.max_shield_damage
    }

    pub fn min_energy_damage(&self) -> f32 {
        self.min_energy_damage
    }

    pub fn max_energy_damage(&self) -> f32 {
        self.max_energy_damage
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for Storm {
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
        UnitKind::Storm
    }
}