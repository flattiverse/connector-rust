
use std::sync::Arc;
use std::sync::Weak;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

use Error;
use Vector;
use Universe;
use Connector;
use UniverseGroup;
use unit::Unit;
use unit::UnitData;
use unit::UnitKind;
use net::Packet;
use net::BinaryReader;

downcast!(WormHole);
pub trait WormHole : Unit {

    fn destination(&self) -> &Option<Vector>;

    fn destination_universe(&self) -> &Weak<Universe>;
}

pub struct WormHoleData {
    unit:   UnitData,
    vector: Option<Vector>,
    dest:   Weak<Universe>,
}

impl WormHoleData {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<WormHoleData, Error> {
        let unit = UnitData::from_reader(connector, universe_group, packet, reader, UnitKind::WormHole)?;
        let vector;
        let dest;

        if reader.read_byte()? == 0x00 {
            vector = Some(Vector::from_reader(reader)?);
            dest   = Weak::default();

        } else {
            vector = None;
            let player = connector.player().upgrade().ok_or(Error::PlayerNotAvailable)?;
            let group  = player.universe_group().upgrade().ok_or(Error::PlayerNotAvailable)?;
            dest   = group.universe(reader.read_unsigned_byte()?)
        }

        Ok(WormHoleData {
            unit,
            vector,
            dest
        })
    }
}


// implicitly implement Unit
impl Borrow<UnitData> for WormHoleData {
    fn borrow(&self) -> &UnitData {
        &self.unit
    }
}
impl BorrowMut<UnitData> for WormHoleData {
    fn borrow_mut(&mut self) -> &mut UnitData {
        &mut self.unit
    }
}

impl<T: 'static + Borrow<WormHoleData> + BorrowMut<WormHoleData> + Unit> WormHole for  T {
    fn destination(&self) -> &Option<Vector> {
        &self.borrow().vector
    }

    fn destination_universe(&self) -> &Weak<Universe> {
        &self.borrow().dest
    }
}