
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

downcast!(Meteoroid);
pub trait Meteoroid : Unit {

}

pub struct MeteoroidData {
    unit: UnitData,
}

impl MeteoroidData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<MeteoroidData, Error> {
        Ok(MeteoroidData {
            unit: UnitData::from_reader(connector, universe_group, packet, reader, UnitKind::Meteoroid)?
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for MeteoroidData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for MeteoroidData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<MeteoroidData> + BorrowMut<MeteoroidData> + Unit> Meteoroid for  T {

}