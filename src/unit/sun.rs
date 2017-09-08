
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Connector;
use UniverseGroup;
use unit::Unit;
use unit::Corona;
use unit::UnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;


pub trait Sun : Unit {
    fn coronas(&self) -> &Vec<Corona>;

    fn kind(&self) -> UnitKind {
        UnitKind::Sun
    }
}

pub struct SunData {
    unit: UnitData,
    coronas: Vec<Corona>
}

impl SunData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<SunData, Error> {
        let unit = UnitData::from_reader(connector, universe_group, packet, reader)?;
        let count = reader.read_unsigned_byte()?;
        let mut coronas = Vec::with_capacity(count as usize);

        for _ in 0..count {
            coronas.push(Corona::from_reader(reader)?);
        }

        Ok(SunData {
            unit,
            coronas
        })
    }
}

// implicitly implement Unit
impl Borrow<UnitData> for SunData {
    fn borrow(& self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for SunData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: Borrow<SunData> + BorrowMut<SunData> + Unit> Sun for T {
    fn coronas(&self) -> &Vec<Corona> {
        &self.borrow().coronas
    }
}