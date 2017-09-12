
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::PowerUp;
use unit::UnitData;
use unit::UnitKind;
use unit::PowerUpData;
use net::Packet;
use net::BinaryReader;

downcast!(RefreshingPowerUp);
pub trait RefreshingPowerUp : PowerUp {
    fn amount(&self) -> f32;
}

pub struct RefreshingPowerUpData {
    unit:   PowerUpData,
    amount: f32,
}

impl RefreshingPowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader, kind: UnitKind) -> Result<RefreshingPowerUpData, Error> {
        Ok(RefreshingPowerUpData {
            unit:   PowerUpData::from_reader(connector, universe_group, packet, reader, kind)?,
            amount: reader.read_single()?,
        })
    }
}


// implicitly implement PowerUp
impl Borrow<PowerUpData> for RefreshingPowerUpData {
    fn borrow(&self) -> &PowerUpData {
        &self.unit
    }
}
impl BorrowMut<PowerUpData> for RefreshingPowerUpData {
    fn borrow_mut(&mut self) -> &mut PowerUpData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for RefreshingPowerUpData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for RefreshingPowerUpData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<RefreshingPowerUpData> + BorrowMut<RefreshingPowerUpData> + PowerUp> RefreshingPowerUp for  T {
    fn amount(&self) -> f32 {
        self.borrow().amount
    }
}