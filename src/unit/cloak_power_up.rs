
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::PowerUp;
use unit::PowerUpData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

impl_downcast!(CloakPowerUp);
pub trait CloakPowerUp : PowerUp {
    fn kind(&self) -> UnitKind {
        UnitKind::CloakPowerUp
    }
}

pub struct CloakPowerUpData {
    unit:   PowerUpData,
}

impl CloakPowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<CloakPowerUpData, Error> {
        Ok(CloakPowerUpData {
            unit:   PowerUpData::from_reader(connector, universe_group, packet, reader)?,
        })
    }
}


// implicitly implement PowerUp
impl Borrow<PowerUpData> for CloakPowerUpData {
    fn borrow(&self) -> &PowerUpData {
        &self.unit
    }
}
impl BorrowMut<PowerUpData> for CloakPowerUpData {
    fn borrow_mut(&mut self) -> &mut PowerUpData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<CloakPowerUpData> + BorrowMut<CloakPowerUpData> + PowerUp> CloakPowerUp for  T {

}