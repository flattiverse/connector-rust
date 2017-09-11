
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

downcast!(StormWhirl);
pub trait StormWhirl : Unit {

    /// Remaining time for this [StormWhirl] to be active
    fn time(&self) -> u8;

    fn hull_damage(&self) -> f32;

    fn shield_damage(&self) -> f32;

    fn energy_damage(&self) -> f32;

    fn kind(&self) -> UnitKind {
        UnitKind::StormWhirl
    }
}

pub struct StormWhirlData {
    unit:           UnitData,
    time:           u8,
    hull_damage:    f32,
    shield_damage:  f32,
    energy_damage:  f32,
}

impl StormWhirlData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<StormWhirlData, Error> {
        Ok(StormWhirlData {
            unit:           UnitData::from_reader(connector, universe_group, packet, reader)?,
            time:           reader.read_unsigned_byte()?,
            hull_damage:    reader.read_single()?,
            shield_damage:  reader.read_single()?,
            energy_damage:  reader.read_single()?,
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for StormWhirlData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for StormWhirlData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<StormWhirlData> + BorrowMut<StormWhirlData> + Unit> StormWhirl for  T {
    fn time(&self) -> u8 {
        self.borrow().time
    }

    fn hull_damage(&self) -> f32 {
        self.borrow().hull_damage
    }

    fn shield_damage(&self) -> f32 {
        self.borrow().shield_damage
    }

    fn energy_damage(&self) -> f32 {
        self.borrow().energy_damage
    }
}