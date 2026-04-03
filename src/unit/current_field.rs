use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractSteadyUnit, CurrentFieldMode, SteadyUnit, SteadyUnitInternal, Unit, UnitCastTable,
    UnitHierarchy, UnitInternal, UnitKind,
};
use crate::utils::Atomic;
use crate::{GameError, Vector};
use std::sync::{Arc, Weak};

/// A non-solid field that induces movement on mobile units.
#[derive(Debug, Clone)]
pub struct CurrentField {
    parent: AbstractSteadyUnit,
    mode: Atomic<CurrentFieldMode>,
    flow: Atomic<Vector>,
    radial_force: Atomic<f32>,
    tangential_force: Atomic<f32>,
}

impl CurrentField {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractSteadyUnit::new(cluster, name, reader)?,
            mode: Atomic::from(CurrentFieldMode::Directional),
            flow: Atomic::default(),
            radial_force: Atomic::from(0.0),
            tangential_force: Atomic::from(0.0),
        }))
    }

    /// Current-field mode.
    #[inline]
    pub fn mode(&self) -> CurrentFieldMode {
        self.mode.load()
    }

    /// Fixed world-space movement vector for directional fields.
    #[inline]
    pub fn flow(&self) -> Vector {
        self.flow.load()
    }

    /// Radial movement component for relative fields.
    #[inline]
    pub fn radial_force(&self) -> f32 {
        self.radial_force.load()
    }

    /// Tangential movement component for relative fields.
    #[inline]
    pub fn tangential_force(&self) -> f32 {
        self.tangential_force.load()
    }
}

impl UnitInternal for CurrentField {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.mode.read(reader);
        self.flow.read(reader);
        self.radial_force.read(reader);
        self.tangential_force.read(reader);
    }
}

impl UnitCastTable for CurrentField {
    cast_fn!(steady_unit_cast_fn, CurrentField, dyn SteadyUnit);
}

impl UnitHierarchy for CurrentField {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_current_field(&self) -> Option<&CurrentField> {
        Some(self)
    }
}

impl Unit for CurrentField {
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
        UnitKind::CurrentField
    }
}

impl SteadyUnitInternal for CurrentField {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for CurrentField {}
