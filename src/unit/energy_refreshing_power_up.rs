
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

downcast!(EnergyRefreshingPowerUp);
pub trait EnergyRefreshingPowerUp : RefreshingPowerUp {
    fn kind(&self) -> UnitKind {
        UnitKind::EnergyPowerUp
    }
}

pub struct EnergyRefreshingPowerUpData {
    unit: RefreshingPowerUpData,
}

impl EnergyRefreshingPowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<EnergyRefreshingPowerUpData, Error> {
        Ok(EnergyRefreshingPowerUpData {
            unit: RefreshingPowerUpData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement RefreshingPowerUp
impl Borrow<RefreshingPowerUpData> for EnergyRefreshingPowerUpData {
    fn borrow(&self) -> &RefreshingPowerUpData {
        &self.unit
    }
}
impl BorrowMut<RefreshingPowerUpData> for EnergyRefreshingPowerUpData {
    fn borrow_mut(&mut self) -> &mut RefreshingPowerUpData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for EnergyRefreshingPowerUpData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for EnergyRefreshingPowerUpData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<EnergyRefreshingPowerUpData> + BorrowMut<EnergyRefreshingPowerUpData> + RefreshingPowerUp> EnergyRefreshingPowerUp for  T {

}