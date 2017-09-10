
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Task;
use Error;
use Connector;
use UniverseGroup;
use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

impl_downcast!(Buoy);
pub trait Buoy : Unit {
    fn message(&self) -> &str;

    fn kind(&self) -> UnitKind {
        UnitKind::Buoy
    }
}

pub struct BuoyData {
    unit: UnitData,
    message: String,
}

impl BuoyData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<BuoyData, Error> {
        Ok(BuoyData {
            unit: UnitData::from_reader(connector, universe_group, packet, reader)?,
            message: reader.read_string()?,
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for BuoyData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for BuoyData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<BuoyData> + BorrowMut<BuoyData> + Unit> Buoy for  T {
    fn message(&self) -> &str {
        if let Some(connector) = self.connector().upgrade() {
            connector.register_task_quitely_if_unknown(Task::UsedBuoyMessage);
        }
        &self.borrow().message
    }
}