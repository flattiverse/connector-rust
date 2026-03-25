use crate::galaxy_hierarchy::{
    ControllableInfo, ControllableInfoId, Galaxy, Player, PlayerId, Team,
};
use crate::network::PacketReader;
use crate::unit::{
    BatterySubsystemInfo, EnergyCellSubsystemInfo, HullSubsystemInfo, Mobility,
    ShieldSubsystemInfo, UnitBase, UnitExt, UnitExtSealed,
};
use crate::utils::{Atomic, Readable};
use crate::{SubsystemStatus, Vector};
use std::sync::{Arc, Weak};

/// Represents a player unit.
#[derive(Debug, Clone)]
pub struct PlayerUnit {
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

impl PlayerUnit {
    pub(crate) fn read(galaxy: &Galaxy, reader: &mut dyn PacketReader) -> Self {
        let player_id = PlayerId(reader.read_byte());
        let controllable_info_id = ControllableInfoId(reader.read_byte());

        let player = galaxy.get_player(player_id);
        let controllable_info = player.get_controllable_info(controllable_info_id);

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
        }
    }

    /// Represents the player which controls the PlayerUnit.
    #[inline]
    pub fn player(&self) -> Arc<Player> {
        self.player.upgrade().unwrap()
    }

    /// Represents the ControllableInfo of this PlayerUnit.
    #[inline]
    pub fn controllable_info(&self) -> Arc<ControllableInfo> {
        self.controllable_info.upgrade().unwrap()
    }

    /// Visible snapshot of the energy battery subsystem.
    #[inline]
    pub fn energy_battery(&self) -> &BatterySubsystemInfo {
        &self.energy_battery
    }

    /// Visible snapshot of the ion battery subsystem.
    #[inline]
    pub fn ion_battery(&self) -> &BatterySubsystemInfo {
        &self.ion_battery
    }

    /// Visible snapshot of the neutrino battery subsystem.
    #[inline]
    pub fn neutrino_battery(&self) -> &BatterySubsystemInfo {
        &self.neutrino_battery
    }

    /// Visible snapshot of the energy cell subsystem.
    #[inline]
    pub fn energy_cell(&self) -> &EnergyCellSubsystemInfo {
        &self.energy_cell
    }

    /// Visible snapshot of the ion cell subsystem.
    #[inline]
    pub fn ion_cell(&self) -> &EnergyCellSubsystemInfo {
        &self.ion_cell
    }

    /// Visible snapshot of the neutrino cell subsystem.
    #[inline]
    pub fn neutrino_cell(&self) -> &EnergyCellSubsystemInfo {
        &self.neutrino_cell
    }

    /// Visible snapshot of the hull subsystem.
    #[inline]
    pub fn hull(&self) -> &HullSubsystemInfo {
        &self.hull
    }

    /// Visible snapshot of the shield subsystem.
    #[inline]
    pub fn shield(&self) -> &ShieldSubsystemInfo {
        &self.shield
    }
}

impl<'a> UnitExtSealed<'a> for (&'a UnitBase, &'a PlayerUnit)
where
    Self: 'a,
{
    type Parent = &'a UnitBase;

    #[inline]
    fn parent(self) -> Self::Parent {
        self.0
    }

    #[inline]
    fn update_movement(self, reader: &mut dyn PacketReader) {
        self.parent().update_movement(reader);

        self.1.position.read(reader);
        self.1.movement.read(reader);
    }

    #[inline]
    fn update_state(self, reader: &mut dyn PacketReader) {
        self.parent().update_state(reader);

        self.1.energy_battery.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );
        self.1.ion_battery.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );
        self.1.neutrino_battery.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );

        self.1.energy_cell.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );
        self.1.ion_cell.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );
        self.1.neutrino_cell.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );

        self.1.hull.update(
            reader.read_byte() != 0,
            reader.read_f32(),
            reader.read_f32(),
            SubsystemStatus::read(reader),
        );

        self.1.shield.update(
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

impl<'b> UnitExt<'b> for (&'b UnitBase, &'b PlayerUnit) {
    #[inline]
    fn position(self) -> Vector {
        self.1.position.load()
    }

    #[inline]
    fn movement(self) -> Vector {
        self.1.movement.load()
    }

    #[inline]
    fn angle(self) -> f32 {
        self.1.movement.load().angle()
    }

    #[inline]
    fn mobility(self) -> Mobility {
        Mobility::Mobile
    }

    #[inline]
    fn team(self) -> Weak<Team> {
        self.1.player().team_weak()
    }
}
