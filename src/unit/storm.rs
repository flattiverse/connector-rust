use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractSteadyUnit, SteadyUnit, SteadyUnitInternal, Unit, UnitCastTable, UnitHierarchy,
    UnitInternal, UnitKind,
};
use crate::utils::Atomic;
use crate::GameError;
use std::sync::{Arc, Weak};

#[derive(Debug, Clone)]
pub struct Storm {
    parent: AbstractSteadyUnit,
    spawn_chance_per_tick: Atomic<f32>,
    min_announcement_ticks: Atomic<u16>,
    max_announcement_ticks: Atomic<u16>,
    min_active_ticks: Atomic<u16>,
    max_active_ticks: Atomic<u16>,
    min_whirl_radius: Atomic<f32>,
    max_whirl_radius: Atomic<f32>,
    min_whirl_speed: Atomic<f32>,
    max_whirl_speed: Atomic<f32>,
    min_whirl_gravity: Atomic<f32>,
    max_whirl_gravity: Atomic<f32>,
    damage: Atomic<f32>,
}

impl Storm {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractSteadyUnit::new(cluster, name, reader)?,
            spawn_chance_per_tick: Atomic::from(0.0),
            min_announcement_ticks: Atomic::from(0),
            max_announcement_ticks: Atomic::from(0),
            min_active_ticks: Atomic::from(0),
            max_active_ticks: Atomic::from(0),
            min_whirl_radius: Atomic::from(0.0),
            max_whirl_radius: Atomic::from(0.0),
            min_whirl_speed: Atomic::from(0.0),
            max_whirl_speed: Atomic::from(0.0),
            min_whirl_gravity: Atomic::from(0.0),
            max_whirl_gravity: Atomic::from(0.0),
            damage: Atomic::from(0.0),
        }))
    }

    /// Probability in the range `[0; 1]` that the storm spawns one announcing whirl in a tick.
    #[inline]
    pub fn spawn_chance_per_tick(&self) -> f32 {
        self.spawn_chance_per_tick.load()
    }

    /// Minimum announcement duration for newly spawned whirls.
    #[inline]
    pub fn min_announcement_ticks(&self) -> u16 {
        self.min_announcement_ticks.load()
    }

    /// Maximum announcement duration for newly spawned whirls.
    #[inline]
    pub fn max_announcement_ticks(&self) -> u16 {
        self.max_announcement_ticks.load()
    }

    /// Minimum active duration for newly spawned whirls.
    #[inline]
    pub fn min_active_ticks(&self) -> u16 {
        self.min_active_ticks.load()
    }

    /// Maximum active duration for newly spawned whirls.
    #[inline]
    pub fn max_active_ticks(&self) -> u16 {
        self.max_active_ticks.load()
    }

    /// Minimum radius used for newly spawned whirls.
    #[inline]
    pub fn min_whirl_radius(&self) -> f32 {
        self.min_whirl_radius.load()
    }

    /// Maximum radius used for newly spawned whirls.
    #[inline]
    pub fn max_whirl_radius(&self) -> f32 {
        self.max_whirl_radius.load()
    }

    /// Minimum initial speed used for newly spawned whirls.
    #[inline]
    pub fn min_whirl_speed(&self) -> f32 {
        self.min_whirl_speed.load()
    }

    /// Maximum initial speed used for newly spawned whirls.
    #[inline]
    pub fn max_whirl_speed(&self) -> f32 {
        self.max_whirl_speed.load()
    }

    /// Minimum gravity used for active whirls.
    #[inline]
    pub fn min_whirl_gravity(&self) -> f32 {
        self.min_whirl_gravity.load()
    }

    /// Maximum gravity used for active whirls.
    #[inline]
    pub fn max_whirl_gravity(&self) -> f32 {
        self.max_whirl_gravity.load()
    }

    /// Damage applied by each active-whirl hit.
    #[inline]
    pub fn damage(&self) -> f32 {
        self.damage.load()
    }
}

impl UnitInternal for Storm {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.spawn_chance_per_tick.read(reader);
        self.min_announcement_ticks.read(reader);
        self.max_announcement_ticks.read(reader);
        self.min_active_ticks.read(reader);
        self.max_active_ticks.read(reader);
        self.min_whirl_radius.read(reader);
        self.max_whirl_radius.read(reader);
        self.min_whirl_speed.read(reader);
        self.max_whirl_speed.read(reader);
        self.min_whirl_gravity.read(reader);
        self.max_whirl_gravity.read(reader);
        self.damage.read(reader);
    }
}

impl UnitCastTable for Storm {
    cast_fn!(steady_unit_cast_fn, Storm, dyn SteadyUnit);
}

impl UnitHierarchy for Storm {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_storm(&self) -> Option<&Storm> {
        Some(self)
    }
}

impl Unit for Storm {
    #[inline]
    fn is_masking(&self) -> bool {
        false
    }

    #[inline]
    fn is_solid(&self) -> bool {
        false
    }

    #[inline]
    fn can_be_edited(&self) -> bool {
        true
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Storm
    }
}

impl SteadyUnitInternal for Storm {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for Storm {}
