
use std::sync::Arc;
use std::sync::Weak;

use Error;
use Connector;
use net::BinaryReader;

use item::CargoItemKind;
use item::NebulaCargoItem;
use item::CrystalCargoItem;
use item::MissionTargetCargoItem;


pub trait CargoItem : Sync + Send {

    fn weight(&self) -> f32;

    fn kind(&self) -> CargoItemKind;
}

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

impl CargoItem for AnyCargoItem {
    fn weight(&self) -> f32 {
        match self {
            &AnyCargoItem::Nebula       (ref item) => item.weight(),
            &AnyCargoItem::Crystal      (ref item) => item.weight(),
            &AnyCargoItem::MissionTarget(ref item) => item.weight(),
        }
    }

    fn kind(&self) -> CargoItemKind {
        match self {
            &AnyCargoItem::Nebula       (ref item) => item.kind(),
            &AnyCargoItem::Crystal      (ref item) => item.kind(),
            &AnyCargoItem::MissionTarget(ref item) => item.kind(),
        }
    }
}



pub(crate) struct CargoItemData {
    pub(crate) connector: Weak<Connector>,
    pub(crate) weight: f32,
    pub(crate) master: bool
}

impl CargoItemData {
    pub(crate) fn new(connector: &Arc<Connector>, reader: &mut BinaryReader, master: bool) -> Result<CargoItemData, Error> {
        Ok(CargoItemData {
            master,
            connector: Arc::downgrade(connector),
            weight:    reader.read_single()?,
        })
    }
}