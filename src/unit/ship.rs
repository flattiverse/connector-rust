use crate::hierarchy::GlaxyId;
use crate::network::{ConnectionHandle, PacketReader};
use crate::{Indexer, NamedUnit, UniversalHolder, Upgrade, UpgradeId};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, derive_more::From)]
pub struct ShipId(u8);

impl Indexer for ShipId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

pub struct Ship {
    galaxy: GlaxyId,
    id: ShipId,
    upgrades: UniversalHolder<UpgradeId, Upgrade>,
    name: String,
    cost_energy: f64,
    cost_ion: f64,
    cost_iron: f64,
    cost_tungsten: f64,
    cost_silicon: f64,
    cost_tritium: f64,
    cost_time: f64,
    hull: f64,
    hull_repair: f64,
    shields: f64,
    shields_repair: f64,
    size: f64,
    weight: f64,
    energy_max: f64,
    energy_cells: f64,
    energy_reactor: f64,
    energy_transfer: f64,
    ion_max: f64,
    ion_cells: f64,
    ion_reactor: f64,
    ion_transfer: f64,
    thruster: f64,
    nozzle: f64,
    speed: f64,
    turnrate: f64,
    cargo: f64,
    extractor: f64,
    weapon_speed: f64,
    weapon_time: f64,
    weapon_load: f64,
    connection: ConnectionHandle,
}

impl Ship {
    pub fn new(
        id: impl Into<ShipId>,
        galaxy: GlaxyId,
        connection: ConnectionHandle,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            id: id.into(),
            galaxy,
            upgrades: UniversalHolder::with_capacity(256),
            name: reader.read_string(),
            cost_energy: reader.read_2u(1.0),
            cost_ion: reader.read_2u(100.0),
            cost_iron: reader.read_2u(1.0),
            cost_tungsten: reader.read_2u(100.0),
            cost_silicon: reader.read_2u(1.0),
            cost_tritium: reader.read_2u(10.0),
            cost_time: reader.read_2u(10.0),
            hull: reader.read_2u(10.0),
            hull_repair: reader.read_2u(100.0),
            shields: reader.read_2u(10.0),
            shields_repair: reader.read_2u(100.0),
            size: reader.read_2u(10.0),
            weight: reader.read_2s(10000.0),
            energy_max: reader.read_2u(10.0),
            energy_cells: reader.read_4u(100.0),
            energy_reactor: reader.read_2u(100.0),
            energy_transfer: reader.read_2u(100.0),
            ion_max: reader.read_2u(100.0),
            ion_cells: reader.read_2u(100.0),
            ion_reactor: reader.read_2u(1000.0),
            ion_transfer: reader.read_2u(1000.0),
            thruster: reader.read_2u(10000.0),
            nozzle: reader.read_2u(100.0),
            speed: reader.read_2u(100.0),
            turnrate: reader.read_2u(100.0),
            cargo: reader.read_4u(1000.0),
            extractor: reader.read_2u(100.0),
            weapon_speed: reader.read_2u(10.0),
            weapon_time: reader.read_uint16() as _,
            weapon_load: reader.read_2u(10.0),
            connection,
        }
    }

    pub(crate) fn read_upgrade(&mut self, id: UpgradeId, reader: &mut dyn PacketReader) {
        self.upgrades.set(
            id,
            Upgrade::new(id, self.galaxy, self.id, self.connection.clone(), reader),
        );
    }

    #[inline]
    pub fn id(&self) -> ShipId {
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

    #[inline]
    pub fn cost_energy(&self) -> f64 {
        self.cost_energy
    }

    #[inline]
    pub fn cost_ion(&self) -> f64 {
        self.cost_ion
    }

    #[inline]
    pub fn cost_iron(&self) -> f64 {
        self.cost_iron
    }

    #[inline]
    pub fn cost_tungsten(&self) -> f64 {
        self.cost_tungsten
    }

    #[inline]
    pub fn cost_silicon(&self) -> f64 {
        self.cost_silicon
    }

    #[inline]
    pub fn cost_tritium(&self) -> f64 {
        self.cost_tritium
    }

    #[inline]
    pub fn cost_time(&self) -> f64 {
        self.cost_time
    }

    #[inline]
    pub fn hull(&self) -> f64 {
        self.hull
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
    pub fn shields_repair(&self) -> f64 {
        self.shields_repair
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
    pub fn nozzle(&self) -> f64 {
        self.nozzle
    }

    #[inline]
    pub fn speed(&self) -> f64 {
        self.speed
    }

    #[inline]
    pub fn turnrate(&self) -> f64 {
        self.turnrate
    }

    #[inline]
    pub fn cargo(&self) -> f64 {
        self.cargo
    }

    #[inline]
    pub fn extractor(&self) -> f64 {
        self.extractor
    }

    #[inline]
    pub fn weapon_speed(&self) -> f64 {
        self.weapon_speed
    }

    #[inline]
    pub fn weapon_time(&self) -> f64 {
        self.weapon_time
    }

    #[inline]
    pub fn weapon_load(&self) -> f64 {
        self.weapon_load
    }

    #[inline]
    pub fn get_upgrade(&self, id: UpgradeId) -> Option<&Upgrade> {
        self.upgrades.get(id)
    }
}

impl NamedUnit for Ship {
    #[inline]
    fn name(&self) -> &str {
        Ship::name(self)
    }
}
