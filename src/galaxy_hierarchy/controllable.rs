use crate::galaxy_hierarchy::{
    AsSubsystemBase, BatterySubsystem, ClassicShipControllable, Cluster, EnergyCellSubsystem,
    HullSubsystem, Identifiable, Indexer, ShieldSubsystem,
};
use crate::network::{InvalidArgumentKind, PacketReader};
use crate::unit::UnitKind;
use crate::utils::{Also, Atomic, Readable};
use crate::{FlattiverseEvent, GameError, GameErrorKind, SubsystemSlot, SubsystemStatus, Vector};
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
    cluster: Weak<Cluster>,
    active: Atomic<bool>,
    alive: Atomic<bool>,
    position: Atomic<Vector>,
    movement: Atomic<Vector>,
    hull: HullSubsystem,
    shield: ShieldSubsystem,
    energy_battery: BatterySubsystem,
    ion_battery: BatterySubsystem,
    neutrino_battery: BatterySubsystem,
    energy_cell: EnergyCellSubsystem,
    ion_cell: EnergyCellSubsystem,
    neutrino_cell: EnergyCellSubsystem,
    specialization: ControllableSpecialization,
}

impl Controllable {
    pub(crate) fn from_packet(
        kind: UnitKind,
        cluster: Weak<Cluster>,
        id: ControllableId,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        match kind {
            UnitKind::ClassicShipPlayerUnit => Ok(Arc::new(Self {
                name,
                id,
                cluster,
                active: Atomic::from(true),
                alive: Atomic::from(false),
                position: Atomic::from_reader(reader),
                movement: Atomic::from_reader(reader),
                hull: HullSubsystem::create_classic_ship_hull(Weak::default()),
                shield: ShieldSubsystem::create_classic_ship_shield(Weak::default()),
                energy_battery: BatterySubsystem::create_classic_ship_energy_battery(
                    Weak::default(),
                ),
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
                specialization: ControllableSpecialization::ClassicShip(
                    ClassicShipControllable::new(),
                ),
            })
            .also(|this| {
                // finish the initialization of cross-references
                for subsystem in [
                    this.hull.as_subsystem_base(),
                    this.shield.as_subsystem_base(),
                    this.energy_battery.as_subsystem_base(),
                    this.ion_battery.as_subsystem_base(),
                    this.neutrino_battery.as_subsystem_base(),
                    this.energy_cell.as_subsystem_base(),
                    this.ion_cell.as_subsystem_base(),
                    this.neutrino_cell.as_subsystem_base(),
                ]
                .into_iter()
                .chain(match this.specialization() {
                    ControllableSpecialization::ClassicShip(c) => c.iter_subsystem_bases(),
                }) {
                    subsystem.controllable.store(Arc::downgrade(this));
                }
            })),
            _ => Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::Unknown(Default::default()),
                parameter: "kind".to_string(),
            }
            .into()),
        }
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

    /// The cluster this unit currently is in.
    #[inline]
    pub fn cluster(&self) -> Arc<Cluster> {
        self.cluster.upgrade().unwrap()
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

    /// true, if the unit is alive.
    #[inline]
    pub fn alive(&self) -> bool {
        self.alive.load()
    }

    /// true if this object still can be used. If the unit has been finally closed this is false.
    #[inline]
    pub fn active(&self) -> bool {
        self.active.load()
    }

    /// The gravity this controllable has.
    #[inline]
    pub fn gravity(&self) -> f32 {
        match self.specialization() {
            ControllableSpecialization::ClassicShip(_) => 0.0012,
        }
    }

    /// The size (radius) of the controllable.
    #[inline]
    pub fn size(&self) -> f32 {
        match self.specialization() {
            ControllableSpecialization::ClassicShip(_) => 14.0,
        }
    }

    /// Call this to continue the game with this unit after you are dead or when you hve created the
    /// unit.
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

    /// Call this to suicide (=self destroy).
    pub async fn suicide(&self) -> Result<(), GameError> {
        self.cluster()
            .galaxy()
            .connection()
            .suicide_controllable(self.id())
            .await
    }

    /// Call this to request closing the unit. The server may keep it alive for a grace period
    /// before it is finally removed.
    pub async fn request_close(&self) -> Result<(), GameError> {
        self.cluster()
            .galaxy()
            .connection()
            .request_controllable_close(self.id())
            .await
    }

    pub(crate) fn deceased(&self) {
        self.alive.store(false);
        self.position.store_default();
        self.movement.store_default();
        self.reset_runtime();
    }

    pub(crate) fn update(&self, reader: &mut dyn PacketReader) {
        self.position.read(reader);
        self.movement.read(reader);

        self.energy_battery.update_runtime(
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );
        self.ion_battery.update_runtime(
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );
        self.neutrino_battery.update_runtime(
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );

        self.energy_cell
            .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
        self.ion_cell
            .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
        self.neutrino_cell
            .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));

        self.hull
            .update_runtime(reader.read_f32(), SubsystemStatus::read(reader));
        self.shield.update_runtime(
            reader.read_f32(),
            reader.read_byte() != 0,
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );

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

        match self.specialization() {
            ControllableSpecialization::ClassicShip(s) => s.reset_runtime(),
        }
    }

    pub(crate) fn read_runtime(&self, reader: &mut dyn PacketReader) {
        match self.specialization() {
            ControllableSpecialization::ClassicShip(s) => s.read_runtime(reader),
        }
    }

    pub(crate) fn emit_runtime_events(&self) {
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
            ]
            .into_iter()
            .flatten(),
        );

        match self.specialization() {
            ControllableSpecialization::ClassicShip(specialization) => {
                self.push_runtime_events(specialization.iter_runtime_events());
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
pub enum ControllableSpecialization {
    ClassicShip(ClassicShipControllable),
}

pub(crate) struct Proven<T>(PhantomData<T>);

impl<T> Proven<T> {
    pub(crate) fn control(
        self,
        controllable: Arc<Controllable>,
    ) -> Controls<ClassicShipControllable> {
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
