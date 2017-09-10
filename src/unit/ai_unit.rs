
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

impl_downcast!(AiUnit);
pub trait AiUnit : Unit {

    fn hull(&self) -> f32;

    fn hull_max(&self) -> f32;

    fn hull_armor(&self) -> f32;

    fn shield(&self) -> f32;

    fn shield_max(&self) -> f32;

    fn shield_armor(&self) -> f32;
}

pub struct AiUnitData {
    unit: UnitData,
    hull:           f32,
    hull_max:       f32,
    hull_armor:     f32,
    shield:         f32,
    shield_max:     f32,
    shield_armor:   f32,
}

impl AiUnitData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<AiUnitData, Error> {
        Ok(AiUnitData {
            unit: UnitData::from_reader(connector, universe_group, packet, reader)?,
            hull:           reader.read_single()?,
            hull_max:       reader.read_single()?,
            hull_armor:     reader.read_single()?,
            shield:         reader.read_single()?,
            shield_max:     reader.read_single()?,
            shield_armor:   reader.read_single()?,
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for AiUnitData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for AiUnitData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<AiUnitData> + BorrowMut<AiUnitData> + Unit> AiUnit for  T {
    fn hull(&self) -> f32 {
        self.borrow().hull
    }

    fn hull_max(&self) -> f32 {
        self.borrow().hull_max
    }

    fn hull_armor(&self) -> f32 {
        self.borrow().hull_armor
    }

    fn shield(&self) -> f32 {
        self.borrow().shield
    }

    fn shield_max(&self) -> f32 {
        self.borrow().shield_max
    }

    fn shield_armor(&self) -> f32 {
        self.borrow().shield_armor
    }
}