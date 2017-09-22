
use std::fmt;

use Error;

use unit::UnitKind;

use event::UniverseEvent;
use event::UniverseEventData;

use net::Packet;
use net::BinaryReader;

#[derive(Debug)]
pub struct RepairUniverseEvent {
    data: UniverseEventData,
    hull_repair: f32,
    shield_load: f32,
}

impl RepairUniverseEvent {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<RepairUniverseEvent, Error> {
        Ok(RepairUniverseEvent {
            data:           UniverseEventData::from_reader(packet, reader)?,
            hull_repair:    reader.read_single()?,
            shield_load:    reader.read_single()?,
        })
    }

    pub fn hull_repair(&self) -> f32 {
        self.hull_repair
    }

    pub fn shield_load(&self) -> f32 {
        self.shield_load
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl UniverseEvent for RepairUniverseEvent {
    fn unit_kind(&self) -> UnitKind {
        self.data.unit_kind()
    }

    fn unit_name(&self) -> &str {
        self.data.unit_name()
    }
}

impl fmt::Display for RepairUniverseEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RepairUniverseEvent: {}; {}",
            self.hull_repair,
            self.shield_load
        )
    }
}