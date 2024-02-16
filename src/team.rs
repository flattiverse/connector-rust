use crate::network::PacketReader;
use crate::{Indexer, NamedUnit};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, derive_more::From)]
pub struct TeamId(u8);

impl Indexer for TeamId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Team {
    id: TeamId,
    name: String,
    red: u8,
    green: u8,
    blue: u8,
}

impl Team {
    #[inline]
    pub fn new(id: impl Into<TeamId>, reader: &mut dyn PacketReader) -> Self {
        Self {
            id: id.into(),
            name: reader.read_string(),
            red: reader.read_byte(),
            green: reader.read_byte(),
            blue: reader.read_byte(),
        }
    }

    #[inline]
    pub fn id(&self) -> TeamId {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &&self.name
    }

    #[inline]
    pub fn red(&self) -> u8 {
        self.red
    }

    #[inline]
    pub fn green(&self) -> u8 {
        self.green
    }

    #[inline]
    pub fn blue(&self) -> u8 {
        self.blue
    }
}

impl NamedUnit for Team {
    #[inline]
    fn name(&self) -> &str {
        Team::name(self)
    }
}
