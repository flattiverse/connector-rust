
use std::sync::Arc;

use crate::Error;
use crate::Connector;
use crate::item::CargoItem;
use crate::item::CargoItemData;
use crate::item::CargoItemKind;
use crate::net::BinaryReader;


pub struct MissionTargetCargoItem {
    cargo: CargoItemData,
}

impl MissionTargetCargoItem {
    pub fn from_reader(connector: &Arc<Connector>, reader: &mut BinaryReader, master: bool) -> Result<MissionTargetCargoItem, Error> {
        Ok(MissionTargetCargoItem {
            cargo: CargoItemData::new(connector, reader, master)?,
        })
    }
}

impl CargoItem for MissionTargetCargoItem {
    fn weight(&self) -> f32 {
        self.cargo.weight()
    }

    fn kind(&self) -> CargoItemKind {
        CargoItemKind::MissionTarget
    }
}