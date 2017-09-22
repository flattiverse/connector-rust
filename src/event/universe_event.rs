
use std::fmt;

use Error;

use unit::UnitKind;

use net::Packet;
use net::BinaryReader;

pub trait UniverseEvent : fmt::Display {

    fn unit_kind(&self) -> UnitKind;

    fn unit_name(&self) -> &str;
}

#[derive(Debug)]
pub(crate) struct UniverseEventData {
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

impl UniverseEvent for UniverseEventData {
    fn unit_kind(&self) -> UnitKind {
        self.kind
    }

    fn unit_name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for UniverseEventData {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}