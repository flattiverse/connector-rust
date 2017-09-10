
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

impl_downcast!(PlayerProbe);
pub trait PlayerProbe : PlayerUnit {
    fn kind(&self) -> UnitKind {
        UnitKind::PlayerProbe
    }
}

pub struct PlayerProbeData {
    unit: PlayerUnitData,
}

impl PlayerProbeData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<PlayerProbeData, Error> {
        Ok(PlayerProbeData {
            unit: PlayerUnitData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement PlayerUnit
impl Borrow<PlayerUnitData> for PlayerProbeData {
    fn borrow(&self) -> &PlayerUnitData {
        &self.unit
    }
}
impl BorrowMut<PlayerUnitData> for PlayerProbeData {
    fn borrow_mut(&mut self) -> &mut PlayerUnitData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for PlayerProbeData {
    fn borrow(&self) -> &UnitData {
        self.borrow()
    }
}
impl BorrowMut<UnitData> for PlayerProbeData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.borrow_mut()
    }
}

impl<T: 'static + Borrow<PlayerProbeData> + BorrowMut<PlayerProbeData> + PlayerUnit> PlayerProbe for  T {

}