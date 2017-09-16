
use std::sync::Arc;


use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use controllable::ControllableData;

pub struct Platform {
    controllable: ControllableData,
}

impl Platform {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Platform, Error>  {
        Ok(Platform {
            controllable: ControllableData::from_reader(connector, packet, reader)?
        })
    }
}

impl AsRef<ControllableData> for Platform {
    fn as_ref(&self) -> &ControllableData {
        &self.controllable
    }
}