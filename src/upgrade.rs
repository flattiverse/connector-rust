use crate::hierarchy::{GlaxyId, UpgradeConfig};
use crate::network::{ConnectionHandle, PacketReader};
use crate::unit::ShipId;
use crate::{GameError, Indexer, NamedUnit};
use std::future::Future;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct UpgradeId(pub(crate) u8);

impl Indexer for UpgradeId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Upgrade {
    galaxy: GlaxyId,
    ship: ShipId,
    id: UpgradeId,
    name: String,
    previous_upgrade: Option<UpgradeId>,
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
    free_spawn: bool,
    connection: ConnectionHandle,
}

impl Upgrade {
    pub fn new(
        id: impl Into<UpgradeId>,
        galaxy: GlaxyId,
        ship: ShipId,
        connection: ConnectionHandle,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            id: id.into(),
            galaxy,
            ship,
            name: reader.read_string(),
            previous_upgrade: reader.read_nullable_byte().map(UpgradeId),
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
            weapon_time: reader.read_uint16() as f64 / 20.0,
            weapon_load: reader.read_2u(10.0),
            free_spawn: reader.read_boolean(),
            connection,
        }
    }

    /// Sets the given values for this [`Upgrade`].
    /// See also [`ConnectionHandle::configure_upgrade`].
    #[inline]
    pub async fn configure(
        &self,
        config: &UpgradeConfig,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection
            .configure_upgrade_split(self.id, config)
            .await
    }

    /// Removes this [`Upgrade`].
    /// See also [`ConnectionHandle::remove_upgrade`].
    #[inline]
    pub async fn remove(&self) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.remove_upgrade_split(self.id).await
    }

    #[inline]
    pub fn galaxy(&self) -> GlaxyId {
        self.galaxy
    }

    #[inline]
    pub fn ship(&self) -> ShipId {
        self.ship
    }

    #[inline]
    pub fn id(&self) -> UpgradeId {
        self.id
    }

    /// The id of the previous [`Upgrade`]. Can be found on the orresponding [`crate::unit::Ship`]
    /// of this [`Upgrade`].
    #[inline]
    pub fn previous_upgrade(&self) -> Option<UpgradeId> {
        self.previous_upgrade
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
    pub fn free_spawn(&self) -> bool {
        self.free_spawn
    }
}

impl NamedUnit for Upgrade {
    #[inline]
    fn name(&self) -> &str {
        Upgrade::name(self)
    }
}
