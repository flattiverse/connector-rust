
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

impl_downcast!(DoubleDamagePowerUp);
pub trait DoubleDamagePowerUp : PowerUp {
    fn kind(&self) -> UnitKind {
        UnitKind::DoubleDamagePowerUp
    }
}

pub struct DoubleDamagePowerUpData {
    unit:   PowerUpData,
}

impl DoubleDamagePowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<DoubleDamagePowerUpData, Error> {
        Ok(DoubleDamagePowerUpData {
            unit:   PowerUpData::from_reader(connector, universe_group, packet, reader)?,
        })
    }
}


// implicitly implement PowerUp
impl Borrow<PowerUpData> for DoubleDamagePowerUpData {
    fn borrow(&self) -> &PowerUpData {
        &self.unit
    }
}
impl BorrowMut<PowerUpData> for DoubleDamagePowerUpData {
    fn borrow_mut(&mut self) -> &mut PowerUpData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for DoubleDamagePowerUpData {
    fn borrow(&self) -> &UnitData {
        self.borrow()
    }
}
impl BorrowMut<UnitData> for DoubleDamagePowerUpData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.borrow_mut()
    }
}

impl<T: 'static + Borrow<DoubleDamagePowerUpData> + BorrowMut<DoubleDamagePowerUpData> + PowerUp> DoubleDamagePowerUp for  T {

}