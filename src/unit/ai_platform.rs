
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

downcast!(AiPlatform);
pub trait AiPlatform : AiUnit {
    fn kind(&self) -> UnitKind {
        UnitKind::AiPlatform
    }
}

pub struct AiPlatformData {
    unit: AiUnitData,
}

impl AiPlatformData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<AiPlatformData, Error> {
        Ok(AiPlatformData {
            unit: AiUnitData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement AiUnit
impl Borrow<AiUnitData> for AiPlatformData {
    fn borrow(&self) -> &AiUnitData {
        &self.unit
    }
}
impl BorrowMut<AiUnitData> for AiPlatformData {
    fn borrow_mut(&mut self) -> &mut AiUnitData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for AiPlatformData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for AiPlatformData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<AiPlatformData> + BorrowMut<AiPlatformData> + AiUnit> AiPlatform for  T {

}