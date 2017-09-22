
use std::fmt;

use Error;

use unit::UnitKind;

use event::UniverseEvent;
use event::UniverseEventData;

use net::Packet;
use net::BinaryReader;

#[derive(Debug)]
pub struct TractorbeamUniverseEvent {
    data:           UniverseEventData,
    direction:      f32,
    range:          f32,
    force:          f32,
    self_affected:  bool,
}

impl TractorbeamUniverseEvent {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<TractorbeamUniverseEvent, Error> {
        Ok(TractorbeamUniverseEvent {
            data:           UniverseEventData::from_reader(packet, reader)?,
            direction:      reader.read_single()?,
            range:          reader.read_single()?,
            force:          reader.read_single()?,
            self_affected:  reader.read_byte()? == 1,
        })
    }

    pub fn direction(&self) -> f32 {
        self.direction
    }

    pub fn range(&self) -> f32 {
        self.range
    }

    pub fn force(&self) -> f32 {
        self.force
    }

    pub fn is_self_affected(&self) -> bool {
        self.self_affected
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl UniverseEvent for TractorbeamUniverseEvent {
    fn unit_kind(&self) -> UnitKind {
        self.data.unit_kind()
    }

    fn unit_name(&self) -> &str {
        self.data.unit_name()
    }
}

impl fmt::Display for TractorbeamUniverseEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TractorbeamUniverseEvent: {}° range={} force={} self_affected={}",
            self.direction,
            self.range,
            self.force,
            self.self_affected
        )
    }
}