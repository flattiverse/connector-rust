
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

impl_downcast!(ShotProductionRefreshingPowerUp);
pub trait ShotProductionRefreshingPowerUp : RefreshingPowerUp {
    fn kind(&self) -> RefreshingPowerUpKind {
        UnitKind::ShotProductionPowerUp
    }
}

pub struct ShotProductionRefreshingPowerUpData {
    unit: RefreshingPowerUpData,
}

impl ShotProductionRefreshingPowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<ShotProductionRefreshingPowerUpData, Error> {
        Ok(ShotProductionRefreshingPowerUpData {
            unit: RefreshingPowerUpData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement RefreshingPowerUp
impl Borrow<RefreshingPowerUpData> for ShotProductionRefreshingPowerUpData {
    fn borrow(&self) -> &RefreshingPowerUpData {
        &self.unit
    }
}
impl BorrowMut<RefreshingPowerUpData> for ShotProductionRefreshingPowerUpData {
    fn borrow_mut(&mut self) -> &mut RefreshingPowerUpData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<ShotProductionRefreshingPowerUpData> + BorrowMut<ShotProductionRefreshingPowerUpData> + RefreshingPowerUp> ShotProductionRefreshingPowerUp for  T {

}