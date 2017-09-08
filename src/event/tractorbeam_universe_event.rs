
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::borrow::Borrow;

use Error;
use event::UniverseEvent;
use event::UniverseEventData;

use net::Packet;
use net::BinaryReader;

impl_downcast!(TractorbeamUniverseEvent);
pub trait TractorbeamUniverseEvent : UniverseEvent + Display + Debug {

    fn direction(&self) -> f32;

    fn range(&self) -> f32;

    fn force(&self) -> f32;

    fn self_affected(&self) -> bool;
}


#[derive(Debug)]
pub struct TractorbeamUniverseEventData {
    data:           UniverseEventData,
    direction:      f32,
    range:          f32,
    force:          f32,
    self_affected:  bool,
}

impl TractorbeamUniverseEventData {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<TractorbeamUniverseEventData, Error> {
        Ok(TractorbeamUniverseEventData {
            data:           UniverseEventData::from_reader(packet, reader)?,
            direction:      reader.read_single()?,
            range:          reader.read_single()?,
            force:          reader.read_single()?,
            self_affected:  reader.read_byte()? == 1,
        })
    }
}

// implicitly implement UniverseEvent
impl Borrow<UniverseEventData> for TractorbeamUniverseEventData {
    fn borrow(&self) -> &UniverseEventData {
        &self.data
    }
}

impl<T: 'static + Borrow<TractorbeamUniverseEventData> + UniverseEvent + Display + Debug> TractorbeamUniverseEvent for T {
    fn direction(&self) -> f32 {
        self.borrow().direction
    }

    fn range(&self) -> f32 {
        self.borrow().range
    }

    fn force(&self) -> f32 {
        self.borrow().force
    }

    fn self_affected(&self) -> bool {
        self.borrow().self_affected
    }
}

impl Display for TractorbeamUniverseEventData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TractorbeamUniverseEvent: {}° range={} force={} self_affected={}",
            self.direction,
            self.range,
            self.force,
            self.self_affected
        )
    }
}