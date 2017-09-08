

use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::borrow::Borrow;

use Error;
use event::UniverseEvent;
use event::UniverseEventData;

use net::Packet;
use net::BinaryReader;
use net::is_set_u8;

impl_downcast!(DamageUniverseEvent);
pub trait DamageUniverseEvent : UniverseEvent + Display + Debug {

    fn hull_damage(&self) -> f32;

    fn hull_damage_was_critical_strike(&self) -> bool;

    fn shield_damage(&self) -> f32;

    fn shield_damage_was_critical_strike(&self) -> bool;

    fn energy_damage(&self) -> f32;

    fn energy_damage_was_critical_strike(&self) -> bool;
}

#[derive(Debug)]
pub struct DamageUniverseEventData {
    data: UniverseEventData,
    hull_damage: f32,
    hull_damage_was_critical_strike: bool,
    shield_damage: f32,
    shield_damage_was_critical_strike: bool,
    energy_damage: f32,
    energy_damage_was_critical_strike: bool,
}

impl DamageUniverseEventData {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<DamageUniverseEventData, Error> {
        let header = reader.read_unsigned_byte()?;
        Ok(DamageUniverseEventData {
            data: UniverseEventData::from_reader(packet, reader)?,

            hull_damage:    reader.read_single()?,
            shield_damage:  reader.read_single()?,
            energy_damage:  reader.read_single()?,

            hull_damage_was_critical_strike:    is_set_u8(header, 0x01),
            shield_damage_was_critical_strike:  is_set_u8(header, 0x02),
            energy_damage_was_critical_strike:  is_set_u8(header, 0x04),
        })
    }
}


// implicitly implement UniverseEvent
impl Borrow<UniverseEventData> for DamageUniverseEventData {
    fn borrow(&self) -> &UniverseEventData {
        &self.data
    }
}


impl<T: 'static + Borrow<DamageUniverseEventData> + UniverseEvent + Display + Debug> DamageUniverseEvent for T {
    fn hull_damage(&self) -> f32 {
        self.borrow().hull_damage
    }

    fn hull_damage_was_critical_strike(&self) -> bool {
        self.borrow().hull_damage_was_critical_strike
    }

    fn shield_damage(&self) -> f32 {
        self.borrow().shield_damage
    }

    fn shield_damage_was_critical_strike(&self) -> bool {
        self.borrow().shield_damage_was_critical_strike
    }

    fn energy_damage(&self) -> f32 {
        self.borrow().energy_damage
    }

    fn energy_damage_was_critical_strike(&self) -> bool {
        self.borrow().energy_damage_was_critical_strike
    }
}

impl Display for DamageUniverseEventData {
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