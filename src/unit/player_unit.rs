use crate::galaxy_hierarchy::{
    Cluster, ControllableInfo, ControllableInfoId, Player, PlayerId, Team,
};
use crate::network::PacketReader;
use crate::unit::{
    AbstractUnit, BatterySubsystemInfo, EnergyCellSubsystemInfo, HullSubsystemInfo, Mobility,
    ShieldSubsystemInfo, Unit, UnitHierarchy, UnitInternal,
};
use crate::utils::{Atomic, Let, Readable};
use crate::{GameError, SubsystemStatus, Vector};
use std::sync::{Arc, Weak};

pub(crate) trait PlayerUnitInternal {
    fn parent(&self) -> &dyn PlayerUnit;
}

/// Represents a player unit.
#[allow(private_bounds)]
pub trait PlayerUnit: PlayerUnitInternal + Unit {
    /// Represents the player which controls the PlayerUnit.
    #[inline]
    fn player(&self) -> Arc<Player> {
        PlayerUnitInternal::parent(self).player()
    }

    /// Represents the ControllableInfo of this PlayerUnit.
    #[inline]
    fn controllable_info(&self) -> Arc<ControllableInfo> {
        PlayerUnitInternal::parent(self).controllable_info()
    }

    /// Visible snapshot of the energy battery subsystem.
    #[inline]
    fn energy_battery(&self) -> &BatterySubsystemInfo {
        PlayerUnitInternal::parent(self).energy_battery()
    }

    /// Visible snapshot of the ion battery subsystem.
    #[inline]
    fn ion_battery(&self) -> &BatterySubsystemInfo {
        PlayerUnitInternal::parent(self).ion_battery()
    }

    /// Visible snapshot of the neutrino battery subsystem.
    #[inline]
    fn neutrino_battery(&self) -> &BatterySubsystemInfo {
        PlayerUnitInternal::parent(self).neutrino_battery()
    }

    /// Visible snapshot of the energy cell subsystem.
    #[inline]
    fn energy_cell(&self) -> &EnergyCellSubsystemInfo {
        PlayerUnitInternal::parent(self).energy_cell()
    }

    /// Visible snapshot of the ion cell subsystem.
    #[inline]
    fn ion_cell(&self) -> &EnergyCellSubsystemInfo {
        PlayerUnitInternal::parent(self).ion_cell()
    }

    /// Visible snapshot of the neutrino cell subsystem.
    #[inline]
    fn neutrino_cell(&self) -> &EnergyCellSubsystemInfo {
        PlayerUnitInternal::parent(self).neutrino_cell()
    }

    /// Visible snapshot of the hull subsystem.
    #[inline]
    fn hull(&self) -> &HullSubsystemInfo {
        PlayerUnitInternal::parent(self).hull()
    }

    /// Visible snapshot of the shield subsystem.
    #[inline]
    fn shield(&self) -> &ShieldSubsystemInfo {
        PlayerUnitInternal::parent(self).shield()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AbstractPlayerUnit {
    parent: AbstractUnit,
    player: Weak<Player>,
    controllable_info: Weak<ControllableInfo>,
    position: Atomic<Vector>,
    movement: Atomic<Vector>,
    energy_battery: BatterySubsystemInfo,
    ion_battery: BatterySubsystemInfo,
    neutrino_battery: BatterySubsystemInfo,
    energy_cell: EnergyCellSubsystemInfo,
    ion_cell: EnergyCellSubsystemInfo,
    neutrino_cell: EnergyCellSubsystemInfo,
    hull: HullSubsystemInfo,
    shield: ShieldSubsystemInfo,
}

impl AbstractPlayerUnit {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Self, GameError> {
        Ok(AbstractUnit::new(cluster, name).r#let(|parent| {
            let player = parent
                .cluster()
                .galaxy()
                .get_player(PlayerId(reader.read_byte()));

            let controllable_info =
                player.get_controllable_info(ControllableInfoId(reader.read_byte()));

            Self {
                player: Arc::downgrade(&player),
                controllable_info: Arc::downgrade(&controllable_info),
                position: Atomic::from_reader(reader),
                movement: Atomic::from_reader(reader),
                energy_battery: BatterySubsystemInfo::default(),
                ion_battery: BatterySubsystemInfo::default(),
                neutrino_battery: BatterySubsystemInfo::default(),
                energy_cell: EnergyCellSubsystemInfo::default(),
                ion_cell: EnergyCellSubsystemInfo::default(),
                neutrino_cell: EnergyCellSubsystemInfo::default(),
                hull: HullSubsystemInfo::default(),
                shield: ShieldSubsystemInfo::default(),
                parent,
            }
        }))
    }
}

impl UnitInternal for AbstractPlayerUnit {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_movement(&self, reader: &mut dyn PacketReader) {
        self.parent.update_movement(reader);

        self.position.read(reader);
        self.movement.read(reader);
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.energy_battery.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );
        self.ion_battery.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );
        self.neutrino_battery.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );

        self.energy_cell.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );
        self.ion_cell.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );
        self.neutrino_cell.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );

        self.hull.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );

        self.shield.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_byte() != 0,
            reader.read_f32(),
            SubsystemStatus::read(reader),
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
        );
    }
}

impl UnitHierarchy for AbstractPlayerUnit {
    #[inline]
    fn as_player_unit(&self) -> Option<&dyn PlayerUnit> {
        Some(self)
    }
}

impl Unit for AbstractPlayerUnit {
    #[inline]
    fn position(&self) -> Vector {
        self.position.load()
    }

    #[inline]
    fn movement(&self) -> Vector {
        self.movement.load()
    }

    #[inline]
    fn angle(&self) -> f32 {
        self.movement.load().angle()
    }

    #[inline]
    fn mobility(&self) -> Mobility {
        Mobility::Mobile
    }

    #[inline]
    fn team(&self) -> Weak<Team> {
        self.player().team_weak()
    }
}

impl PlayerUnitInternal for AbstractPlayerUnit {
    #[inline]
    fn parent(&self) -> &dyn PlayerUnit {
        unreachable!()
    }
}

#[forbid(clippy::missing_trait_methods)]
impl PlayerUnit for AbstractPlayerUnit {
    #[inline]
    fn player(&self) -> Arc<Player> {
        self.player.upgrade().unwrap()
    }

    #[inline]
    fn controllable_info(&self) -> Arc<ControllableInfo> {
        self.controllable_info.upgrade().unwrap()
    }

    #[inline]
    fn energy_battery(&self) -> &BatterySubsystemInfo {
        &self.energy_battery
    }

    #[inline]
    fn ion_battery(&self) -> &BatterySubsystemInfo {
        &self.ion_battery
    }

    #[inline]
    fn neutrino_battery(&self) -> &BatterySubsystemInfo {
        &self.neutrino_battery
    }

    #[inline]
    fn energy_cell(&self) -> &EnergyCellSubsystemInfo {
        &self.energy_cell
    }

    #[inline]
    fn ion_cell(&self) -> &EnergyCellSubsystemInfo {
        &self.ion_cell
    }

    #[inline]
    fn neutrino_cell(&self) -> &EnergyCellSubsystemInfo {
        &self.neutrino_cell
    }

    #[inline]
    fn hull(&self) -> &HullSubsystemInfo {
        &self.hull
    }

    #[inline]
    fn shield(&self) -> &ShieldSubsystemInfo {
        &self.shield
    }
}
