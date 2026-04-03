use crate::galaxy_hierarchy::{Cluster, Team, TeamId};
use crate::network::PacketReader;
use crate::unit::{
    AbstractSteadyUnit, SteadyUnit, SteadyUnitInternal, SwitchMode, Unit, UnitCastTable,
    UnitHierarchy, UnitInternal, UnitKind,
};
use crate::utils::{Atomic, Let};
use crate::GameError;
use arc_swap::ArcSwapWeak;
use std::sync::{Arc, Weak};

/// A triggerable switch that affects linked gates.
#[derive(Debug)]
pub struct Switch {
    parent: AbstractSteadyUnit,
    team: ArcSwapWeak<Team>,
    link_id: Atomic<u16>,
    range: Atomic<f32>,
    cooldown_ticks: Atomic<i32>,
    cooldown_remaining_ticks: Atomic<i32>,
    mode: Atomic<SwitchMode>,
    switched: Atomic<bool>,
}

impl Clone for Switch {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            team: ArcSwapWeak::new(self.team.load_full()),
            link_id: self.link_id.clone(),
            range: self.range.clone(),
            cooldown_ticks: self.cooldown_ticks.clone(),
            cooldown_remaining_ticks: self.cooldown_remaining_ticks.clone(),
            mode: self.mode.clone(),
            switched: self.switched.clone(),
        }
    }
}

impl Switch {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(
            AbstractSteadyUnit::new(cluster, name, reader)?.r#let(|parent| Self {
                team: ArcSwapWeak::new(Arc::downgrade(
                    &parent
                        .cluster()
                        .galaxy()
                        .get_team(TeamId(reader.read_byte())),
                )),
                link_id: Atomic::from_reader(reader),
                range: Atomic::from_reader(reader),
                cooldown_ticks: Atomic::from_reader(reader),
                mode: Atomic::from_reader(reader),
                cooldown_remaining_ticks: Atomic::from(0),
                switched: Atomic::from(false),
                parent,
            }),
        ))
    }

    /// Link id shared with linked gates.
    #[inline]
    pub fn link_id(&self) -> u16 {
        self.link_id.load()
    }

    /// Search radius for linked gates.
    #[inline]
    pub fn range(&self) -> f32 {
        self.range.load()
    }

    /// Configured switch cooldown in ticks.
    #[inline]
    pub fn cooldown_ticks(&self) -> i32 {
        self.cooldown_ticks.load()
    }

    /// Remaining runtime cooldown in ticks.
    #[inline]
    pub fn cooldown_remaining_ticks(&self) -> i32 {
        self.cooldown_remaining_ticks.load()
    }

    /// Switch output mode.
    #[inline]
    pub fn mode(&self) -> SwitchMode {
        self.mode.load()
    }

    /// Runtime toggled state of the switch itself.
    #[inline]
    pub fn switched(&self) -> bool {
        self.switched.load()
    }
}

impl UnitInternal for Switch {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    #[inline]
    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.cooldown_remaining_ticks.read(reader);
        self.switched.store(reader.read_byte() != 0x00)
    }
}

impl UnitCastTable for Switch {
    cast_fn!(steady_unit_cast_fn, Switch, dyn SteadyUnit);
}

impl UnitHierarchy for Switch {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_switch(&self) -> Option<&Switch> {
        Some(self)
    }
}

impl Unit for Switch {
    #[inline]
    fn can_be_edited(&self) -> bool {
        true
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Switch
    }

    #[inline]
    fn team(&self) -> Weak<Team> {
        self.team.load_full()
    }
}

impl SteadyUnitInternal for Switch {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for Switch {}
