
use crate::Error;
use crate::Color;
use crate::Connector;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::any_unit::prelude::*;

pub struct Switch {
    unit:   UnitData,
    color:  Color,
    range:              f32,
    switch_time_cycle:  u16,
    switch_time_current:u16,
    switched:           bool,
}

impl Switch {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<Switch, Error> {
        Ok(Switch {
            unit:   UnitData::from_reader(connector, universe_group, packet, reader)?,
            color:  Color::from_rgb(
                reader.read_single()?,
                reader.read_single()?,
                reader.read_single()?,
            ),
            range:              reader.read_single()?,
            switch_time_cycle:  reader.read_u16()?,
            switch_time_current:reader.read_u16()?,
            switched:           reader.read_bool()?,
        })
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn range(&self) -> f32 {
        self.range
    }

    pub fn switch_time_cycle(&self) -> u16 {
        self.switch_time_cycle
    }

    pub fn switch_time_current(&self) -> u16 {
        self.switch_time_current
    }

    pub fn is_switched(&self) -> bool {
        self.switched
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl Unit for Switch {
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
        UnitKind::Switch
    }
}