
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

impl_downcast!(ParticlesRefreshingPowerUp);
pub trait ParticlesRefreshingPowerUp : RefreshingPowerUp {
    fn kind(&self) -> UnitKind {
        UnitKind::ParticlesPowerUp
    }
}

pub struct ParticlesRefreshingPowerUpData {
    unit: RefreshingPowerUpData,
}

impl ParticlesRefreshingPowerUpData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<ParticlesRefreshingPowerUpData, Error> {
        Ok(ParticlesRefreshingPowerUpData {
            unit: RefreshingPowerUpData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement RefreshingPowerUp
impl Borrow<RefreshingPowerUpData> for ParticlesRefreshingPowerUpData {
    fn borrow(&self) -> &RefreshingPowerUpData {
        &self.unit
    }
}
impl BorrowMut<RefreshingPowerUpData> for ParticlesRefreshingPowerUpData {
    fn borrow_mut(&mut self) -> &mut RefreshingPowerUpData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for ParticlesRefreshingPowerUpData {
    fn borrow(&self) -> &UnitData {
        self.borrow()
    }
}
impl BorrowMut<UnitData> for ParticlesRefreshingPowerUpData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.borrow_mut()
    }
}

impl<T: 'static + Borrow<ParticlesRefreshingPowerUpData> + BorrowMut<ParticlesRefreshingPowerUpData> + RefreshingPowerUp> ParticlesRefreshingPowerUp for  T {

}