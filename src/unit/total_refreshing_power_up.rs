
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

downcast!(TotalRefreshPowerUp);
pub trait TotalRefreshPowerUp : RefreshingPowerUp {
    fn kind(&self) -> UnitKind {
        UnitKind::TotalRefreshPowerUp
    }
}

pub struct TotalRefreshPowerUpData {
    unit: RefreshingPowerUpData,
}

impl TotalRefreshPowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<TotalRefreshPowerUpData, Error> {
        Ok(TotalRefreshPowerUpData {
            unit: RefreshingPowerUpData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement RefreshingPowerUp
impl Borrow<RefreshingPowerUpData> for TotalRefreshPowerUpData {
    fn borrow(&self) -> &RefreshingPowerUpData {
        &self.unit
    }
}
impl BorrowMut<RefreshingPowerUpData> for TotalRefreshPowerUpData {
    fn borrow_mut(&mut self) -> &mut RefreshingPowerUpData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for TotalRefreshPowerUpData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for TotalRefreshPowerUpData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<TotalRefreshPowerUpData> + BorrowMut<TotalRefreshPowerUpData> + RefreshingPowerUp> TotalRefreshPowerUp for  T {

}