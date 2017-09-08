
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::borrow::Borrow;

use Error;
use unit::UnitKind;
use event::UniverseEvent;
use event::UniverseEventData;

use net::Packet;
use net::BinaryReader;

impl_downcast!(TransferredEnergyUniverseEvent);
pub trait TransferredEnergyUniverseEvent : UniverseEvent + Display + Debug {

    fn energy(&self) -> f32;

    fn particles(&self) -> f32;

    fn ions(&self) -> f32;

    fn source_kind(&self) -> UnitKind;

    fn source_name(&self) -> &str;
}


#[derive(Debug)]
pub struct TransferredEnergyUniverseEventData {
    data:           UniverseEventData,
    energy:         f32,
    particles:      f32,
    ions:           f32,
    source_kind:    UnitKind,
    source_name:    String,
}

impl TransferredEnergyUniverseEventData {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<TransferredEnergyUniverseEventData, Error> {
        Ok(TransferredEnergyUniverseEventData {
            data:       UniverseEventData::from_reader(packet, reader)?,
            energy:     reader.read_single()?,
            particles:  reader.read_single()?,
            ions:       reader.read_single()?,
            source_kind:UnitKind::from_id(reader.read_unsigned_byte()?),
            source_name:reader.read_string()?,
        })
    }
}

// implicitly implement UniverseEvent
impl Borrow<UniverseEventData> for TransferredEnergyUniverseEventData {
    fn borrow(&self) -> &UniverseEventData {
        &self.data
    }
}

impl<T: 'static + Borrow<TransferredEnergyUniverseEventData> + UniverseEvent + Display + Debug> TransferredEnergyUniverseEvent for T {
    fn energy(&self) -> f32 {
        self.borrow().energy
    }

    fn particles(&self) -> f32 {
        self.borrow().particles
    }

    fn ions(&self) -> f32 {
        self.borrow().ions
    }

    fn source_kind(&self) -> UnitKind {
        self.borrow().source_kind
    }

    fn source_name(&self) -> &str {
        &self.borrow().source_name
    }
}

impl Display for TransferredEnergyUniverseEventData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TransferredEnergyUniverseEvent: {}; {}, {} from {}",
            self.energy,
            self.particles,
            self.ions,
            self.source_name
        )
    }
}