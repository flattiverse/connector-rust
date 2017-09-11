
use std::sync::Arc;

use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;

use controllable::Controllable;
use controllable::ControllableData;

use unit::UnitKind;

use net::Packet;
use net::BinaryReader;

downcast!(Drone);
pub trait Drone : Controllable {
    fn kind(&self) -> UnitKind {
        UnitKind::PlayerDrone
    }
}

pub struct DroneData {
    data: ControllableData
}

impl DroneData {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<DroneData, Error>  {
        Ok(DroneData {
            data: ControllableData::from_reader(connector, packet, reader)?
        })
    }
}

// implicitly 'extend' Controllable
impl Borrow<ControllableData> for DroneData {
    fn borrow(&self) -> &ControllableData {
        &self.data
    }
}
impl BorrowMut<ControllableData> for DroneData {
    fn borrow_mut(&mut self) -> &mut ControllableData {
        &mut self.data
    }
}

impl<T: 'static + Borrow<DroneData> + BorrowMut<DroneData> + Controllable> Drone for T {

}