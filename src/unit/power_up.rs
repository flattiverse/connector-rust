
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

impl_downcast!(PowerUp);
pub trait PowerUp : Unit {
}

pub struct PowerUpData {
    unit: UnitData,
}

impl PowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PowerUpData, Error> {
        Ok(PowerUpData {
            unit: UnitData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for PowerUpData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for PowerUpData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<PowerUpData> + BorrowMut<PowerUpData> + Unit> PowerUp for  T {

}