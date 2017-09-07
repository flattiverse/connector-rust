
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Connector;
use UniverseGroup;
use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

pub trait Planet : Unit {
    fn kind(&self) -> UnitKind {
        UnitKind::Planet
    }
}


pub(crate) struct PlanetData {
    unit: UnitData,
}

impl PlanetData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PlanetData, Error> {
        Ok(PlanetData {
            unit: UnitData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


impl Borrow<Unit> for PlanetData {
    fn borrow(&self) -> &Unit {
        &self.unit
    }
}

impl BorrowMut<Unit> for PlanetData {
    fn borrow_mut(&mut self) -> &mut Unit {
        &mut self.unit
    }
}

impl<T: Borrow<PlanetData> + BorrowMut<PlanetData> + Unit> Planet for  T {

}