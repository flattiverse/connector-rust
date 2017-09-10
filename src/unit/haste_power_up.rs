
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::UnitData;
use unit::PowerUp;
use unit::PowerUpData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

impl_downcast!(HastePowerUp);
pub trait HastePowerUp : PowerUp {
    fn kind(&self) -> UnitKind {
        UnitKind::HastePowerUp
    }
}

pub struct HastePowerUpData {
    unit:   PowerUpData,
}

impl HastePowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<HastePowerUpData, Error> {
        Ok(HastePowerUpData {
            unit:   PowerUpData::from_reader(connector, universe_group, packet, reader)?,
        })
    }
}


// implicitly implement PowerUp
impl Borrow<PowerUpData> for HastePowerUpData {
    fn borrow(&self) -> &PowerUpData {
        &self.unit
    }
}
impl BorrowMut<PowerUpData> for HastePowerUpData {
    fn borrow_mut(&mut self) -> &mut PowerUpData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for HastePowerUpData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for HastePowerUpData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<HastePowerUpData> + BorrowMut<HastePowerUpData> + PowerUp> HastePowerUp for  T {

}