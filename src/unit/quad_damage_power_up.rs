
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::PowerUp;
use unit::UnitData;
use unit::PowerUpData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

impl_downcast!(QuadDamagePowerUp);
pub trait QuadDamagePowerUp : PowerUp {
    fn kind(&self) -> UnitKind {
        UnitKind::QuadDamagePowerUp
    }
}

pub struct QuadDamagePowerUpData {
    unit:   PowerUpData,
}

impl QuadDamagePowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<QuadDamagePowerUpData, Error> {
        Ok(QuadDamagePowerUpData {
            unit:   PowerUpData::from_reader(connector, universe_group, packet, reader)?,
        })
    }
}


// implicitly implement PowerUp
impl Borrow<PowerUpData> for QuadDamagePowerUpData {
    fn borrow(&self) -> &PowerUpData {
        &self.unit
    }
}
impl BorrowMut<PowerUpData> for QuadDamagePowerUpData {
    fn borrow_mut(&mut self) -> &mut PowerUpData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for QuadDamagePowerUpData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for QuadDamagePowerUpData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<QuadDamagePowerUpData> + BorrowMut<QuadDamagePowerUpData> + PowerUp> QuadDamagePowerUp for  T {

}