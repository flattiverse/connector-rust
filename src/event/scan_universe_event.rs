
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::borrow::Borrow;

use Error;
use unit::ScanInfo;
use event::UniverseEvent;
use event::UniverseEventData;

use net::Packet;
use net::BinaryReader;

impl_downcast!(ScanUniverseEvent);
pub trait ScanUniverseEvent : UniverseEvent + Display + Debug {

    fn scan_info(&self) -> &ScanInfo;
}

#[derive(Debug)]
pub struct ScanUniverseEventData {
    data: UniverseEventData,
    info: ScanInfo
}

impl ScanUniverseEventData {
    pub fn from_packet(packet: &Packet, reader: &mut BinaryReader) -> Result<ScanUniverseEventData, Error> {
        Ok(ScanUniverseEventData {
            data: UniverseEventData::from_reader(packet, reader)?,
            info: ScanInfo::new(
                reader.read_single()?,
                reader.read_single()?,
                reader.read_single()?
            )?
        })
    }
}


// implicitly implement UniverseEvent
impl Borrow<UniverseEventData> for ScanUniverseEventData {
    fn borrow(&self) -> &UniverseEventData {
        &self.data
    }
}

impl<T: 'static + Borrow<ScanUniverseEventData> + UniverseEvent + Display + Debug> ScanUniverseEvent for T {
    fn scan_info(&self) -> &ScanInfo {
        &self.borrow().info
    }
}

impl Display for ScanUniverseEventData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScanUniverseEvent: {} -> {}, range = {}", self.info.from_degree(), self.info.to_degree(), self.info.range())
    }
}