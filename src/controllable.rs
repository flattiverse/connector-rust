use crate::hierarchy::GalaxyId;
use crate::network::{ConnectionHandle, PacketReader};
use crate::Indexer;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ControllableId(pub(crate) u8);

impl Indexer for ControllableId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Controllable {
    active: bool,
    galaxy: GalaxyId,
    id: ControllableId,
    name: String,
    connection: ConnectionHandle,
}

impl Controllable {
    pub fn new(
        galaxy: GalaxyId,
        id: ControllableId,
        reader: &mut dyn PacketReader,
        connection: ConnectionHandle,
    ) -> Self {
        Self {
            active: true,
            galaxy,
            id,
            name: reader.read_string(),
            connection,
        }
    }

    pub(crate) fn deactivate(&mut self) {
        self.active = false;
    }

    pub(crate) fn update(&mut self, reader: &mut dyn PacketReader) {
        todo!()
    }

    pub(crate) fn dynamic_update(&mut self, reader: &mut dyn PacketReader) {
        todo!()
    }
}
