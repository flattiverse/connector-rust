
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

impl_downcast!(SpaceJellyFish);
pub trait SpaceJellyFish : Unit {

    fn hull(&self) -> f32;

    fn hull_max(&self) -> f32;

    fn hull_armor(&self) -> f32;


    fn kind(&self) -> UnitKind {
        UnitKind::SpaceJellyFish
    }
}

pub struct SpaceJellyFishData {
    unit: UnitData,
    hull:       f32,
    hull_max:   f32,
    hull_armor: f32,
}

impl SpaceJellyFishData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<SpaceJellyFishData, Error> {
        Ok(SpaceJellyFishData {
            unit:       UnitData::from_reader(connector, universe_group, packet, reader)?,
            hull:       reader.read_single()?,
            hull_max:   reader.read_single()?,
            hull_armor: reader.read_single()?,
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for SpaceJellyFishData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for SpaceJellyFishData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<SpaceJellyFishData> + BorrowMut<SpaceJellyFishData> + Unit> SpaceJellyFish for  T {
    fn hull(&self) -> f32 {
        self.borrow().hull
    }

    fn hull_max(&self) -> f32 {
        self.borrow().hull_max
    }

    fn hull_armor(&self) -> f32 {
        self.borrow().hull_armor
    }
}