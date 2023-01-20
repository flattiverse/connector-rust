use crate::con::handle::ConnectionHandle;
use crate::units::uni::{Universe, UniverseId};
use std::collections::HashMap;
use std::sync::Arc;

pub struct UniverseGroup {
    connection: Arc<ConnectionHandle>,
    universes: HashMap<u16, Universe, nohash_hasher::BuildNoHashHasher<u16>>,
}

impl UniverseGroup {
    pub fn new(connection: Arc<ConnectionHandle>) -> Self {
        Self {
            connection,
            universes: HashMap::default(),
        }
    }

    #[inline]
    pub fn add_universe(&mut self, id: UniverseId) {
        self.universes
            .insert(id.0, Universe::new(id, Arc::clone(&self.connection)));
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=&Universe> {
        self.universes.values()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.universes.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.universes.is_empty()
    }
}
