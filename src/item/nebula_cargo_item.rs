
use std::sync::Arc;

use crate::Color;
use crate::Error;
use crate::Connector;
use crate::item::CargoItem;
use crate::item::CargoItemData;
use crate::item::CargoItemKind;
use crate::net::BinaryReader;

pub struct NebulaCargoItem {
    cargo: CargoItemData,
    color: Color,
}

impl NebulaCargoItem {
    pub fn from_reader(connector: &Arc<Connector>, reader: &mut BinaryReader, master: bool) -> Result<NebulaCargoItem, Error> {
        Ok(NebulaCargoItem {
            cargo: CargoItemData::new(connector, reader, master)?,
            color: Color::from_hue(reader.read_single()?)?
        })
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn red(&self) -> f32 {
        self.color.red
    }

    fn green(&self) -> f32 {
        self.color.green
    }

    fn blue(&self) -> f32 {
        self.color.blue
    }

    fn alpha(&self) -> f32 {
        self.color.alpha
    }
}

impl CargoItem for NebulaCargoItem {
    fn weight(&self) -> f32 {
        self.cargo.weight()
    }

    fn kind(&self) -> CargoItemKind {
        CargoItemKind::Nebula
    }
}