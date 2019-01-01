
use std::fmt;

use crate::Error;

use crate::unit::UnitKind;

use crate::event::UniverseEvent;
use crate::event::UniverseEventData;

use crate::net::Packet;
use crate::net::BinaryReader;

#[derive(Debug)]
pub struct HarvestUniverseEvent {
    data: UniverseEventData,
    amount: f32,
    hue: f32,
}

impl HarvestUniverseEvent {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<HarvestUniverseEvent, Error> {
        Ok(HarvestUniverseEvent {
            data:   UniverseEventData::from_reader(packet, reader)?,
            amount: reader.read_single()?,
            hue:    reader.read_single()?,
        })
    }

    pub fn amount(&self) -> f32 {
        self.amount
    }

    pub fn hue(&self) -> f32 {
        self.hue
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl UniverseEvent for HarvestUniverseEvent {
    fn unit_kind(&self) -> UnitKind {
        self.data.unit_kind()
    }

    fn unit_name(&self) -> &str {
        self.data.unit_name()
    }
}

impl fmt::Display for HarvestUniverseEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HarvestUniverseEvent: {}; {}°",
            self.amount,
            self.hue
        )
    }
}