
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

downcast!(ShieldRefreshingPowerUp);
pub trait ShieldRefreshingPowerUp : RefreshingPowerUp {
    fn kind(&self) -> UnitKind {
        UnitKind::ShieldPowerUp
    }
}

pub struct ShieldRefreshingPowerUpData {
    unit: RefreshingPowerUpData,
}

impl ShieldRefreshingPowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<ShieldRefreshingPowerUpData, Error> {
        Ok(ShieldRefreshingPowerUpData {
            unit: RefreshingPowerUpData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement RefreshingPowerUp
impl Borrow<RefreshingPowerUpData> for ShieldRefreshingPowerUpData {
    fn borrow(&self) -> &RefreshingPowerUpData {
        &self.unit
    }
}
impl BorrowMut<RefreshingPowerUpData> for ShieldRefreshingPowerUpData {
    fn borrow_mut(&mut self) -> &mut RefreshingPowerUpData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for ShieldRefreshingPowerUpData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for ShieldRefreshingPowerUpData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<ShieldRefreshingPowerUpData> + BorrowMut<ShieldRefreshingPowerUpData> + RefreshingPowerUp> ShieldRefreshingPowerUp for  T {

}