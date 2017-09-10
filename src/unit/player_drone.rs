
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

impl_downcast!(PlayerDrone);
pub trait PlayerDrone : PlayerUnit {
    fn kind(&self) -> UnitKind {
        UnitKind::PlayerDrone
    }
}

pub struct PlayerDroneData {
    unit: PlayerUnitData,
}

impl PlayerDroneData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerDroneData, Error> {
        Ok(PlayerDroneData {
            unit: PlayerUnitData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement PlayerUnit
impl Borrow<PlayerUnitData> for PlayerDroneData {
    fn borrow(&self) -> &PlayerUnitData {
        &self.unit
    }
}
impl BorrowMut<PlayerUnitData> for PlayerDroneData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for PlayerDroneData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for PlayerDroneData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<PlayerDroneData> + BorrowMut<PlayerDroneData> + PlayerUnit> PlayerDrone for  T {

}