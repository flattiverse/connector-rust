
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::AiUnit;
use unit::AiUnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

impl_downcast!(AiShip);
pub trait AiShip : AiUnit {
    fn kind(&self) -> UnitKind {
        UnitKind::AiShip
    }
}

pub struct AiShipData {
    unit: AiUnitData,
}

impl AiShipData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<AiShipData, Error> {
        Ok(AiShipData {
            unit: AiUnitData::from_reader(connector, universe_group, packet, reader)?
        })
    }
}


// implicitly implement AiUnit
impl Borrow<AiUnitData> for AiShipData {
    fn borrow(&self) -> &AiUnitData {
        &self.unit
    }
}
impl BorrowMut<AiUnitData> for AiShipData {
    fn borrow_mut(&mut self) -> &mut AiUnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<AiShipData> + BorrowMut<AiShipData> + AiUnit> AiShip for  T {

}