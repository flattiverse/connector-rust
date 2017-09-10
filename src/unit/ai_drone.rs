
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::AiUnit;
use unit::UnitData;
use unit::AiUnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

impl_downcast!(AiDrone);
pub trait AiDrone : AiUnit {
    fn kind(&self) -> UnitKind {
        UnitKind::AiDrone
    }
}

pub struct AiDroneData {
    unit: AiUnitData,
}

impl AiDroneData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<AiDroneData, Error> {
        Ok(AiDroneData {
            unit: AiUnitData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement AiUnit
impl Borrow<AiUnitData> for AiDroneData {
    fn borrow(&self) -> &AiUnitData {
        &self.unit
    }
}
impl BorrowMut<AiUnitData> for AiDroneData {
    fn borrow_mut(&mut self) -> &mut AiUnitData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for AiDroneData {
    fn borrow(&self) -> &UnitData {
        self.borrow()
    }
}
impl BorrowMut<UnitData> for AiDroneData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.borrow_mut()
    }
}

impl<T: 'static + Borrow<AiDroneData> + BorrowMut<AiDroneData> + AiUnit> AiDrone for  T {

}