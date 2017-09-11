
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Color;
use Connector;
use UniverseGroup;
use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

downcast!(Gate);
pub trait Gate : Unit {

    fn color(&self) -> &Color;

    fn switched(&self) -> bool;

    fn kind(&self) -> UnitKind {
        UnitKind::Gate
    }
}

pub struct GateData {
    unit:       UnitData,
    color:      Color,
    switched:   bool,
}

impl GateData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<GateData, Error> {
        Ok(GateData {
            unit:   UnitData::from_reader(connector, universe_group, packet, reader)?,
            color:  Color::from_rgb(
                reader.read_single()?,
                reader.read_single()?,
                reader.read_single()?,
            ),
            switched: reader.read_bool()?,
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for GateData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for GateData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<GateData> + BorrowMut<GateData> + Unit> Gate for  T {
    fn color(&self) -> &Color {
        &self.borrow().color
    }

    fn switched(&self) -> bool {
        self.borrow().switched
    }
}