use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{SteadyUnit, UnitBase, UnitExt, UnitExtSealed, UnitKind};
use crate::utils::Readable;
use std::sync::Weak;

/// A buoy.
#[derive(Debug, Clone)]
pub struct Buoy {
    base: UnitBase,
    steady: SteadyUnit,
    message: Option<String>,
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
            message: {
                let string = reader.read_string();
                if string.is_empty() {
                    None
                } else {
                    Some(string)
                }
            },
        }
    }

    /// Optional buoy message. [None] means no message.
    #[inline]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
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
