
use std::fmt;

use Error;

use unit::ScanInfo;
use unit::UnitKind;

use event::UniverseEvent;
use event::UniverseEventData;

use net::Packet;
use net::BinaryReader;

#[derive(Debug)]
pub struct ScanUniverseEvent {
    data: UniverseEventData,
    info: ScanInfo
}

impl ScanUniverseEvent {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<ScanUniverseEvent, Error> {
        Ok(ScanUniverseEvent {
            data: UniverseEventData::from_reader(packet, reader)?,
            info: ScanInfo::new(
                reader.read_single()?,
                reader.read_single()?,
                reader.read_single()?
            )?
        })
    }

    pub fn scan_info(&self) -> &ScanInfo {
        &self.info
    }
}

// TODO replace with delegation directive
// once standardized: https://github.com/rust-lang/rfcs/pull/1406
impl UniverseEvent for ScanUniverseEvent {
    fn unit_kind(&self) -> UnitKind {
        self.data.unit_kind()
    }

    fn unit_name(&self) -> &str {
        self.data.unit_name()
    }
}

impl fmt::Display for ScanUniverseEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScanUniverseEvent: {} -> {}, range = {}", self.info.from_degree(), self.info.to_degree(), self.info.range())
    }
}