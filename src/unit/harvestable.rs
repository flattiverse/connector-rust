use crate::network::PacketReader;
use crate::unit::sub_components::HarvestableSection;
use arc_swap::ArcSwap;
use std::sync::Arc;

#[derive(Debug)]
pub struct Harvestable {
    pub(crate) sections: ArcSwap<Vec<HarvestableSection>>,
}

impl Harvestable {
    pub(crate) fn new(reader: &mut dyn PacketReader) -> Self {
        Self {
            sections: ArcSwap::new(Arc::new(
                (0..reader.read_byte())
                    .map(|_| HarvestableSection::default().with_read(reader))
                    .collect(),
            )),
        }
    }

    pub(crate) fn update(&self, reader: &mut dyn PacketReader) {
        self.sections.store(Arc::new(
            (0..reader.read_byte())
                .map(|_| HarvestableSection::default().with_read(reader))
                .collect(),
        ));
    }
}
