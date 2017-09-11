
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

downcast!(SpaceJellyFishSlime);
pub trait SpaceJellyFishSlime : Unit {

    fn hull(&self) -> f32;

    fn hull_max(&self) -> f32;

    fn hull_armor(&self) -> f32;

    /// The amount of damage this [SpaceJellyFishSlime] can deal
    fn damage(&self) -> f32;

    /// The time until this [SpaceJellyFishSlime] dissolves
    fn time(&self) -> u16;

    fn kind(&self) -> UnitKind {
        UnitKind::SpaceJellyFishSlime
    }
}

pub struct SpaceJellyFishSlimeData {
    unit: UnitData,
    hull:       f32,
    hull_max:   f32,
    hull_armor: f32,
    damage:     f32,
    time:       u16,
}

impl SpaceJellyFishSlimeData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<SpaceJellyFishSlimeData, Error> {
        Ok(SpaceJellyFishSlimeData {
            unit:       UnitData::from_reader(connector, universe_group, packet, reader)?,
            hull:       reader.read_single()?,
            hull_max:   reader.read_single()?,
            hull_armor: reader.read_single()?,
            damage:     reader.read_single()?,
            time:       reader.read_u16()?,
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for SpaceJellyFishSlimeData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for SpaceJellyFishSlimeData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<SpaceJellyFishSlimeData> + BorrowMut<SpaceJellyFishSlimeData> + Unit> SpaceJellyFishSlime for  T {
    fn hull(&self) -> f32 {
        self.borrow().hull
    }

    fn hull_max(&self) -> f32 {
        self.borrow().hull_max
    }

    fn hull_armor(&self) -> f32 {
        self.borrow().hull_armor
    }

    fn damage(&self) -> f32 {
        self.borrow().damage
    }

    fn time(&self) -> u16 {
        self.borrow().time
    }
}