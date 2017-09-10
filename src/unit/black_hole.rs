
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use unit::GravityWell;
use net::Packet;
use net::BinaryReader;

impl_downcast!(BlackHole);
pub trait BlackHole : Unit {

    fn gravity_wells(&self) -> &Vec<GravityWell>;

    fn kind(&self) -> UnitKind {
        UnitKind::BlackHole
    }
}

pub struct BlackHoleData {
    unit:  UnitData,
    wells: Vec<GravityWell>
}

impl BlackHoleData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<BlackHoleData, Error> {
        Ok(BlackHoleData {
            unit:  UnitData::from_reader(connector, universe_group, packet, reader)?,
            wells: {
                let mut vec = Vec::new();
                let count = reader.read_unsigned_byte()?;
                for _ in 0..count {
                    vec.push(GravityWell::from_reader(reader)?);
                }
                vec
            },
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for BlackHoleData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for BlackHoleData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<BlackHoleData> + BorrowMut<BlackHoleData> + Unit> BlackHole for  T {
    fn gravity_wells(&self) -> &Vec<GravityWell> {
        &self.borrow().wells
    }
}