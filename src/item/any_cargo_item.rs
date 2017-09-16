
use std::sync::Arc;
use std::ops::Deref;

use Error;
use Connector;
use net::BinaryReader;

use item::*;


#[derive(Clone)]
pub enum AnyCargoItem {
    Nebula       (Arc<NebulaCargoItem>),
    Crystal      (Arc<CrystalCargoItem>),
    MissionTarget(Arc<MissionTargetCargoItem>),
}

impl AnyCargoItem {
    pub fn from_reader(connector: &Arc<Connector>, reader: &mut BinaryReader, master: bool) -> Result<AnyCargoItem, Error> {
        Ok(match reader.read_byte()? {
            0x00 => AnyCargoItem::Nebula       (Arc::new(NebulaCargoItem       ::from_reader(connector, reader, master)?)),
            0x01 => AnyCargoItem::Crystal      (Arc::new(CrystalCargoItem      ::from_reader(connector, reader, master)?)),
            0x02 => AnyCargoItem::MissionTarget(Arc::new(MissionTargetCargoItem::from_reader(connector, reader, master)?)),
            id@_ => return Err(Error::InvalidCargoItem(id)),
        })
    }
}

impl Deref for AnyCargoItem {
    type Target = CargoItem;

    fn deref(&self) -> &Self::Target {
        match self {
            &AnyCargoItem::Nebula       (ref item) => item.deref(),
            &AnyCargoItem::Crystal      (ref item) => item.deref(),
            &AnyCargoItem::MissionTarget(ref item) => item.deref(),
        }
    }
}