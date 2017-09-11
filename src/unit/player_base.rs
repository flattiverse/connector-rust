
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

downcast!(PlayerBase);
pub trait PlayerBase : PlayerUnit {
    fn kind(&self) -> UnitKind {
        UnitKind::PlayerBase
    }
}

pub struct PlayerBaseData {
    unit: PlayerUnitData,
}

impl PlayerBaseData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerBaseData, Error> {
        Ok(PlayerBaseData {
            unit: PlayerUnitData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement PlayerUnit
impl Borrow<PlayerUnitData> for PlayerBaseData {
    fn borrow(&self) -> &PlayerUnitData {
        &self.unit
    }
}
impl BorrowMut<PlayerUnitData> for PlayerBaseData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for PlayerBaseData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for PlayerBaseData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<PlayerBaseData> + BorrowMut<PlayerBaseData> + PlayerUnit> PlayerBase for  T {

}