use crate::galaxy_hierarchy::{
    ArmorSubsystem, AsSubsystemBase, BatterySubsystem, CargoSubsystem, ClassicShipControllable,
    Cluster, EnergyCellSubsystem, HullSubsystem, Identifiable, Indexer, ModernShipControllable,
    ModernShipGeometry, RepairSubsystem, ResourceMinerSubsystem, ShieldSubsystem,
    StructureOptimizerSubsystem, SubsystemExt, SystemExtIntern,
};
use crate::network::{InvalidArgumentKind, PacketReader};
use crate::unit::UnitKind;
use crate::utils::{Also, Atomic, Let, Readable};
use crate::{
    FlattiverseEvent, FlattiverseEventKind, GameError, GameErrorKind, SubsystemSlot,
    SubsystemStatus, Vector,
};
use arc_swap::ArcSwapWeak;
use std::future::Future;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::{Arc, Weak};

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
    name: String,
    id: ControllableId,
    cluster: ArcSwapWeak<Cluster>,
    active: Atomic<bool>,
    alive: Atomic<bool>,
    tier_change_pending: Atomic<bool>,
    tier_change_slot: Atomic<SubsystemSlot>,
    tier_change_target_tier: Atomic<u8>,
    remaining_tier_change_ticks: Atomic<u16>,
    position: Atomic<Vector>,
    movement: Atomic<Vector>,
    angle: Atomic<f32>,
    angular_velocity: Atomic<f32>,
    hull: HullSubsystem,
    shield: ShieldSubsystem,
    armor: ArmorSubsystem,
    repair: RepairSubsystem,
    cargo: CargoSubsystem,
    resource_miner: ResourceMinerSubsystem,
    structure_optimizer: StructureOptimizerSubsystem,
    energy_battery: BatterySubsystem,
    ion_battery: BatterySubsystem,
    neutrino_battery: BatterySubsystem,
    energy_cell: EnergyCellSubsystem,
    ion_cell: EnergyCellSubsystem,
    neutrino_cell: EnergyCellSubsystem,
    environment_heat_this_tick: Atomic<f32>,
    environment_heat_energy_cost_this_tick: Atomic<f32>,
    environment_heat_energy_overflow_this_tick: Atomic<f32>,
    environment_radiation_this_tick: Atomic<f32>,
    environment_radiation_damage_before_armor_this_tick: Atomic<f32>,
    environment_armor_blocked_damage_this_tick: Atomic<f32>,
    environment_hull_damage_this_tick: Atomic<f32>,
    specialization: ControllableSpecialization,
}

impl Controllable {
    pub(crate) fn from_packet(
        kind: UnitKind,
        cluster: &Arc<Cluster>,
        id: ControllableId,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Self {
            name,
            id,
            cluster: ArcSwapWeak::new(Arc::downgrade(cluster)),
            active: Atomic::from(true),
            position: Atomic::from_reader(reader),
            movement: Atomic::from_reader(reader),
            angle: Atomic::from_reader(reader),
            angular_velocity: Atomic::from_reader(reader),
            alive: Atomic::from(reader.read_byte() != 0x00),
            tier_change_pending: Atomic::from(reader.read_byte() != 0x00),
            tier_change_slot: Atomic::from_reader(reader),
            tier_change_target_tier: Atomic::from(reader.read_byte()),
            remaining_tier_change_ticks: Atomic::from(reader.read_uint16()),
            hull: HullSubsystem::create_classic_ship_hull(Weak::default()),
            shield: ShieldSubsystem::create_classic_ship_shield(Weak::default()),
            armor: ArmorSubsystem::create_classic_ship_armor(Weak::default()),
            repair: RepairSubsystem::create_classic_ship_repair(Weak::default()),
            cargo: CargoSubsystem::create_classic_ship_cargo(Weak::default()),
            resource_miner: ResourceMinerSubsystem::create_classic_ship_resource_miner(
                Weak::default(),
            ),
            structure_optimizer: StructureOptimizerSubsystem::new(Weak::default(), false, 0.0),
            energy_battery: BatterySubsystem::create_classic_ship_energy_battery(Weak::default()),
            ion_battery: BatterySubsystem::create_missing_battery(
                Weak::default(),
                "IonBattery".to_string(),
                SubsystemSlot::IonBattery,
            ),
            neutrino_battery: BatterySubsystem::create_missing_battery(
                Weak::default(),
                "NeutrinoBattery".to_string(),
                SubsystemSlot::NeutrinoBattery,
            ),
            energy_cell: EnergyCellSubsystem::create_classic_ship_energy_cell(Weak::default()),
            ion_cell: EnergyCellSubsystem::create_missing_cell(
                Weak::default(),
                "IonCell".to_string(),
                SubsystemSlot::IonCell,
            ),
            neutrino_cell: EnergyCellSubsystem::create_missing_cell(
                Weak::default(),
                "NeutrinoCell".to_string(),
                SubsystemSlot::NeutrinoCell,
            ),
            environment_heat_this_tick: Default::default(),
            environment_heat_energy_cost_this_tick: Default::default(),
            environment_heat_energy_overflow_this_tick: Default::default(),
            environment_radiation_this_tick: Default::default(),
            environment_radiation_damage_before_armor_this_tick: Default::default(),
            environment_armor_blocked_damage_this_tick: Default::default(),
            environment_hull_damage_this_tick: Default::default(),
            specialization: match kind {
                UnitKind::ClassicShipPlayerUnit => {
                    ControllableSpecialization::ClassicShip(ClassicShipControllable::new())
                }
                UnitKind::ModernShipPlayerUnit => {
                    ControllableSpecialization::ModernShip(ModernShipControllable::new())
                }
                _ => {
                    return Err(GameErrorKind::InvalidArgument {
                        reason: InvalidArgumentKind::Unknown(Default::default()),
                        parameter: "kind".to_string(),
                    }
                    .into())
                }
            },
        }
        .also(|this| {
            this.read_initial_state(reader);
            match &mut this.specialization {
                ControllableSpecialization::ClassicShip(ship) => ship.read_initial_state(reader),
                ControllableSpecialization::ModernShip(ship) => ship.read_initial_state(reader),
            }
        })
        .r#let(Arc::new)
        .also(|this| {
            // finish the initialization of cross-references
            for subsystem in [
                this.hull.as_subsystem_base(),
                this.shield.as_subsystem_base(),
                this.armor.as_subsystem_base(),
                this.repair.as_subsystem_base(),
                this.cargo.as_subsystem_base(),
                this.resource_miner.as_subsystem_base(),
                this.structure_optimizer.as_subsystem_base(),
                this.energy_battery.as_subsystem_base(),
                this.ion_battery.as_subsystem_base(),
                this.neutrino_battery.as_subsystem_base(),
                this.energy_cell.as_subsystem_base(),
                this.ion_cell.as_subsystem_base(),
                this.neutrino_cell.as_subsystem_base(),
            ] {
                subsystem.controllable.store(Arc::downgrade(this));
            }

            match this.specialization() {
                ControllableSpecialization::ClassicShip(ship) => ship
                    .iter_subsystem_bases()
                    .for_each(|s| s.controllable.store(Arc::downgrade(this))),
                ControllableSpecialization::ModernShip(ship) => ship
                    .iter_subsystem_bases()
                    .for_each(|s| s.controllable.store(Arc::downgrade(this))),
            }
        }))
    }

    /// The id of the controllable.
    #[inline]
    pub fn id(&self) -> ControllableId {
        self.id
    }

    /// The name of the controllable.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Declared runtime unit kind that this controllable uses while it is alive in the world.
    #[inline]
    pub fn kind(&self) -> UnitKind {
        UnitKind::ClassicShipPlayerUnit
    }

    /// The cluster this unit currently is in.
    #[inline]
    pub fn cluster(&self) -> Arc<Cluster> {
        self.cluster.load().upgrade().unwrap()
    }

    /// The position of the unit.
    #[inline]
    pub fn position(&self) -> Vector {
        self.position.load()
    }

    /// The movement of the unit.
    #[inline]
    pub fn movement(&self) -> Vector {
        self.movement.load()
    }

    /// The facing angle of the unit.
    #[inline]
    pub fn angle(&self) -> f32 {
        self.angle.load()
    }

    /// The angular velocity of the unit.
    #[inline]
    pub fn angular_velocity(&self) -> f32 {
        self.angular_velocity.load()
    }

    /// The hull subsystem of the controllable.
    #[inline]
    pub fn hull(&self) -> &HullSubsystem {
        &self.hull
    }

    /// The shield subsystem of the controllable.
    #[inline]
    pub fn shield(&self) -> &ShieldSubsystem {
        &self.shield
    }

    /// The armor subsystem of the controllable.
    #[inline]
    pub fn armor(&self) -> &ArmorSubsystem {
        &self.armor
    }

    /// The energy battery subsystem of the controllable.
    #[inline]
    pub fn energy_battery(&self) -> &BatterySubsystem {
        &self.energy_battery
    }

    /// The ion battery subsystem of the controllable.
    #[inline]
    pub fn ion_battery(&self) -> &BatterySubsystem {
        &self.ion_battery
    }

    /// The neutrino battery subsystem of the controllable.
    #[inline]
    pub fn neutrino_battery(&self) -> &BatterySubsystem {
        &self.neutrino_battery
    }

    /// The energy cell subsystem of the controllable.
    #[inline]
    pub fn energy_cell(&self) -> &EnergyCellSubsystem {
        &self.energy_cell
    }

    /// The ion cell subsystem of the controllable.
    #[inline]
    pub fn ion_cell(&self) -> &EnergyCellSubsystem {
        &self.ion_cell
    }

    /// The neutrino cell subsystem of the controllable.
    #[inline]
    pub fn neutrino_cell(&self) -> &EnergyCellSubsystem {
        &self.neutrino_cell
    }

    /// The repair subsystem of the controllable.
    #[inline]
    pub fn repair(&self) -> &RepairSubsystem {
        &self.repair
    }

    /// The cargo subsystem of the controllable.
    #[inline]
    pub fn cargo(&self) -> &CargoSubsystem {
        &self.cargo
    }

    /// The resource miner subsystem of the controllable.
    #[inline]
    pub fn resource_miner(&self) -> &ResourceMinerSubsystem {
        &self.resource_miner
    }

    #[inline]
    pub fn structure_optimizer(&self) -> &StructureOptimizerSubsystem {
        &self.structure_optimizer
    }

    /// Aggregated environment heat applied during the current server tick.
    #[inline]
    pub fn environment_heat_this_tick(&self) -> f32 {
        self.environment_heat_this_tick.load()
    }

    /// Energy drained by environment heat during the current server tick.
    #[inline]
    pub fn environment_heat_energy_cost_this_tick(&self) -> f32 {
        self.environment_heat_energy_cost_this_tick.load()
    }

    /// Heat energy that could not be paid and overflowed into radiation during the current server
    /// tick.
    #[inline]
    pub fn environment_heat_energy_overflow_this_tick(&self) -> f32 {
        self.environment_heat_energy_overflow_this_tick.load()
    }

    /// Aggregated environment radiation applied during the current server tick.
    #[inline]
    pub fn environment_radiation_this_tick(&self) -> f32 {
        self.environment_radiation_this_tick.load()
    }

    /// Radiation damage before armor reduction during the current server tick.
    #[inline]
    pub fn environment_radiation_damage_before_armor_this_tick(&self) -> f32 {
        self.environment_radiation_damage_before_armor_this_tick
            .load()
    }

    /// Environment damage blocked by armor during the current server tick.
    #[inline]
    pub fn environment_armor_blocked_damage_this_tick(&self) -> f32 {
        self.environment_armor_blocked_damage_this_tick.load()
    }

    /// Environment damage that reached the hull during the current server tick.
    #[inline]
    pub fn environment_hull_damage_this_tick(&self) -> f32 {
        self.environment_hull_damage_this_tick.load()
    }

    /// True while this controllable currently has an active in-world runtime.
    #[inline]
    pub fn alive(&self) -> bool {
        self.alive.load()
    }

    /// True while this controllable still exists on the server and can still be addressed by
    /// commands. After the final close, this becomes `false` permanently.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    // TODO
    // pub(crate) fn get_tier_change_target_tier(&self, slot: SubsystemSlot, current_tier: u8) -> u8 {
    //     if self.tier_change_pending.load() && self.tier_change_slot.load() == slot {
    //         self.tier_change_target_tier.load()
    //     } else {
    //         current_tier
    //     }
    // }

    // TODO
    // pub(crate) fn get_remaining_tier_change_ticks(&self, slot: SubsystemSlot) -> i32 {
    //     if self.tier_change_pending.load() && self.tier_change_slot.load() == slot {
    //         self.remaining_tier_change_ticks.load()
    //     } else {
    //         0
    //     }
    // }

    // TODO GetTierChangeTargetTier
    // TODO GetRemainingTierChangeTicks
    // TODO CalculateProjectedEffectiveStructuralLoad
    // TODO GetCommonProjectedStructuralLoad
    // TODO StructuralLoadFor
    // TODO GetProjectedRawStructuralLoad

    // TODO ShipBalancing.CalculateGravity(CurrentEffectiveStructuralLoad)
    ///// Gravity emitted by the live runtime of this controllable.
    //#[inline]
    //pub fn gravity(&self) -> f32 {
    //    match self.specialization() {
    //        ControllableSpecialization::ClassicShip(_) => 0.0012,
    //        ControllableSpecialization::ModernShip(_) => 0.0012,
    //    }
    //}

    /// Collision radius of the live runtime of this controllable.
    #[inline]
    pub fn size(&self) -> f32 {
        match self.specialization() {
            ControllableSpecialization::ClassicShip(_) => 14.0,
            ControllableSpecialization::ModernShip(_) => ModernShipGeometry::RADIUS,
        }
    }

    /// Requests that this controllable enters the world again after initial registration or after a
    /// previous death.
    ///
    /// In Flattiverse, owning a [`Controllable`] and currently flying it are different lifecycle
    /// states. A newly registered ship exists as an owner-side controllable before it has an active
    /// in-world runtime, and a dead controllable also remains registered until it is finally closed.
    /// [`Controllable::r#continue`] is the command that asks the server to spawn or respawn that
    /// registered controllable.
    ///
    /// Spawn cluster, spawn position, revived runtime values, and all subsystem state are chosen
    /// authoritatively by the server. Do not infer the post-continue state locally. Instead, read
    /// the updated owner-side mirror afterwards via properties such as [`Controllable::alive`],
    /// [`Controllable::cluster`], [`Controllable::position`], and the subsystem objects on this
    /// controllable.
    ///
    /// A dead controllable can usually be continued repeatedly until
    /// [`Controllable::request_close`] has been issued and the server has performed the final
    /// close. Calling this on an already alive controllable is invalid.
    pub async fn r#continue(&self) -> Result<(), GameError> {
        self.cluster()
            .galaxy()
            .connection()
            .continue_controllable(self.id())
            .await
    }

    pub(crate) fn deactivate(&self) {
        self.active.store(false);
        self.alive.store(false);
        self.reset_runtime();
    }

    /// Requests self-destruction of the currently alive runtime of this controllable.
    /// The registration itself remains available afterwards until it is explicitly closed.
    pub async fn suicide(&self) -> Result<(), GameError> {
        self.cluster()
            .galaxy()
            .connection()
            .suicide_controllable(self.id())
            .await
    }

    /// Requests final closure of this controllable registration.
    /// The server may keep the controllable alive for a grace period before it is actually removed.
    ///
    /// This request is fire-and-forget and therefore does not wait for a reply. Observe subsequent
    /// events and [`Controllable::active`] to detect when the close has completed or await the
    /// returned [`Future`].
    pub async fn request_close(
        &self,
    ) -> Result<impl Future<Output = Result<(), GameError>>, GameError> {
        self.cluster()
            .galaxy()
            .connection()
            .request_controllable_close_split(self.id())
            .await
    }

    pub(crate) fn read_initial_state(&self, reader: &mut dyn PacketReader) {
        self.energy_battery.set_exists(reader.read_byte() != 0x00);
        if self.energy_battery.exists() {
            let energy_battery_tier = reader.read_byte();
            self.energy_battery.set_maximum(reader.read_f32());
            self.energy_battery.update_runtime(
                reader.read_f32(),
                reader.read_f32(),
                SubsystemStatus::read(reader),
            );
            self.energy_battery.set_reported_tier(energy_battery_tier);
        }

        self.ion_battery.set_exists(reader.read_byte() != 0x00);
        if self.ion_battery.exists() {
            let ion_battery_tier = reader.read_byte();
            self.ion_battery.set_maximum(reader.read_f32());
            self.ion_battery.update_runtime(
                reader.read_f32(),
                reader.read_f32(),
                SubsystemStatus::read(reader),
            );
            self.ion_battery.set_reported_tier(ion_battery_tier);
        }

        self.neutrino_battery.set_exists(reader.read_byte() != 0x00);
        if self.neutrino_battery.exists() {
            let neutrino_battery_tier = reader.read_byte();
            self.neutrino_battery.set_maximum(reader.read_f32());
            self.neutrino_battery.update_runtime(
                reader.read_f32(),
                reader.read_f32(),
                SubsystemStatus::read(reader),
            );
            self.neutrino_battery
                .set_reported_tier(neutrino_battery_tier);
        }

        self.energy_cell.set_exists(reader.read_byte() != 0x00);
        if self.energy_cell.exists() {
            let energy_cell_tier = reader.read_byte();
            self.energy_cell.set_efficiency(reader.read_f32());
            self.energy_cell
                .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
            self.energy_cell.set_reported_tier(energy_cell_tier);
        }

        self.ion_cell.set_exists(reader.read_byte() != 0x00);
        if self.ion_cell.exists() {
            let ion_cell_tier = reader.read_byte();
            self.ion_cell.set_efficiency(reader.read_f32());
            self.ion_cell
                .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
            self.ion_cell.set_reported_tier(ion_cell_tier);
        }

        self.neutrino_cell.set_exists(reader.read_byte() != 0x00);
        if self.neutrino_cell.exists() {
            let neutrino_cell_tier = reader.read_byte();
            self.neutrino_cell.set_efficiency(reader.read_f32());
            self.neutrino_cell
                .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
            self.neutrino_cell.set_reported_tier(neutrino_cell_tier);
        }

        self.hull.set_exists(reader.read_byte() != 0x00);
        if self.hull.exists() {
            let hull_tier = reader.read_byte();
            self.hull.set_maximum(reader.read_f32());
            self.hull
                .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
            self.hull.set_reported_tier(hull_tier);
        }

        self.shield.set_exists(reader.read_byte() != 0x00);
        if self.shield.exists() {
            let shield_tier = reader.read_byte();
            self.shield.set_maximum(reader.read_f32());
            self.shield
                .set_rate_capabilities(reader.read_f32(), reader.read_f32());
            self.shield.update_runtime(
                reader.read_f32(),
                reader.read_byte() != 0x00,
                reader.read_f32(),
                SubsystemStatus::read(reader),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
            );
            self.shield.set_reported_tier(shield_tier);
        }

        self.armor.set_exists(reader.read_byte() != 0x00);
        if self.armor.exists() {
            let armor_tier = reader.read_byte();
            self.armor.set_reduction(reader.read_f32());
            let armor_status = SubsystemStatus::read(reader);
            let armor_blocked_direct_damage_this_tick = reader.read_f32();
            let armor_blocked_radiation_damage_this_tick = reader.read_f32();
            self.armor.update_runtime(
                armor_blocked_direct_damage_this_tick,
                armor_blocked_radiation_damage_this_tick,
                armor_status,
            );
            self.armor.set_reported_tier(armor_tier);
        }

        self.repair.set_exists(reader.read_byte() != 0x00);
        if self.repair.exists() {
            let repair_tier = reader.read_byte();
            self.repair
                .set_capabilities(reader.read_f32(), reader.read_f32());
            self.repair.update_runtime(
                reader.read_f32(),
                SubsystemStatus::read(reader),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
            );
            self.repair.set_reported_tier(repair_tier);
        }

        self.cargo.set_exists(reader.read_byte() != 0x00);
        if self.cargo.exists() {
            let cargo_tier = reader.read_byte();
            self.cargo.set_maximums(
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
            );
            self.cargo.update_runtime(
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                SubsystemStatus::read(reader),
            );
            self.cargo.set_reported_tier(cargo_tier);
        }

        self.resource_miner.set_exists(reader.read_byte() != 0x00);
        if self.resource_miner.exists() {
            let resource_miner_tier = reader.read_byte();
            self.resource_miner
                .set_capabilities(reader.read_f32(), reader.read_f32());
            self.resource_miner.update_runtime(
                reader.read_f32(),
                SubsystemStatus::read(reader),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
            );
            self.resource_miner.set_reported_tier(resource_miner_tier);
        }

        self.structure_optimizer
            .set_exists(reader.read_byte() != 0x00);
        if self.structure_optimizer.exists() {
            let structure_optimizer_tier = reader.read_byte();
            let structure_optimizer_reduction_percentage = reader.read_f32();
            self.structure_optimizer
                .set_reduction_percentage(structure_optimizer_reduction_percentage);
            self.structure_optimizer
                .set_reported_tier(structure_optimizer_tier);
        }
    }

    pub(crate) fn deceased(&self) {
        self.alive.store(false);
        self.position.store_default();
        self.movement.store_default();
        self.angle.store_default();
        self.angular_velocity.store_default();
        self.reset_runtime();
    }

    pub(crate) fn update(self: &Arc<Self>, cluster: Arc<Cluster>, reader: &mut dyn PacketReader) {
        self.cluster.store(Arc::downgrade(&cluster));
        self.position.read(reader);
        self.movement.read(reader);
        self.angle.read(reader);
        self.angular_velocity.read(reader);

        self.alive.store(reader.read_byte() != 0x00);
        self.tier_change_pending.store(reader.read_byte() != 0x00);
        self.tier_change_slot.read(reader);
        self.tier_change_target_tier.store(reader.read_byte());
        self.remaining_tier_change_ticks.store(reader.read_uint16());

        if self.energy_battery.exists() {
            self.energy_battery.update_runtime(
                reader.read_f32(),
                reader.read_f32(),
                SubsystemStatus::read(reader),
            );
        }
        if self.ion_battery.exists() {
            self.ion_battery.update_runtime(
                reader.read_f32(),
                reader.read_f32(),
                SubsystemStatus::read(reader),
            );
        }
        if self.neutrino_battery.exists() {
            self.neutrino_battery.update_runtime(
                reader.read_f32(),
                reader.read_f32(),
                SubsystemStatus::read(reader),
            );
        }

        if self.energy_cell.exists() {
            self.energy_cell
                .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
        }
        if self.ion_cell.exists() {
            self.ion_cell
                .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
        }
        if self.neutrino_cell.exists() {
            self.neutrino_cell
                .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
        }

        if self.hull.exists() {
            self.hull
                .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
        }
        if self.shield.exists() {
            self.shield.update_runtime(
                reader.read_f32(),
                reader.read_byte() != 0,
                reader.read_f32(),
                SubsystemStatus::read(reader),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
            );
        }

        if self.armor.exists() {
            self.armor.update_runtime(
                reader.read_f32(),
                reader.read_f32(),
                SubsystemStatus::read(reader),
            );
        }
        if self.repair.exists() {
            self.repair.update_runtime(
                reader.read_f32(),
                SubsystemStatus::read(reader),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
            );
        }
        if self.cargo.exists() {
            self.cargo.update_runtime(
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                SubsystemStatus::read(reader),
            );
        }
        if self.resource_miner.exists() {
            self.resource_miner.update_runtime(
                reader.read_f32(),
                SubsystemStatus::read(reader),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
                reader.read_f32(),
            );
        }

        self.environment_heat_this_tick.read(reader);
        self.environment_heat_energy_cost_this_tick.read(reader);
        self.environment_heat_energy_overflow_this_tick.read(reader);
        self.environment_radiation_this_tick.read(reader);
        self.environment_radiation_damage_before_armor_this_tick
            .read(reader);
        self.environment_armor_blocked_damage_this_tick.read(reader);
        self.environment_hull_damage_this_tick.read(reader);

        self.read_runtime(reader);
        self.alive.store(true);
        self.emit_runtime_events();
    }

    pub(crate) fn reset_runtime(&self) {
        self.energy_battery.reset_runtime();
        self.ion_battery.reset_runtime();
        self.neutrino_battery.reset_runtime();
        self.energy_cell.reset_runtime();
        self.ion_cell.reset_runtime();
        self.neutrino_cell.reset_runtime();
        self.hull.reset_runtime();
        self.shield.reset_runtime();
        self.armor.reset_runtime();
        self.repair.reset_runtime();
        self.cargo.reset_runtime();
        self.resource_miner.reset_runtime();
        self.environment_heat_this_tick.store_default();
        self.environment_heat_energy_cost_this_tick.store_default();
        self.environment_heat_energy_overflow_this_tick
            .store_default();
        self.environment_radiation_this_tick.store_default();
        self.environment_radiation_damage_before_armor_this_tick
            .store_default();
        self.environment_armor_blocked_damage_this_tick
            .store_default();
        self.environment_hull_damage_this_tick.store_default();

        match self.specialization() {
            ControllableSpecialization::ClassicShip(s) => s.reset_runtime(),
            ControllableSpecialization::ModernShip(s) => s.reset_runtime(),
        }
    }

    pub(crate) fn read_runtime(&self, reader: &mut dyn PacketReader) {
        match self.specialization() {
            ControllableSpecialization::ClassicShip(s) => s.read_runtime(reader),
            ControllableSpecialization::ModernShip(s) => s.read_runtime(reader),
        }
    }

    pub(crate) fn emit_runtime_events(self: &Arc<Self>) {
        self.push_runtime_events(
            [
                self.energy_battery.create_runtime_event(),
                self.ion_battery.create_runtime_event(),
                self.neutrino_battery.create_runtime_event(),
                self.energy_cell.create_runtime_event(),
                self.ion_cell.create_runtime_event(),
                self.neutrino_cell.create_runtime_event(),
                self.hull.create_runtime_event(),
                self.shield.create_runtime_event(),
                self.armor.create_runtime_event(),
                self.repair.create_runtime_event(),
                self.cargo.create_runtime_event(),
                self.resource_miner.create_runtime_event(),
                Arc::clone(self).create_environment_runtime_event(),
            ]
            .into_iter()
            .flatten(),
        );

        match self.specialization() {
            ControllableSpecialization::ClassicShip(specialization) => {
                self.push_runtime_events(specialization.iter_runtime_events());
            }
            ControllableSpecialization::ModernShip(ship) => {
                self.push_runtime_events(ship.iter_runtime_events())
            }
        }
    }

    pub(crate) fn push_runtime_events(&self, events: impl Iterator<Item = FlattiverseEvent>) {
        match self.cluster().galaxy().connection().event_sender.upgrade() {
            Some(sender) => {
                for event in events {
                    if let Err(e) = sender.try_send(event) {
                        warn!("Failed to push event {e:?}");
                    }
                }
            }
            None => {
                warn!("Can no longer push FlattiversEvents, Sender is gone!")
            }
        }
    }

    pub(crate) fn create_environment_runtime_event(self: Arc<Self>) -> Option<FlattiverseEvent> {
        let environment_heat_this_tick = self.environment_heat_this_tick();
        let environment_heat_energy_cost_this_tick = self.environment_heat_energy_cost_this_tick();
        let environment_heat_energy_overflow_this_tick =
            self.environment_heat_energy_overflow_this_tick();
        let environment_radiation_this_tick = self.environment_radiation_this_tick();
        let environment_radiation_damage_before_armor_this_tick =
            self.environment_radiation_damage_before_armor_this_tick();
        let environment_armor_blocked_damage_this_tick =
            self.environment_armor_blocked_damage_this_tick();
        let environment_hull_damage_this_tick = self.environment_hull_damage_this_tick();

        if environment_heat_this_tick == 0.0
            && environment_heat_energy_cost_this_tick == 0.0
            && environment_heat_energy_overflow_this_tick == 0.0
            && environment_radiation_this_tick == 0.0
            && environment_radiation_damage_before_armor_this_tick == 0.0
            && environment_armor_blocked_damage_this_tick == 0.0
            && environment_hull_damage_this_tick == 0.0
        {
            None
        } else {
            Some(
                FlattiverseEventKind::EnvironmentDamage {
                    controllable: self,
                    heat: environment_heat_this_tick,
                    heat_energy_cost: environment_heat_energy_cost_this_tick,
                    heat_energy_overflow: environment_heat_energy_overflow_this_tick,
                    radiation: environment_radiation_this_tick,
                    radiation_damage_before_armor:
                        environment_radiation_damage_before_armor_this_tick,
                    armor_blocked_damage: environment_armor_blocked_damage_this_tick,
                    hull_damage: environment_hull_damage_this_tick,
                }
                .into(),
            )
        }
    }

    pub fn specialization(&self) -> &ControllableSpecialization {
        &self.specialization
    }

    #[inline]
    pub fn try_into_controls<T>(self: Arc<Self>) -> Result<Controls<T>, Arc<Self>>
    where
        Controls<T>: TryFrom<Arc<Self>, Error = Arc<Self>>,
    {
        Controls::<T>::try_from(self)
    }
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum ControllableSpecialization {
    ClassicShip(ClassicShipControllable),
    ModernShip(ModernShipControllable),
}

pub(crate) struct Proven<T>(PhantomData<T>);

impl<T> Proven<T> {
    pub(crate) fn control(self, controllable: Arc<Controllable>) -> Controls<T> {
        Controls {
            controllable,
            _specialization: PhantomData,
        }
    }
}

/// Wrapper to easily access the specialization for a controllable, once proven.
#[derive(Debug)]
pub struct Controls<T> {
    controllable: Arc<Controllable>,
    _specialization: PhantomData<T>,
}

impl<T> Controls<T> {
    #[inline]
    pub fn controllable(&self) -> &Arc<Controllable> {
        &self.controllable
    }
}

impl<T> Clone for Controls<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            controllable: self.controllable.clone(),
            _specialization: PhantomData,
        }
    }
}

impl<T> Controls<T> {
    #[inline(always)]
    pub(crate) fn proven(t: &T) -> Proven<T> {
        let _ = t;
        Proven(PhantomData)
    }
}

impl<T> Deref for Controls<T> {
    type Target = Controllable;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.controllable
    }
}

impl Identifiable<ControllableId> for Controllable {
    #[inline]
    fn id(&self) -> ControllableId {
        Controllable::id(self)
    }
}
