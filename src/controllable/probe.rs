
use std::sync::Arc;


use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use controllable::ControllableData;

pub struct Probe {
    controllable: ControllableData,
}

impl Probe {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Probe, Error>  {
        Ok(Probe {
            controllable: ControllableData::from_reader(connector, packet, reader)?
        })
    }
}

impl AsRef<ControllableData> for Probe {
    fn as_ref(&self) -> &ControllableData {
        &self.controllable
    }
}