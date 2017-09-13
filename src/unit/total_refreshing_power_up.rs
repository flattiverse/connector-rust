
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::UnitData;
use unit::RefreshingPowerUp;
use unit::RefreshingPowerUpData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

downcast!(TotalRefreshingPowerUp);
pub trait TotalRefreshingPowerUp: RefreshingPowerUp {

}

pub struct TotalRefreshingPowerUpData {
    unit: RefreshingPowerUpData,
}

impl TotalRefreshingPowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<TotalRefreshingPowerUpData, Error> {
        Ok(TotalRefreshingPowerUpData {
            unit: RefreshingPowerUpData::from_reader(connector, universe_group, packet, reader, UnitKind::TotalRefreshPowerUp)?
        })
    }
}


// implicitly implement RefreshingPowerUp
impl Borrow<RefreshingPowerUpData> for TotalRefreshingPowerUpData {
    fn borrow(&self) -> &RefreshingPowerUpData {
        &self.unit
    }
}
impl BorrowMut<RefreshingPowerUpData> for TotalRefreshingPowerUpData {
    fn borrow_mut(&mut self) -> &mut RefreshingPowerUpData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for TotalRefreshingPowerUpData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for TotalRefreshingPowerUpData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<TotalRefreshingPowerUpData> + BorrowMut<TotalRefreshingPowerUpData> + RefreshingPowerUp> TotalRefreshingPowerUp for  T {

}