use crate::units::uni::{Universe, UniverseId};
use std::collections::HashMap;

#[derive(Default)]
pub struct UniverseGroup {
    universes: HashMap<u16, Universe, nohash_hasher::BuildNoHashHasher<u16>>,
}

impl UniverseGroup {
    #[inline]
    pub fn add_universe(&mut self, id: UniverseId) {
        self.universes.insert(id.0, Universe::new(id));
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
