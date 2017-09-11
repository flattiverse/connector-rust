
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

downcast!(Switch);
pub trait Switch : Unit {

    fn color(&self) -> &Color;

    fn range(&self) -> f32;

    fn switch_time_cycle(&self) -> u16;

    fn switch_time_current(&self) -> u16;

    fn switched(&self) -> bool;

    fn kind(&self) -> UnitKind {
        UnitKind::Switch
    }
}

pub struct SwitchData {
    unit:   UnitData,
    color:  Color,
    range:              f32,
    switch_time_cycle:  u16,
    switch_time_current:u16,
    switched:           bool,
}

impl SwitchData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<SwitchData, Error> {
        Ok(SwitchData {
            unit:   UnitData::from_reader(connector, universe_group, packet, reader)?,
            color:  Color::from_rgb(
                reader.read_single()?,
                reader.read_single()?,
                reader.read_single()?,
            ),
            range:              reader.read_single()?,
            switch_time_cycle:  reader.read_u16()?,
            switch_time_current:reader.read_u16()?,
            switched:           reader.read_bool()?,
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for SwitchData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for SwitchData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<SwitchData> + BorrowMut<SwitchData> + Unit> Switch for  T {
    fn color(&self) -> &Color {
        &self.borrow().color
    }

    fn range(&self) -> f32 {
        self.borrow().range
    }

    fn switch_time_cycle(&self) -> u16 {
        self.borrow().switch_time_cycle
    }

    fn switch_time_current(&self) -> u16 {
        self.borrow().switch_time_current
    }

    fn switched(&self) -> bool {
        self.borrow().switched
    }
}