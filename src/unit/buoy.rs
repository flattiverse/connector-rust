use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::Readable;
use arc_swap::{ArcSwapOption, Guard};
use std::sync::{Arc, Weak};

/// A buoy.
#[derive(Debug)]
pub struct Buoy {
    base: UnitBase,
    steady: SteadyUnit,
    message: ArcSwapOption<String>,
}

impl Clone for Buoy {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            steady: self.steady.clone(),
            message: ArcSwapOption::new(self.message.load_full()),
        }
    }
}

impl Buoy {
    pub(crate) fn read(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Self {
        Self {
            base: UnitBase::new(cluster, name),
            steady: SteadyUnit::read(reader),
            message: ArcSwapOption::default(),
        }
    }

    /// Optional buoy message. [None] means no message.
    #[inline]
    pub fn message(&self) -> Guard<Option<Arc<String>>> {
        self.message.load()
    }
}

impl AsRef<UnitBase> for Buoy {
    #[inline]
    fn as_ref(&self) -> &UnitBase {
        &self.base
    }
}

impl AsRef<SteadyUnit> for Buoy {
    #[inline]
    fn as_ref(&self) -> &SteadyUnit {
        &self.steady
    }
}

impl<'a> UnitExtSealed<'a> for &'a Buoy {
    type Parent = (&'a UnitBase, &'a SteadyUnit);

    fn parent(self) -> Self::Parent {
        (&self.base, &self.steady)
    }

    fn update_state(self, reader: &mut dyn PacketReader) {
        self.parent().update_state(reader);

        self.message.store(reader.opt_read_string().map(Arc::new));
    }
}

impl<'a> UnitExt<'a> for &'a Buoy {
    #[inline]
    fn is_masking(self) -> bool {
        false
    }

    #[inline]
    fn is_solid(self) -> bool {
        false
    }

    #[inline]
    fn kind(self) -> UnitKind {
        UnitKind::Buoy
    }
}
