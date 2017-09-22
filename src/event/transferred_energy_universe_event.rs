
use std::fmt;

use Error;

use unit::UnitKind;

use event::UniverseEvent;
use event::UniverseEventData;

use net::Packet;
use net::BinaryReader;


#[derive(Debug)]
pub struct TransferredEnergyUniverseEvent {
    data:           UniverseEventData,
    energy:         f32,
    particles:      f32,
    ions:           f32,
    source_kind:    UnitKind,
    source_name:    String,
}

impl TransferredEnergyUniverseEvent {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<TransferredEnergyUniverseEvent, Error> {
        Ok(TransferredEnergyUniverseEvent {
            data:       UniverseEventData::from_reader(packet, reader)?,
            energy:     reader.read_single()?,
            particles:  reader.read_single()?,
            ions:       reader.read_single()?,
            source_kind:UnitKind::from_id(reader.read_unsigned_byte()?),
            source_name:reader.read_string()?,
        })
    }

    pub fn energy(&self) -> f32 {
        self.energy
    }

    pub fn particles(&self) -> f32 {
        self.particles
    }

    pub fn ions(&self) -> f32 {
        self.ions
    }

    pub fn source_unit_kind(&self) -> UnitKind {
        self.source_kind
    }

    pub fn source_unit_name(&self) -> &str {
        &self.source_name
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl UniverseEvent for TransferredEnergyUniverseEvent {
    fn unit_kind(&self) -> UnitKind {
        self.data.unit_kind()
    }

    fn unit_name(&self) -> &str {
        self.data.unit_name()
    }
}

impl fmt::Display for TransferredEnergyUniverseEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TransferredEnergyUniverseEvent: {}; {}, {} from {}",
            self.energy,
            self.particles,
            self.ions,
            self.source_name
        )
    }
}