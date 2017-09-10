
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

impl_downcast!(Probe);
pub trait Probe : Controllable {
    fn kind(&self) -> UnitKind {
        UnitKind::PlayerProbe
    }
}

pub struct ProbeData {
    data: ControllableData
}

impl ProbeData {
    pub fn from_reader(connector: &Arc<Connector>, packet: &Packet, reader: &mut BinaryReader) -> Result<ProbeData, Error>  {
        Ok(ProbeData {
            data: ControllableData::from_reader(connector, packet, reader)?
        })
    }
}

// implicitly 'extend' Controllable
impl Borrow<ControllableData> for ProbeData {
    fn borrow(&self) -> &ControllableData {
        &self.data
    }
}
impl BorrowMut<ControllableData> for ProbeData {
    fn borrow_mut(&mut self) -> &mut ControllableData {
        &mut self.data
    }
}

impl<T: 'static + Borrow<ProbeData> + BorrowMut<ProbeData> + Controllable> Probe for T {

}