
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::RefreshingPowerUp;
use unit::RefreshingPowerUpData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

impl_downcast!(HullRefreshingPowerUp);
pub trait HullRefreshingPowerUp : RefreshingPowerUp {
    fn kind(&self) -> RefreshingPowerUpKind {
        UnitKind::HullPowerUp
    }
}

pub struct HullRefreshingPowerUpData {
    unit: RefreshingPowerUpData,
}

impl HullRefreshingPowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<HullRefreshingPowerUpData, Error> {
        Ok(HullRefreshingPowerUpData {
            unit: RefreshingPowerUpData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement RefreshingPowerUp
impl Borrow<RefreshingPowerUpData> for HullRefreshingPowerUpData {
    fn borrow(&self) -> &RefreshingPowerUpData {
        &self.unit
    }
}
impl BorrowMut<RefreshingPowerUpData> for HullRefreshingPowerUpData {
    fn borrow_mut(&mut self) -> &mut RefreshingPowerUpData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<HullRefreshingPowerUpData> + BorrowMut<HullRefreshingPowerUpData> + RefreshingPowerUp> HullRefreshingPowerUp for  T {

}