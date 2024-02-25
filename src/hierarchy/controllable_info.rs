use crate::hierarchy::{GalaxyId, ShipDesignId, ShipUpgradeId};
use crate::network::PacketReader;
use crate::{Indexer, NamedUnit, PlayerId};

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
    active: bool,
    galaxy: GalaxyId,
    id: ControllableInfoId,
    name: String,
    reduced: bool,
    ship_design: ShipDesignId,
    player: PlayerId,
    upgrades: Box<[ShipUpgradeId]>,

    hull: f64,
    hull_max: f64,

    shields: f64,
    shields_max: f64,

    energy: f64,
    energy_max: f64,

    ion: f64,
    ion_max: f64,
}

impl ControllableInfo {
    pub fn new(
        galaxy: GalaxyId,
        id: ControllableInfoId,
        player: PlayerId,
        reader: &mut dyn PacketReader,
        reduced: bool,
    ) -> Self {
        Self {
            active: true,
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

            hull_max: reader.read_2u(10.0),
            shields_max: reader.read_2u(10.0),
            energy_max: reader.read_4u(10.0),
            ion_max: reader.read_2u(100.0),

            hull: if reduced { 0.0 } else { reader.read_2u(10.0) },
            shields: if reduced { 0.0 } else { reader.read_2u(10.0) },
            energy: if reduced { 0.0 } else { reader.read_4u(10.0) },
            ion: if reduced { 0.0 } else { reader.read_2u(100.0) },
        }
    }

    pub(crate) fn deactivate(&mut self) {
        self.active = false;
    }

    #[inline]
    pub fn galaxy(&self) -> GalaxyId {
        self.galaxy
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
    pub fn player(&self) -> PlayerId {
        self.player
    }

    #[inline]
    pub fn upgrade(&self) -> &[ShipUpgradeId] {
        &self.upgrades[..]
    }

    #[inline]
    pub fn hull(&self) -> f64 {
        self.hull
    }

    #[inline]
    pub fn hull_max(&self) -> f64 {
        self.hull_max
    }

    #[inline]
    pub fn shields(&self) -> f64 {
        self.shields
    }

    #[inline]
    pub fn shields_max(&self) -> f64 {
        self.shields_max
    }

    #[inline]
    pub fn energy(&self) -> f64 {
        self.energy
    }

    #[inline]
    pub fn energy_max(&self) -> f64 {
        self.energy_max
    }

    #[inline]
    pub fn ion(&self) -> f64 {
        self.ion
    }

    #[inline]
    pub fn ion_max(&self) -> f64 {
        self.ion_max
    }

    #[inline]
    pub fn active(&self) -> bool {
        self.active
    }
}

impl NamedUnit for ControllableInfo {
    #[inline]
    fn name(&self) -> &str {
        ControllableInfo::name(self)
    }
}
