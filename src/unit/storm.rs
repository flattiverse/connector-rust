
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

downcast!(Storm);
pub trait Storm : Unit {

    fn max_whirls(&self) -> u8;

    fn child_min_announcement_time(&self) -> u8;

    fn child_max_announcement_time(&self) -> u8;

    fn child_min_active_time(&self) -> u8;

    fn child_max_active_time(&self) -> u8;

    fn child_min_size(&self) -> f32;

    fn child_max_size(&self) -> f32;

    fn child_min_speed(&self) -> f32;

    fn child_max_speed(&self) -> f32;

    fn child_min_gravity(&self) -> f32;

    fn child_max_gravity(&self) -> f32;

    fn min_hull_damage(&self) -> f32;

    fn max_hull_damage(&self) -> f32;

    fn min_shield_damage(&self) -> f32;

    fn max_shield_damage(&self) -> f32;

    fn min_energy_damage(&self) -> f32;

    fn max_energy_damage(&self) -> f32;
}

pub struct StormData {
    unit: UnitData,
    max_whirls: u8,
    child_min_announcement_time:    u8,
    child_max_announcement_time:    u8,
    child_min_active_time:          u8,
    child_max_active_time:          u8,
    child_min_size:     f32,
    child_max_size:     f32,
    child_min_speed:    f32,
    child_max_speed:    f32,
    child_min_gravity:  f32,
    child_max_gravity:  f32,
    min_hull_damage:    f32,
    max_hull_damage:    f32,
    min_shield_damage:  f32,
    max_shield_damage:  f32,
    min_energy_damage:  f32,
    max_energy_damage:  f32,
}

impl StormData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<StormData, Error> {
        Ok(StormData {
            unit: UnitData::from_reader(connector, universe_group, packet, reader, UnitKind::Storm)?,
            max_whirls:                     reader.read_unsigned_byte()?,
            child_min_announcement_time:    reader.read_unsigned_byte()?,
            child_max_announcement_time:    reader.read_unsigned_byte()?,
            child_min_active_time:          reader.read_unsigned_byte()?,
            child_max_active_time:          reader.read_unsigned_byte()?,
            child_min_size:                 reader.read_single()?,
            child_max_size:                 reader.read_single()?,
            child_min_speed:                reader.read_single()?,
            child_max_speed:                reader.read_single()?,
            child_min_gravity:              reader.read_single()?,
            child_max_gravity:              reader.read_single()?,
            min_hull_damage:                reader.read_single()?,
            max_hull_damage:                reader.read_single()?,
            min_shield_damage:              reader.read_single()?,
            max_shield_damage:              reader.read_single()?,
            min_energy_damage:              reader.read_single()?,
            max_energy_damage:              reader.read_single()?,
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for StormData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for StormData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<StormData> + BorrowMut<StormData> + Unit> Storm for  T {
    fn max_whirls(&self) -> u8 {
        self.borrow().max_whirls
    }

    fn child_min_announcement_time(&self) -> u8 {
        self.borrow().child_min_announcement_time
    }

    fn child_max_announcement_time(&self) -> u8 {
        self.borrow().child_max_announcement_time
    }

    fn child_min_active_time(&self) -> u8 {
        self.borrow().child_min_active_time
    }

    fn child_max_active_time(&self) -> u8 {
        self.borrow().child_max_active_time
    }

    fn child_min_size(&self) -> f32 {
        self.borrow().child_min_size
    }

    fn child_max_size(&self) -> f32 {
        self.borrow().child_max_size
    }

    fn child_min_speed(&self) -> f32 {
        self.borrow().child_min_speed
    }

    fn child_max_speed(&self) -> f32 {
        self.borrow().child_max_speed
    }

    fn child_min_gravity(&self) -> f32 {
        self.borrow().child_min_gravity
    }

    fn child_max_gravity(&self) -> f32 {
        self.borrow().child_max_gravity
    }

    fn min_hull_damage(&self) -> f32 {
        self.borrow().min_hull_damage
    }

    fn max_hull_damage(&self) -> f32 {
        self.borrow().max_hull_damage
    }

    fn min_shield_damage(&self) -> f32 {
        self.borrow().min_shield_damage
    }

    fn max_shield_damage(&self) -> f32 {
        self.borrow().max_shield_damage
    }

    fn min_energy_damage(&self) -> f32 {
        self.borrow().min_energy_damage
    }

    fn max_energy_damage(&self) -> f32 {
        self.borrow().max_energy_damage
    }
}