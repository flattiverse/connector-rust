use crate::hierarchy::{ClusterId, ShipDesignId, UpgradeId};
use crate::network::PacketReader;
use crate::unit::{Unit, UnitKind};
use crate::{Indexer, PlayerId, Vector};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ShipId(pub(crate) u8);

impl Indexer for ShipId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Ship {
    name: String,
    id: ShipId,
    cluster: ClusterId,
    ship_design: ShipDesignId,
    player: PlayerId,
    upgrade: UpgradeId,
    hull: f64,
    hull_max: f64,
    hull_repair: f64,
    shields: f64,
    shields_max: f64,
    shields_load: f64,
    size: f64,
    weight: f64,
    energy: f64,
    energy_max: f64,
    energy_cells: f64,
    energy_reactor: f64,
    energy_transfer: f64,
    ion: f64,
    ion_max: f64,
    ion_cells: f64,
    ion_reactor: f64,
    ion_transfer: f64,
    thruster: f64,
    thruster_max: f64,
    nozzle: f64,
    nozzle_max: f64,
    speed_max: f64,
    turnrate: f64,
    cargo_tungsten: f64,
    cargo_iron: f64,
    cargo_silicon: f64,
    cargo_tritium: f64,
    cargo_max: f64,
    extractor: f64,
    extractor_max: f64,
    weapon_speed: f64,
    weapon_time: u16,
    weapon_load: f64,
    weapon_ammo: u16,
    weapon_ammo_max: u16,
    weapon_ammo_production: f64,
}

impl Ship {
    pub fn new(cluster: ClusterId, reader: &mut dyn PacketReader) -> Self {
        Self {
            cluster,
            name: reader.read_string(),
            player: PlayerId(
                reader
                    .read_int32()
                    .try_into()
                    .expect("PlayerId is not within the expected range"),
            ),
            ship_design: ShipDesignId(
                reader
                    .read_int32()
                    .try_into()
                    .expect("ShipDesignId is not within the expected range"),
            ),
            id: ShipId(
                reader
                    .read_int32()
                    .try_into()
                    .expect("ShipId is not within the expected range"),
            ),
            upgrade: UpgradeId(
                reader
                    .read_int32()
                    .try_into()
                    .expect("UpgradeId is not within the expected range"),
            ),

            hull: reader.read_2u(10.0),
            hull_max: reader.read_2u(10.0),
            hull_repair: reader.read_2u(100.0),
            shields: reader.read_2u(10.0),
            shields_max: reader.read_2u(10.0),
            shields_load: reader.read_2u(100.0),
            size: reader.read_3u(1_000.0),
            weight: reader.read_2s(10000.0),
            energy: reader.read_4u(10.0),
            energy_max: reader.read_4u(10.0),
            energy_cells: reader.read_2u(100.0),
            energy_reactor: reader.read_2u(100.0),
            energy_transfer: reader.read_2u(100.0),
            ion: reader.read_2u(100.0),
            ion_max: reader.read_2u(100.0),
            ion_cells: reader.read_2u(100.0),
            ion_reactor: reader.read_2u(1000.0),
            ion_transfer: reader.read_2u(1000.0),
            thruster: reader.read_2u(10000.0),
            thruster_max: reader.read_2u(10000.0),
            nozzle: reader.read_2u(100.0),
            nozzle_max: reader.read_2u(100.0),
            speed_max: reader.read_2u(100.0),
            turnrate: reader.read_2u(100.0),
            cargo_tungsten: reader.read_4u(1000.0),
            cargo_iron: reader.read_4u(1000.0),
            cargo_silicon: reader.read_4u(1000.0),
            cargo_tritium: reader.read_4u(1000.0),
            cargo_max: reader.read_4u(1000.0),
            extractor: reader.read_2u(100.0),
            extractor_max: reader.read_2u(100.0),
            weapon_speed: reader.read_2u(10.0),
            weapon_time: reader.read_uint16(),
            weapon_load: reader.read_3u(1_000.0),
            weapon_ammo: reader.read_uint16(),
            weapon_ammo_max: reader.read_uint16(),
            weapon_ammo_production: reader.read_2u(100000.0),
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn cluster(&self) -> ClusterId {
        self.cluster
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
    pub fn hull_repair(&self) -> f64 {
        self.hull_repair
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
    pub fn shields_load(&self) -> f64 {
        self.shields_load
    }

    #[inline]
    pub fn size(&self) -> f64 {
        self.size
    }

    #[inline]
    pub fn weight(&self) -> f64 {
        self.weight
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
    pub fn energy_cells(&self) -> f64 {
        self.energy_cells
    }

    #[inline]
    pub fn energy_reactor(&self) -> f64 {
        self.energy_reactor
    }

    #[inline]
    pub fn energy_transfer(&self) -> f64 {
        self.energy_transfer
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
    pub fn ion_cells(&self) -> f64 {
        self.ion_cells
    }

    #[inline]
    pub fn ion_reactor(&self) -> f64 {
        self.ion_reactor
    }

    #[inline]
    pub fn ion_transfer(&self) -> f64 {
        self.ion_transfer
    }

    #[inline]
    pub fn thruster(&self) -> f64 {
        self.thruster
    }

    #[inline]
    pub fn thruster_max(&self) -> f64 {
        self.thruster_max
    }

    #[inline]
    pub fn nozzle(&self) -> f64 {
        self.nozzle
    }

    #[inline]
    pub fn nozzle_max(&self) -> f64 {
        self.nozzle_max
    }

    #[inline]
    pub fn speed_max(&self) -> f64 {
        self.speed_max
    }

    #[inline]
    pub fn turnrate(&self) -> f64 {
        self.turnrate
    }

    #[inline]
    pub fn cargo_tungsten(&self) -> f64 {
        self.cargo_tungsten
    }

    #[inline]
    pub fn cargo_iron(&self) -> f64 {
        self.cargo_iron
    }

    #[inline]
    pub fn cargo_silicon(&self) -> f64 {
        self.cargo_silicon
    }

    #[inline]
    pub fn cargo_tritium(&self) -> f64 {
        self.cargo_tritium
    }

    #[inline]
    pub fn cargo_max(&self) -> f64 {
        self.cargo_max
    }

    #[inline]
    pub fn extractor(&self) -> f64 {
        self.extractor
    }

    #[inline]
    pub fn extractor_max(&self) -> f64 {
        self.extractor_max
    }

    #[inline]
    pub fn weapon_speed(&self) -> f64 {
        self.weapon_speed
    }

    #[inline]
    pub fn weapon_time(&self) -> u16 {
        self.weapon_time
    }

    #[inline]
    pub fn weapon_load(&self) -> f64 {
        self.weapon_load
    }

    #[inline]
    pub fn weapon_ammo(&self) -> u16 {
        self.weapon_ammo
    }

    #[inline]
    pub fn weapon_ammo_max(&self) -> u16 {
        self.weapon_ammo_max
    }

    #[inline]
    pub fn weapon_ammo_production(&self) -> f64 {
        self.weapon_ammo_production
    }
}

impl Unit for Ship {
    #[inline]
    fn name(&self) -> &str {
        Ship::name(self)
    }

    #[inline]
    fn cluster(&self) -> ClusterId {
        Ship::cluster(self)
    }

    #[inline]
    fn position(&self) -> Vector {
        warn!("Ship has no position yet!");
        Vector::default()
    }

    #[inline]
    fn gravity(&self) -> f64 {
        warn!("Ship has no gravity set!");
        0.0
    }

    #[inline]
    fn radius(&self) -> f64 {
        warn!("Ship has no radius yet!");
        0.0
    }

    fn update(&mut self, reader: &mut dyn PacketReader) {
        warn!("Ship cannot be updated yet!");
        let _ = reader;
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Ship
    }
}
