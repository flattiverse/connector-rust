
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Color;
use Error;
use Connector;
use UniverseGroup;
use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

downcast!(Nebula);
pub trait Nebula : Unit {

    fn color(&self) -> &Color;
}

pub struct NebulaData {
    unit: UnitData,
    color: Color
}

impl NebulaData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<NebulaData, Error> {
        Ok(NebulaData {
            unit:  UnitData::from_reader(connector, universe_group, packet, reader, UnitKind::Nebula)?,
            color: Color::from_hue(reader.read_single()?)?,
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for NebulaData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for NebulaData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<NebulaData> + BorrowMut<NebulaData> + Unit> Nebula for  T {
    fn color(&self) -> &Color {
        &self.borrow().color
    }
}