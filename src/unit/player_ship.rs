
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

impl_downcast!(PlayerShip);
pub trait PlayerShip : PlayerUnit {
    fn kind(&self) -> UnitKind {
        UnitKind::PlayerShip
    }
}

pub struct PlayerShipData {
    unit: PlayerUnitData,
}

impl PlayerShipData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerShipData, Error> {
        Ok(PlayerShipData {
            unit: PlayerUnitData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement PlayerUnit
impl Borrow<PlayerUnitData> for PlayerShipData {
    fn borrow(&self) -> &PlayerUnitData {
        &self.unit
    }
}
impl BorrowMut<PlayerUnitData> for PlayerShipData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for PlayerShipData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for PlayerShipData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: Borrow<PlayerShipData> + BorrowMut<PlayerShipData> + PlayerUnit> PlayerShip for  T {

}