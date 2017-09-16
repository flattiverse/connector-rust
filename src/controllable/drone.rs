
use std::sync::Arc;


use Error;
use Connector;

use net::Packet;
use net::BinaryReader;

use controllable::ControllableData;

pub struct Drone {
    controllable: ControllableData,
}

impl Drone {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<Drone, Error>  {
        Ok(Drone {
            controllable: ControllableData::from_reader(connector, packet, reader)?
        })
    }
}

impl AsRef<ControllableData> for Drone {
    fn as_ref(&self) -> &ControllableData {
        &self.controllable
    }
}