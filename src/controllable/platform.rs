
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

downcast!(Platform);
pub trait Platform : Controllable {
    fn kind(&self) -> UnitKind {
        UnitKind::PlayerPlatform
    }
}

pub struct PlatformData {
    data: ControllableData
}

impl PlatformData {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<PlatformData, Error>  {
        Ok(PlatformData {
            data: ControllableData::from_reader(connector, packet, reader)?
        })
    }
}

// implicitly 'extend' Controllable
impl Borrow<ControllableData> for PlatformData {
    fn borrow(&self) -> &ControllableData {
        &self.data
    }
}
impl BorrowMut<ControllableData> for PlatformData {
    fn borrow_mut(&mut self) -> &mut ControllableData {
        &mut self.data
    }
}

impl<T: 'static + Borrow<PlatformData> + BorrowMut<PlatformData> + Controllable> Platform for T {

}