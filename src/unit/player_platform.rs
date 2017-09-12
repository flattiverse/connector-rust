
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::UnitData;
use unit::PlayerUnit;
use unit::PlayerUnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

downcast!(PlayerPlatform);
pub trait PlayerPlatform : PlayerUnit {

}

pub struct PlayerPlatformData {
    unit: PlayerUnitData,
}

impl PlayerPlatformData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerPlatformData, Error> {
        Ok(PlayerPlatformData {
            unit: PlayerUnitData::from_reader(connector, universe_group, packet, reader, UnitKind::PlayerPlatform)?
        })
    }
}


// implicitly implement PlayerUnit
impl Borrow<PlayerUnitData> for PlayerPlatformData {
    fn borrow(&self) -> &PlayerUnitData {
        &self.unit
    }
}
impl BorrowMut<PlayerUnitData> for PlayerPlatformData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for PlayerPlatformData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for PlayerPlatformData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<PlayerPlatformData> + BorrowMut<PlayerPlatformData> + PlayerUnit> PlayerPlatform for  T {

}