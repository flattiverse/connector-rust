use crate::hierarchy::ClusterId;
use crate::network::PacketReader;
use crate::unit::ShipDesignId;
use crate::{PlayerId, UpgradeId};

pub struct ControllableInfo {
    cluster: ClusterId,
    name: String,
    reduced: bool,
    ship_design: ShipDesignId,
    player: PlayerId,
    upgrade: UpgradeId,

    hull: f64,
    hull_max: f64,

    shields: f64,
    shields_max: f64,

    energy: f64,
    energy_max: f64,

    ion: f64,
    ion_max: f64,

    active: bool,
}

impl ControllableInfo {
    pub fn new(
        cluster: ClusterId,
        player: PlayerId,
        reader: &mut dyn PacketReader,
        reduced: bool,
    ) -> Self {
        Self {
            active: true,
            cluster,
            player,
            reduced,

            name: reader.read_string(),

            ship_design: ShipDesignId(
                reader
                    .read_int32()
                    .try_into()
                    .expect("ShipDesignId is not within the expected range"),
            ),
            upgrade: UpgradeId(
                reader
                    .read_int32()
                    .try_into()
                    .expect("UpgradeId is not within the expected range"),
            ),

            hull: if reduced { 0.0 } else { reader.read_2u(10.0) },
            hull_max: if reduced { 0.0 } else { reader.read_2u(10.0) },

            shields: if reduced { 0.0 } else { reader.read_2u(10.0) },
            shields_max: if reduced { 0.0 } else { reader.read_2u(10.0) },

            energy: if reduced { 0.0 } else { reader.read_4u(10.0) },
            energy_max: if reduced { 0.0 } else { reader.read_4u(10.0) },

            ion: if reduced { 0.0 } else { reader.read_2u(100.0) },
            ion_max: if reduced { 0.0 } else { reader.read_2u(100.0) },
        }
    }

    pub(crate) fn deactivate(&mut self) {
        self.active = false;
    }

    #[inline]
    pub fn cluster(&self) -> ClusterId {
        self.cluster
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
    pub fn upgrade(&self) -> UpgradeId {
        self.upgrade
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
