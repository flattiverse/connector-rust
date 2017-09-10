
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

impl_downcast!(Base);
pub trait Base : Controllable {
    fn kind(&self) -> UnitKind {
        UnitKind::PlayerBase
    }
}

pub struct BaseData {
    data: ControllableData
}

impl BaseData {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<BaseData, Error>  {
        Ok(BaseData {
            data: ControllableData::from_reader(connector, packet, reader)?
        })
    }
}

// implicitly 'extend' Controllable
impl Borrow<ControllableData> for BaseData {
    fn borrow(&self) -> &ControllableData {
        &self.data
    }
}
impl BorrowMut<ControllableData> for BaseData {
    fn borrow_mut(&mut self) -> &mut ControllableData {
        &mut self.data
    }
}

impl<T: 'static + Borrow<BaseData> + BorrowMut<BaseData> + Controllable> Base for T {

}