use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractSteadyUnit, SteadyUnit, SteadyUnitInternal, Unit, UnitHierarchy, UnitInternal, UnitKind,
};
use crate::GameError;
use arc_swap::{ArcSwapOption, Guard};
use std::sync::{Arc, Weak};

/// A buoy.
#[derive(Debug)]
pub struct Buoy {
    parent: AbstractSteadyUnit,
    message: ArcSwapOption<String>,
}

impl Clone for Buoy {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            message: ArcSwapOption::new(self.message.load_full()),
        }
    }
}

impl Buoy {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(Self {
            parent: AbstractSteadyUnit::new(cluster, name, reader)?,
            message: ArcSwapOption::default(),
        }))
    }

    /// Optional buoy message. [`None`] means no message.
    #[inline]
    pub fn message(&self) -> Guard<Option<Arc<String>>> {
        self.message.load()
    }
}

impl UnitInternal for Buoy {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.message.store(reader.opt_read_string().map(Arc::new));
    }
}

impl UnitHierarchy for Buoy {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_buoy(&self) -> Option<&Buoy> {
        Some(self)
    }
}

impl Unit for Buoy {
    #[inline]
    fn is_masking(&self) -> bool {
        false
    }

    #[inline]
    fn is_solid(&self) -> bool {
        false
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Buoy
    }
}

impl SteadyUnitInternal for Buoy {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for Buoy {}
