
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

downcast!(AiProbe);
pub trait AiProbe : AiUnit {

}

pub struct AiProbeData {
    unit: AiUnitData,
}

impl AiProbeData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<AiProbeData, Error> {
        Ok(AiProbeData {
            unit: AiUnitData::from_reader(connector, universe_group, packet, reader, UnitKind::AiProbe)?
        })
    }
}


// implicitly implement AiUnit
impl Borrow<AiUnitData> for AiProbeData {
    fn borrow(&self) -> &AiUnitData {
        &self.unit
    }
}
impl BorrowMut<AiUnitData> for AiProbeData {
    fn borrow_mut(&mut self) -> &mut AiUnitData {
        &mut self.unit
    }
}
impl Borrow<UnitData> for AiProbeData {
    fn borrow(&self) -> &UnitData {
        self.unit.borrow()
    }
}
impl BorrowMut<UnitData> for AiProbeData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        self.unit.borrow_mut()
    }
}

impl<T: 'static + Borrow<AiProbeData> + BorrowMut<AiProbeData> + AiUnit> AiProbe for  T {

}