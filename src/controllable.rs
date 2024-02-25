use crate::hierarchy::{GalaxyId, ShipDesignId, UpgradeId};
use crate::network::{ConnectionHandle, PacketReader};
use crate::{GameError, Indexer, Vector};
use std::future::Future;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ControllableId(pub(crate) u8);

impl Indexer for ControllableId {
    #[inline]
    fn index(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug)]
pub struct Controllable {
    active: bool,
    galaxy: GalaxyId,
    id: ControllableId,
    name: String,
    ship_design: ShipDesignId,
    active_upgrades: Box<[UpgradeId]>,

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
    thruster_max_forward: f64,
    thruster_max_backward: f64,
    nozzle: f64,
    nozzle_max: f64,
    speed_max: f64,
    turnrate: f64,
    cargo_tungsten: f64,
    cargo_iron: f64,
    cargo_silicon: f64,
    cargo_tritium: f64,
    cargo_max: f64,
    extractor_max: f64,
    weapon_speed: f64,
    weapon_time: u16,
    weapon_load: f64,
    weapon_damage: f64,
    weapon_ammo: f64,
    weapon_ammo_max: f64,
    weapon_ammo_production: f64,
    direction: f64,

    position: Vector,
    movement: Vector,

    connection: ConnectionHandle,
}

impl Controllable {
    pub fn new(
        galaxy: GalaxyId,
        id: ControllableId,
        reader: &mut dyn PacketReader,
        connection: ConnectionHandle,
    ) -> Self {
        Self {
            active: true,
            galaxy,
            id,
            name: reader.read_string(),
            ship_design: ShipDesignId(reader.read_byte()),

            size: reader.read_3u(1_000.0),
            weight: reader.read_2s(10_000.0),
            active_upgrades: reader
                .read_bytes(32)
                .into_iter()
                .map(UpgradeId)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            hull_max: reader.read_3u(10_000.0),
            hull_repair: reader.read_3u(10_000.0),
            shields_max: reader.read_3u(10_000.0),
            shields_load: reader.read_3u(10_000.0),

            hull: 0.0,
            shields: 0.0,
            energy: 0.0,
            energy_max: 0.0,
            energy_cells: 0.0,
            energy_reactor: 0.0,
            energy_transfer: 0.0,
            ion: 0.0,
            ion_max: 0.0,
            ion_cells: 0.0,
            ion_reactor: 0.0,
            ion_transfer: 0.0,
            thruster: 0.0,
            thruster_max_forward: 0.0,
            thruster_max_backward: 0.0,
            nozzle: 0.0,
            nozzle_max: 0.0,
            speed_max: 0.0,
            turnrate: 0.0,
            cargo_tungsten: 0.0,
            cargo_iron: 0.0,
            cargo_silicon: 0.0,
            cargo_tritium: 0.0,
            cargo_max: 0.0,
            extractor_max: 0.0,
            weapon_speed: 0.0,
            weapon_time: 0,
            weapon_load: 0.0,
            weapon_damage: 0.0,
            weapon_ammo: 0.0,
            weapon_ammo_max: 0.0,
            weapon_ammo_production: 0.0,
            direction: 0.0,

            position: Default::default(),
            movement: Default::default(),

            connection,
        }
    }

    pub(crate) fn update(&mut self, reader: &mut dyn PacketReader) {
        self.energy_max = reader.read_4u(1_000.0);
        self.energy_cells = reader.read_4u(1_000.0);
        self.energy_reactor = reader.read_4u(1_000.0);
        self.energy_transfer = reader.read_4u(1_000.0);
        self.ion_max = reader.read_4u(1_000.0);
        self.ion_cells = reader.read_4u(1_000.0);
        self.ion_reactor = reader.read_4u(1_000.0);
        self.ion_transfer = reader.read_4u(1_000.0);
        self.thruster_max_forward = reader.read_2u(10_000.0);
        self.thruster_max_backward = reader.read_2u(10_000.0);
        self.nozzle_max = reader.read_2s(100.0);
        self.speed_max = reader.read_2u(1_000.0);
        self.cargo_max = reader.read_4u(1_000.0);
        self.extractor_max = reader.read_4u(1_000.0);
        self.weapon_speed = reader.read_2u(1_000.0);
        self.weapon_time = reader.read_uint16();
        self.weapon_load = reader.read_3u(1_000.0);
        self.weapon_damage = reader.read_3u(10_000.0);
        self.weapon_ammo_max = reader.read_uint16() as f64;
        self.weapon_ammo_production = reader.read_4u(1_000.0);
    }

    pub(crate) fn dynamic_update(&mut self, reader: &mut dyn PacketReader) {
        self.hull = reader.read_3u(10_000.0);
        self.shields = reader.read_3u(10_000.0);
        self.energy = reader.read_4u(1_000.0);
        self.ion = reader.read_4u(1_000.0);
        self.thruster = reader.read_2s(10_000.0);
        self.nozzle = reader.read_2s(100.0);
        self.turnrate = reader.read_2s(100.0);
        self.cargo_tungsten = reader.read_4u(1_000.0);
        self.cargo_iron = reader.read_4u(1_000.0);
        self.cargo_silicon = reader.read_4u(1_000.0);
        self.cargo_tritium = reader.read_4u(1_000.0);
        self.weapon_ammo = reader.read_4u(1_000.0);
        self.position = Vector::default().with_read(reader);
        self.movement = Vector::default().with_read(reader);
        self.direction = reader.read_2u(100.0);
    }

    pub(crate) fn deactivate(&mut self) {
        self.active = false;
    }

    /// Self-destructs this [`Controllable`].
    /// See also [`ConnectionHandle::kill_controllable`].
    #[inline]
    pub async fn kill(&self) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        if !self.alive() {
            return Err(GameError::from(0xF5));
        } else {
            self.connection.kill_controllable_split(self.id).await
        }
    }

    /// Revives this [`Controllable`].
    /// See also [`ConnectionHandle::continue_controllable`].
    #[inline]
    pub async fn r#continue(
        &self,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        if self.alive() {
            return Err(GameError::from(0xF6));
        } else {
            self.connection.continue_controllable_split(self.id).await
        }
    }

    /// Revives this [`Controllable`].
    /// See also [`ConnectionHandle::unregister_controllable`].
    #[inline]
    pub async fn unregister(
        &self,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.connection.unregister_controllable_split(self.id).await
    }

    #[inline]
    pub fn active(&self) -> bool {
        self.active
    }

    #[inline]
    pub fn alive(&self) -> bool {
        self.hull > 0.0
    }

    #[inline]
    pub fn galaxy(&self) -> GalaxyId {
        self.galaxy
    }

    #[inline]
    pub fn id(&self) -> ControllableId {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn ship_design(&self) -> ShipDesignId {
        self.ship_design
    }

    #[inline]
    pub fn active_upgrades(&self) -> &[UpgradeId] {
        &self.active_upgrades[..]
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
    pub fn thruster_max_forward(&self) -> f64 {
        self.thruster_max_forward
    }

    #[inline]
    pub fn thruster_max_backward(&self) -> f64 {
        self.thruster_max_backward
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
    pub fn weapon_damage(&self) -> f64 {
        self.weapon_damage
    }

    #[inline]
    pub fn weapon_ammo(&self) -> f64 {
        self.weapon_ammo
    }

    #[inline]
    pub fn weapon_ammo_max(&self) -> f64 {
        self.weapon_ammo_max
    }

    #[inline]
    pub fn weapon_ammo_production(&self) -> f64 {
        self.weapon_ammo_production
    }

    #[inline]
    pub fn direction(&self) -> f64 {
        self.direction
    }

    #[inline]
    pub fn position(&self) -> Vector {
        self.position
    }

    #[inline]
    pub fn movement(&self) -> Vector {
        self.movement
    }
}
