use crate::atomics::Atomic;
use crate::hierarchy::{Galaxy, ShipDesignId, ShipUpgradeId};
use crate::network::PacketReader;
use crate::{Identifiable, Indexer, NamedUnit, Player};
use std::sync::Arc;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ControllableInfoId(pub(crate) u8);

impl Indexer for ControllableInfoId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct ControllableInfo {
    active: Atomic<bool>,
    galaxy: Arc<Galaxy>,
    id: ControllableInfoId,
    name: String,
    reduced: bool,
    ship_design: ShipDesignId,
    player: Arc<Player>,
    upgrades: Box<[ShipUpgradeId]>,

    hull: Atomic<f64>,
    hull_max: f64,

    shields: Atomic<f64>,
    shields_max: f64,

    energy: Atomic<f64>,
    energy_max: f64,

    ion: Atomic<f64>,
    ion_max: f64,
}

impl ControllableInfo {
    pub fn new(
        galaxy: Arc<Galaxy>,
        id: ControllableInfoId,
        player: Arc<Player>,
        reader: &mut dyn PacketReader,
        reduced: bool,
    ) -> Self {
        Self {
            active: Atomic::from(true),
            galaxy,
            id,
            player,
            reduced,

            name: reader.read_string(),

            ship_design: ShipDesignId(reader.read_byte()),
            upgrades: reader
                .read_bytes(32)
                .into_iter()
                .map(ShipUpgradeId)
                .collect::<Vec<_>>()
                .into_boxed_slice(),

            hull_max: reader.read_double(),
            shields_max: reader.read_double(),
            energy_max: reader.read_double(),
            ion_max: reader.read_double(),

            hull: Atomic::from(if reduced { 0.0 } else { reader.read_double() }),
            shields: Atomic::from(if reduced { 0.0 } else { reader.read_double() }),
            energy: Atomic::from(if reduced { 0.0 } else { reader.read_double() }),
            ion: Atomic::from(if reduced { 0.0 } else { reader.read_double() }),
        }
    }

    pub(crate) fn deactivate(&self) {
        self.active.store(false);
    }

    pub(crate) fn dynamic_update(&self, reader: &mut dyn PacketReader, reduced: bool) {
        if reduced {
            let _ = reader.read_boolean();
        } else {
            self.hull.read(reader);
            self.shields.read(reader);
            self.energy.read(reader);
            self.ion.read(reader);
        }
    }

    #[inline]
    pub fn galaxy(&self) -> &Arc<Galaxy> {
        &self.galaxy
    }

    #[inline]
    pub fn id(&self) -> ControllableInfoId {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn reduced(&self) -> bool {
        self.reduced
    }

    #[inline]
    pub fn ship_design(&self) -> ShipDesignId {
        self.ship_design
    }

    #[inline]
    pub fn player(&self) -> &Arc<Player> {
        &self.player
    }

    #[inline]
    pub fn upgrade(&self) -> &[ShipUpgradeId] {
        &self.upgrades[..]
    }

    #[inline]
    pub fn hull(&self) -> f64 {
        self.hull.load()
    }

    #[inline]
    pub fn hull_max(&self) -> f64 {
        self.hull_max
    }

    #[inline]
    pub fn shields(&self) -> f64 {
        self.shields.load()
    }

    #[inline]
    pub fn shields_max(&self) -> f64 {
        self.shields_max
    }

    #[inline]
    pub fn energy(&self) -> f64 {
        self.energy.load()
    }

    #[inline]
    pub fn energy_max(&self) -> f64 {
        self.energy_max
    }

    #[inline]
    pub fn ion(&self) -> f64 {
        self.ion.load()
    }

    #[inline]
    pub fn ion_max(&self) -> f64 {
        self.ion_max
    }

    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }
}

impl Identifiable<ControllableInfoId> for ControllableInfo {
    #[inline]
    fn id(&self) -> ControllableInfoId {
        self.id
    }
}

impl NamedUnit for ControllableInfo {
    #[inline]
    fn name(&self) -> &str {
        ControllableInfo::name(self)
    }
}
