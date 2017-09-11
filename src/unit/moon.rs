
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

downcast!(Moon);
pub trait Moon : Unit {
    fn kind(&self) -> UnitKind {
        UnitKind::Moon
    }
}

pub struct MoonData {
    unit: UnitData,
}

impl MoonData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<MoonData, Error> {
        Ok(MoonData {
            unit: UnitData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for MoonData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for MoonData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<MoonData> + BorrowMut<MoonData> + Unit> Moon for  T {

}