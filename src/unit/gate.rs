use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractSteadyUnit, SteadyUnit, SteadyUnitInternal, Unit, UnitCastTable, UnitHierarchy,
    UnitInternal, UnitKind,
};
use crate::utils::{Also, Atomic};
use crate::GameError;
use std::sync::{Arc, Weak};

#[derive(Debug, Clone)]
pub struct Gate {
    parent: AbstractSteadyUnit,
    linked_id: Atomic<u16>,
    default_close: Atomic<bool>,
    restore_ticks: Atomic<Option<u16>>,
    closed: Atomic<bool>,
    restore_remaining_ticks: Atomic<Option<u16>>,
}

impl Gate {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<Self>, GameError> {
        Ok(Arc::new(
            Self {
                parent: AbstractSteadyUnit::new(cluster, name, reader)?,
                linked_id: Atomic::from(reader.read_uint16()),
                default_close: Atomic::from(reader.read_byte() != 0x00),
                restore_ticks: Atomic::from(if reader.read_byte() != 0x00 {
                    Some(reader.read_uint16())
                } else {
                    None
                }),
                closed: Atomic::default(),
                restore_remaining_ticks: Atomic::from(None),
            }
            .also(|this| {
                this.closed.store(this.default_close.load());
            }),
        ))
    }

    /// Link id shared with switches.
    #[inline]
    pub fn linked_it(&self) -> u16 {
        self.linked_id.load()
    }

    /// Default closed state to which the gate may restore.
    #[inline]
    pub fn default_closed(&self) -> bool {
        self.default_close.load()
    }

    /// Optional configured restore delay in ticks.
    #[inline]
    pub fn restore_ticks(&self) -> Option<u16> {
        self.restore_ticks.load()
    }

    /// Current gate state.
    #[inline]
    pub fn closed(&self) -> bool {
        self.closed.load()
    }

    /// Remaining restore delay in ticks while a restore is armed.
    #[inline]
    pub fn restore_remaining_ticks(&self) -> Option<u16> {
        self.restore_remaining_ticks.load()
    }
}

impl UnitInternal for Gate {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);

        self.closed.store(reader.read_byte() != 0x00);

        if reader.read_byte() != 0x00 {
            self.restore_remaining_ticks
                .store(Some(reader.read_uint16()));
        } else {
            self.restore_remaining_ticks.store(None);
        }
    }
}

impl UnitCastTable for Gate {
    cast_fn!(steady_unit_cast_fn, Gate, dyn SteadyUnit);
}

impl UnitHierarchy for Gate {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_gate(&self) -> Option<&Gate> {
        Some(self)
    }
}

impl Unit for Gate {
    #[inline]
    fn is_masking(&self) -> bool {
        self.closed.load()
    }

    #[inline]
    fn is_solid(&self) -> bool {
        self.closed.load()
    }

    #[inline]
    fn can_be_edited(&self) -> bool {
        true
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Gate
    }
}

impl SteadyUnitInternal for Gate {
    #[inline]
    fn parent(&self) -> &dyn SteadyUnit {
        &self.parent
    }
}

impl SteadyUnit for Gate {}
