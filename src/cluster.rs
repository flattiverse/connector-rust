use crate::network::PacketReader;
use crate::GlaxyId;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, derive_more::From)]
pub struct ClusterId(pub(crate) u8);

#[derive(Debug)]
pub struct Cluster {
    id: ClusterId,
    galaxy: GlaxyId,
    name: String,
}

impl Cluster {
    #[inline]
    pub fn new(id: impl Into<ClusterId>, galaxy: GlaxyId, reader: &mut dyn PacketReader) -> Self {
        Self {
            id: id.into(),
            galaxy,
            name: reader.read_string(),
        }
    }

    #[inline]
    pub fn id(&self) -> ClusterId {
        self.id
    }

    #[inline]
    pub fn galaxy(&self) -> GlaxyId {
        self.galaxy
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}
