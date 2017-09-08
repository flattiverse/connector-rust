

use std::borrow::Borrow;

use Error;
use unit::UnitKind;

use net::Packet;
use net::BinaryReader;

use downcast_rs::Downcast;

impl_downcast!(UniverseEvent);
pub trait UniverseEvent : Downcast {

    fn kind(&self) -> UnitKind;

    fn name(&self) -> &str;
}

#[derive(Debug)]
pub struct UniverseEventData {
    kind: UnitKind,
    name: String
}

impl UniverseEventData {
    pub fn from_reader(_: &Packet, reader: &mut BinaryReader) -> Result<UniverseEventData, Error> {
        Ok(UniverseEventData {
            kind: UnitKind::from_id(reader.read_unsigned_byte()?),
            name: reader.read_string()?
        })
    }
}

impl<T: 'static + Borrow<UniverseEventData>> UniverseEvent for T {
    fn kind(&self) -> UnitKind {
        self.borrow().kind
    }

    fn name(&self) -> &str {
        &self.borrow().name
    }
}