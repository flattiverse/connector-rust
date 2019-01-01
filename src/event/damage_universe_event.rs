
use std::fmt;

use crate::Error;

use crate::unit::UnitKind;

use crate::event::UniverseEvent;
use crate::event::UniverseEventData;

use crate::net::Packet;
use crate::net::BinaryReader;
use crate::net::is_set_u8;

#[derive(Debug)]
pub struct DamageUniverseEvent {
    data: UniverseEventData,
    hull_damage: f32,
    hull_damage_was_critical_strike: bool,
    shield_damage: f32,
    shield_damage_was_critical_strike: bool,
    energy_damage: f32,
    energy_damage_was_critical_strike: bool,
}

impl DamageUniverseEvent {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<DamageUniverseEvent, Error> {
        let header = reader.read_unsigned_byte()?;
        Ok(DamageUniverseEvent {
            data: UniverseEventData::from_reader(packet, reader)?,

            hull_damage:    reader.read_single()?,
            shield_damage:  reader.read_single()?,
            energy_damage:  reader.read_single()?,

            hull_damage_was_critical_strike:    is_set_u8(header, 0x01),
            shield_damage_was_critical_strike:  is_set_u8(header, 0x02),
            energy_damage_was_critical_strike:  is_set_u8(header, 0x04),
        })
    }

    pub fn hull_damage(&self) -> f32 {
        self.hull_damage
    }

    pub fn hull_damage_was_critical_strike(&self) -> bool {
        self.hull_damage_was_critical_strike
    }

    pub fn shield_damage(&self) -> f32 {
        self.shield_damage
    }

    pub fn shield_damage_was_critical_strike(&self) -> bool {
        self.shield_damage_was_critical_strike
    }

    pub fn energy_damage(&self) -> f32 {
        self.energy_damage
    }

    pub fn energy_damage_was_critical_strike(&self) -> bool {
        self.energy_damage_was_critical_strike
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl UniverseEvent for DamageUniverseEvent {
    fn unit_kind(&self) -> UnitKind {
        self.data.unit_kind()
    }

    fn unit_name(&self) -> &str {
        self.data.unit_name()
    }
}

impl fmt::Display for DamageUniverseEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DamageUniverseEvent: {}{}; {}{}; {}{}",
            self.hull_damage,
            if self.hull_damage_was_critical_strike {"*"} else {""},
            self.shield_damage,
            if self.shield_damage_was_critical_strike {"*"} else {""},
            self.energy_damage,
            if self.energy_damage_was_critical_strike {"*"} else {""},
        )
    }
}