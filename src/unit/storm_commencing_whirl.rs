
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

downcast!(StormCommencingWhirl);
pub trait StormCommencingWhirl : Unit {

    /// Time until this [StormCommencingWhirl] to becomes a [StormWhirl]
    fn time(&self) -> u8;

    /// Time the [StormWhirl] will be active
    fn active_time(&self) -> u8;

    fn configured_gravity(&self) -> f32;

    fn hull_damage(&self) -> f32;

    fn shield_damage(&self) -> f32;

    fn energy_damage(&self) -> f32;

    fn kind(&self) -> UnitKind {
        UnitKind::StormCommencingWhirl
    }
}

pub struct StormCommencingWhirlData {
    unit:           UnitData,
    time:           u8,
    active_time:    u8,
    configured_grav:f32,
    hull_damage:    f32,
    shield_damage:  f32,
    energy_damage:  f32,
}

impl StormCommencingWhirlData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<StormCommencingWhirlData, Error> {
        Ok(StormCommencingWhirlData {
            unit:           UnitData::from_reader(connector, universe_group, packet, reader)?,
            time:           reader.read_unsigned_byte()?,
            active_time:    reader.read_unsigned_byte()?,
            configured_grav:reader.read_single()?,
            hull_damage:    reader.read_single()?,
            shield_damage:  reader.read_single()?,
            energy_damage:  reader.read_single()?,
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for StormCommencingWhirlData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for StormCommencingWhirlData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<StormCommencingWhirlData> + BorrowMut<StormCommencingWhirlData> + Unit> StormCommencingWhirl for  T {
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
    fn active_time(&self) -> u8 {
        self.borrow().active_time
    }

    fn configured_gravity(&self) -> f32 {
        self.borrow().configured_grav
    }
}