use crate::atomics::Atomic;
use crate::hierarchy::{Galaxy, ShipDesignId, ShipUpgradeId};
use crate::network::PacketReader;
use crate::{GameError, Identifiable, Indexer, Vector};
use arc_swap::ArcSwap;
use std::ops::Deref;
use std::sync::Arc;

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
    active: Atomic<bool>,
    galaxy: Arc<Galaxy>,
    id: ControllableId,
    name: String,
    ship_design: ShipDesignId,
    active_upgrades: ArcSwap<Vec<ShipUpgradeId>>,

    hull: Atomic<f64>,
    hull_max: Atomic<f64>,
    hull_repair: Atomic<f64>,
    shields: Atomic<f64>,
    shields_max: Atomic<f64>,
    shields_load: Atomic<f64>,

    radius: Atomic<f64>,
    gravity: Atomic<f64>,
    energy: Atomic<f64>,
    energy_max: Atomic<f64>,
    energy_cells: Atomic<f64>,
    energy_reactor: Atomic<f64>,
    energy_transfer: Atomic<f64>,
    ion: Atomic<f64>,
    ion_max: Atomic<f64>,
    ion_cells: Atomic<f64>,
    ion_reactor: Atomic<f64>,
    ion_transfer: Atomic<f64>,
    thruster: Atomic<f64>,
    thruster_max_forward: Atomic<f64>,
    thruster_max_backward: Atomic<f64>,
    nozzle: Atomic<f64>,
    nozzle_max: Atomic<f64>,
    speed_max: Atomic<f64>,
    turnrate: Atomic<f64>,
    cargo_tungsten: Atomic<f64>,
    cargo_iron: Atomic<f64>,
    cargo_silicon: Atomic<f64>,
    cargo_tritium: Atomic<f64>,
    cargo_max: Atomic<f64>,
    extractor_max: Atomic<f64>,
    weapon_speed: Atomic<f64>,
    weapon_time: Atomic<u16>,
    weapon_load: Atomic<f64>,
    weapon_damage: Atomic<f64>,
    weapon_ammo: Atomic<f64>,
    weapon_ammo_max: Atomic<f64>,
    weapon_ammo_production: Atomic<f64>,
    direction: Atomic<f64>,

    position: Atomic<Vector>,
    movement: Atomic<Vector>,
}

impl Controllable {
    pub fn new(galaxy: Arc<Galaxy>, id: ControllableId, reader: &mut dyn PacketReader) -> Self {
        Self {
            active: Atomic::from(true),
            galaxy,
            id,
            name: reader.read_string(),
            ship_design: ShipDesignId(reader.read_byte()),

            radius: Atomic::from_reader(reader),
            gravity: Atomic::from_reader(reader),
            active_upgrades: ArcSwap::new(Arc::new(
                reader
                    .read_bytes(32)
                    .into_iter()
                    .map(ShipUpgradeId)
                    .collect::<Vec<_>>(),
            )),
            hull_max: Atomic::from_reader(reader),
            hull_repair: Atomic::from_reader(reader),
            shields_max: Atomic::from_reader(reader),
            shields_load: Atomic::from_reader(reader),

            energy_max: Atomic::from_reader(reader),
            energy_cells: Atomic::from_reader(reader),
            energy_reactor: Atomic::from_reader(reader),
            energy_transfer: Atomic::from_reader(reader),
            ion_max: Atomic::from_reader(reader),
            ion_cells: Atomic::from_reader(reader),
            ion_reactor: Atomic::from_reader(reader),
            ion_transfer: Atomic::from_reader(reader),

            hull: Atomic::from(0.0),
            shields: Atomic::from(0.0),
            energy: Atomic::from(0.0),
            ion: Atomic::from(0.0),
            thruster: Atomic::from(0.0),
            thruster_max_forward: Atomic::from(0.0),
            thruster_max_backward: Atomic::from(0.0),
            nozzle: Atomic::from(0.0),
            nozzle_max: Atomic::from(0.0),
            speed_max: Atomic::from(0.0),
            turnrate: Atomic::from(0.0),
            cargo_tungsten: Atomic::from(0.0),
            cargo_iron: Atomic::from(0.0),
            cargo_silicon: Atomic::from(0.0),
            cargo_tritium: Atomic::from(0.0),
            cargo_max: Atomic::from(0.0),
            extractor_max: Atomic::from(0.0),
            weapon_speed: Atomic::from(0.0),
            weapon_time: Atomic::from(0),
            weapon_load: Atomic::from(0.0),
            weapon_damage: Atomic::from(0.0),
            weapon_ammo: Atomic::from(0.0),
            weapon_ammo_max: Atomic::from(0.0),
            weapon_ammo_production: Atomic::from(0.0),
            direction: Atomic::from(0.0),

            position: Default::default(),
            movement: Default::default(),
        }
    }

    pub(crate) fn update(&self, reader: &mut dyn PacketReader) {
        self.radius.read(reader);
        self.gravity.read(reader);
        self.active_upgrades.store(Arc::new(
            reader
                .read_bytes(32)
                .into_iter()
                .map(ShipUpgradeId)
                .collect(),
        ));

        self.hull_max.read(reader);
        self.hull_repair.read(reader);
        self.shields_max.read(reader);
        self.shields_load.read(reader);

        self.energy_max.read(reader);
        self.energy_cells.read(reader);
        self.energy_reactor.read(reader);
        self.energy_transfer.read(reader);
        self.ion_max.read(reader);
        self.ion_cells.read(reader);
        self.ion_reactor.read(reader);
        self.ion_transfer.read(reader);

        self.thruster_max_forward.read(reader);
        self.thruster_max_backward.read(reader);
        self.nozzle_max.read(reader);
        self.speed_max.read(reader);
        self.cargo_max.read(reader);
        self.extractor_max.read(reader);
        self.weapon_speed.read(reader);
        self.weapon_time.read(reader);
        self.weapon_load.read(reader);
        self.weapon_damage.read(reader);
        self.weapon_ammo_max.read(reader);
        self.weapon_ammo_production.read(reader);
    }

    pub(crate) fn dynamic_update(&self, reader: &mut dyn PacketReader) {
        self.hull.read(reader);
        self.shields.read(reader);
        self.energy.read(reader);
        self.ion.read(reader);
        self.thruster.read(reader);
        self.nozzle.read(reader);
        self.turnrate.read(reader);
        self.cargo_tungsten.read(reader);
        self.cargo_iron.read(reader);
        self.cargo_silicon.read(reader);
        self.cargo_tritium.read(reader);
        self.weapon_ammo.read(reader);
        self.position.read(reader);
        self.movement.read(reader);
        self.direction.read(reader);
    }

    pub(crate) fn deactivate(&self) {
        self.active.store(false);
    }

    /// Self-destructs this [`Controllable`].
    /// See also [`ConnectionHandle::kill_controllable`].
    #[inline]
    pub async fn kill(&self) -> Result<(), GameError> {
        if !self.alive() {
            return Err(GameError::from(0x20));
        } else {
            self.galaxy.connection().kill_controllable(self.id).await
        }
    }

    /// Revives this [`Controllable`].
    /// See also [`ConnectionHandle::continue_controllable`].
    #[inline]
    pub async fn r#continue(&self) -> Result<(), GameError> {
        if self.alive() {
            return Err(GameError::from(0x21));
        } else {
            self.galaxy
                .connection()
                .continue_controllable(self.id)
                .await
        }
    }

    /// Sets the thruster fand nozzle of this [`Controllable`] at the same time. Please note, that
    /// you need to stay within the limits of your ships configuration. A postive thruster value
    /// make your ship advance forward. A negative thruster value negatively. Usually a ship is
    /// designed to be faster when flying forward.
    /// See also [`ConnectionHandle::set_controllable_thruster_nozzel`].
    pub async fn set_thruster_nozzle(&self, thruster: f64, nozzle: f64) -> Result<(), GameError> {
        if !self.active() {
            return Err(GameError::from(0x22));
        } else if self.hull() <= 0.0 {
            return Err(GameError::from(0x20));
        } else if thruster < self.thruster_max_backward() * -1.05
            || thruster > self.thruster_max_forward() * 1.05
            || nozzle < self.nozzle_max() * -1.05
            || nozzle > self.nozzle_max() * 1.05
        {
            return Err(GameError::from(0x31));
        } else {
            self.galaxy
                .connection()
                .set_controllable_thruster_nozzel(self.id, thruster, nozzle)
                .await
        }
    }

    /// Sets the thruster of the [`ControllableId`]. Please note, that you need to stay within the
    /// limits of your ships configuration. A positive thruster value will make your ship advance
    /// forward. A negative thruster value nagetively. Usually, a ship is deisgned to be faster when
    /// flying forward.
    pub async fn set_thruster(&self, thruster: f64) -> Result<(), GameError> {
        if !self.active() {
            return Err(GameError::from(0x22));
        } else if self.hull() <= 0.0 {
            return Err(GameError::from(0x20));
        } else if thruster < self.thruster_max_backward() * -1.05
            || thruster > self.thruster_max_forward() * 1.05
        {
            return Err(GameError::from(0x31));
        } else {
            self.galaxy
                .connection()
                .set_controllable_thruster(self.id, thruster)
                .await
        }
    }

    /// See also [`ConnectionHandle::unregister_controllable`].
    #[inline]
    pub async fn unregister(&self) -> Result<(), GameError> {
        self.galaxy
            .connection()
            .unregister_controllable(self.id)
            .await
    }

    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    #[inline]
    pub fn alive(&self) -> bool {
        self.hull() > 0.0
    }

    #[inline]
    pub fn galaxy(&self) -> &Arc<Galaxy> {
        &self.galaxy
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
    pub fn active_upgrades(&self) -> impl Deref<Target = Arc<Vec<ShipUpgradeId>>> + '_ {
        self.active_upgrades.load()
    }

    #[inline]
    pub fn hull(&self) -> f64 {
        self.hull.load()
    }

    #[inline]
    pub fn hull_max(&self) -> f64 {
        self.hull_max.load()
    }

    #[inline]
    pub fn hull_repair(&self) -> f64 {
        self.hull_repair.load()
    }

    #[inline]
    pub fn shields(&self) -> f64 {
        self.shields.load()
    }

    #[inline]
    pub fn shields_max(&self) -> f64 {
        self.shields_max.load()
    }

    #[inline]
    pub fn shields_load(&self) -> f64 {
        self.shields_load.load()
    }

    #[inline]
    pub fn radius(&self) -> f64 {
        self.radius.load()
    }

    #[inline]
    pub fn gravity(&self) -> f64 {
        self.gravity.load()
    }

    #[inline]
    pub fn energy(&self) -> f64 {
        self.energy.load()
    }

    #[inline]
    pub fn energy_max(&self) -> f64 {
        self.energy_max.load()
    }

    #[inline]
    pub fn energy_cells(&self) -> f64 {
        self.energy_cells.load()
    }

    #[inline]
    pub fn energy_reactor(&self) -> f64 {
        self.energy_reactor.load()
    }

    #[inline]
    pub fn energy_transfer(&self) -> f64 {
        self.energy_transfer.load()
    }

    #[inline]
    pub fn ion(&self) -> f64 {
        self.ion.load()
    }

    #[inline]
    pub fn ion_max(&self) -> f64 {
        self.ion_max.load()
    }

    #[inline]
    pub fn ion_cells(&self) -> f64 {
        self.ion_cells.load()
    }

    #[inline]
    pub fn ion_reactor(&self) -> f64 {
        self.ion_reactor.load()
    }

    #[inline]
    pub fn ion_transfer(&self) -> f64 {
        self.ion_transfer.load()
    }

    #[inline]
    pub fn thruster(&self) -> f64 {
        self.thruster.load()
    }

    #[inline]
    pub fn thruster_max_forward(&self) -> f64 {
        self.thruster_max_forward.load()
    }

    #[inline]
    pub fn thruster_max_backward(&self) -> f64 {
        self.thruster_max_backward.load()
    }

    #[inline]
    pub fn nozzle(&self) -> f64 {
        self.nozzle.load()
    }

    #[inline]
    pub fn nozzle_max(&self) -> f64 {
        self.nozzle_max.load()
    }

    #[inline]
    pub fn speed_max(&self) -> f64 {
        self.speed_max.load()
    }

    #[inline]
    pub fn turnrate(&self) -> f64 {
        self.turnrate.load()
    }

    #[inline]
    pub fn cargo_tungsten(&self) -> f64 {
        self.cargo_tungsten.load()
    }

    #[inline]
    pub fn cargo_iron(&self) -> f64 {
        self.cargo_iron.load()
    }

    #[inline]
    pub fn cargo_silicon(&self) -> f64 {
        self.cargo_silicon.load()
    }

    #[inline]
    pub fn cargo_tritium(&self) -> f64 {
        self.cargo_tritium.load()
    }

    #[inline]
    pub fn cargo_max(&self) -> f64 {
        self.cargo_max.load()
    }

    #[inline]
    pub fn extractor_max(&self) -> f64 {
        self.extractor_max.load()
    }

    #[inline]
    pub fn weapon_speed(&self) -> f64 {
        self.weapon_speed.load()
    }

    #[inline]
    pub fn weapon_time(&self) -> u16 {
        self.weapon_time.load()
    }

    #[inline]
    pub fn weapon_load(&self) -> f64 {
        self.weapon_load.load()
    }

    #[inline]
    pub fn weapon_damage(&self) -> f64 {
        self.weapon_damage.load()
    }

    #[inline]
    pub fn weapon_ammo(&self) -> f64 {
        self.weapon_ammo.load()
    }

    #[inline]
    pub fn weapon_ammo_max(&self) -> f64 {
        self.weapon_ammo_max.load()
    }

    #[inline]
    pub fn weapon_ammo_production(&self) -> f64 {
        self.weapon_ammo_production.load()
    }

    #[inline]
    pub fn direction(&self) -> f64 {
        self.direction.load()
    }

    #[inline]
    pub fn position(&self) -> Vector {
        self.position.load()
    }

    #[inline]
    pub fn movement(&self) -> Vector {
        self.movement.load()
    }
}

impl Identifiable<ControllableId> for Controllable {
    #[inline]
    fn id(&self) -> ControllableId {
        self.id
    }
}
