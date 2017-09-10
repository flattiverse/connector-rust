
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

impl_downcast!(AiBase);
pub trait AiBase : AiUnit {
    fn kind(&self) -> UnitKind {
        UnitKind::AiBase
    }
}

pub struct AiBaseData {
    unit: AiUnitData,
}

impl AiBaseData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<AiBaseData, Error> {
        Ok(AiBaseData {
            unit: AiUnitData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement AiUnit
impl Borrow<AiUnitData> for AiBaseData {
    fn borrow(&self) -> &AiUnitData {
        &self.unit
    }
}
impl BorrowMut<AiUnitData> for AiBaseData {
    fn borrow_mut(&mut self) -> &mut AiUnitData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for AiBaseData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for AiBaseData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<AiBaseData> + BorrowMut<AiBaseData> + AiUnit> AiBase for  T {

}