
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::borrow::Borrow;

use Error;
use event::UniverseEvent;
use event::UniverseEventData;

use net::Packet;
use net::BinaryReader;

impl_downcast!(HarvestUniverseEvent);
pub trait HarvestUniverseEvent : UniverseEvent + Display + Debug {

    fn amount(&self) -> f32;

    fn hue(&self) -> f32;
}


#[derive(Debug)]
pub struct HarvestUniverseEventData {
    data: UniverseEventData,
    amount: f32,
    hue: f32,
}

impl HarvestUniverseEventData {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<HarvestUniverseEventData, Error> {
        Ok(HarvestUniverseEventData {
            data:   UniverseEventData::from_reader(packet, reader)?,
            amount: reader.read_single()?,
            hue:    reader.read_single()?,
        })
    }
}


// implicitly implement UniverseEvent
impl Borrow<UniverseEventData> for HarvestUniverseEventData {
    fn borrow(&self) -> &UniverseEventData {
        &self.data
    }
}

impl<T: 'static + Borrow<HarvestUniverseEventData> + UniverseEvent + Display + Debug> HarvestUniverseEvent for T {
    fn amount(&self) -> f32 {
        self.borrow().amount
    }

    fn hue(&self) -> f32 {
        self.borrow().hue
    }
}

impl Display for HarvestUniverseEventData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HarvestUniverseEvent: {}; {}°",
            self.amount,
            self.hue
        )
    }
}