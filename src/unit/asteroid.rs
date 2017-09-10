
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

impl_downcast!(Asteroid);
pub trait Asteroid : Unit {

    fn aggressive(&self) -> bool;

    fn max_speed(&self) -> f32;

    fn kind(&self) -> UnitKind {
        UnitKind::Asteroid
    }
}

pub struct AsteroidData {
    unit:       UnitData,
    aggressive: bool,
    max_speed:  f32,
}

impl AsteroidData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<AsteroidData, Error> {
        Ok(AsteroidData {
            unit:       UnitData::from_reader(connector, universe_group, packet, reader)?,
            max_speed:  reader.read_single()?,
            aggressive: reader.read_byte()? == 1,
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for AsteroidData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for AsteroidData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<AsteroidData> + BorrowMut<AsteroidData> + Unit> Asteroid for  T {
    fn aggressive(&self) -> bool {
        self.borrow().aggressive
    }

    fn max_speed(&self) -> f32 {
        self.borrow().max_speed
    }
}