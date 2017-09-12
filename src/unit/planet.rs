
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

downcast!(Planet);
pub trait Planet : Unit {
    fn kind(&self) -> UnitKind {
        UnitKind::Planet
    }
}

pub struct PlanetData {
    unit: UnitData,
}

impl PlanetData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PlanetData, Error> {
        Ok(PlanetData {
            unit: UnitData::from_reader(connector, universe_group, packet, reader, UnitKind::Planet)?
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for PlanetData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for PlanetData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<PlanetData> + BorrowMut<PlanetData> + Unit> Planet for  T {

}