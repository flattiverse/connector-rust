
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::borrow::Borrow;

use Error;
use event::UniverseEvent;
use event::UniverseEventData;

use net::Packet;
use net::BinaryReader;

downcast!(RepairUniverseEvent);
pub trait RepairUniverseEvent : UniverseEvent + Display + Debug {

    fn hull_repair(&self) -> f32;

    fn shield_load(&self) -> f32;
}


#[derive(Debug)]
pub struct RepairUniverseEventData {
    data: UniverseEventData,
    hull_repair: f32,
    shield_load: f32,
}

impl RepairUniverseEventData {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<RepairUniverseEventData, Error> {
        Ok(RepairUniverseEventData {
            data:           UniverseEventData::from_reader(packet, reader)?,
            hull_repair:    reader.read_single()?,
            shield_load:    reader.read_single()?,
        })
    }
}

// implicitly implement UniverseEvent
impl Borrow<UniverseEventData> for RepairUniverseEventData {
    fn borrow(&self) -> &UniverseEventData {
        &self.data
    }
}


impl<T: 'static + Borrow<RepairUniverseEventData> + UniverseEvent + Display + Debug> RepairUniverseEvent for T {
    fn hull_repair(&self) -> f32 {
        self.borrow().hull_repair
    }

    fn shield_load(&self) -> f32 {
        self.borrow().shield_load
    }
}

impl Display for RepairUniverseEventData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RepairUniverseEvent: {}; {}",
            self.hull_repair,
            self.shield_load
        )
    }
}