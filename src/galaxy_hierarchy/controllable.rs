use crate::galaxy_hierarchy::{Cluster, ControllableBase, ControllableId, Identifiable, NamedUnit};
use crate::network::{InvalidArgumentKind, PacketReader};
use crate::unit::UnitKind;
use crate::{GameError, GameErrorKind, Vector};
use std::ops::Deref;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub enum Controllable {
    Classic {
        base: ControllableBase,
        classic: ClassicControls,
    },
}

impl Controllable {
    pub(crate) fn from_packet(
        kind: UnitKind,
        cluster: Weak<Cluster>,
        id: ControllableId,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Self, GameError> {
        let base = ControllableBase::new(id, name, cluster, reader);
        match kind {
            UnitKind::ClassicShipPlayerUnit => Ok(Self::Classic {
                base,
                classic: ClassicControls {},
            }),
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
        self.base().id()
    }

    /// The name of the controllable.
    #[inline]
    pub fn name(&self) -> &str {
        self.base().name()
    }

    /// The cluster this unit currently is in.
    #[inline]
    pub fn cluster(&self) -> Arc<Cluster> {
        self.base().cluster()
    }

    /// The position of the unit.
    #[inline]
    pub fn position(&self) -> Vector {
        self.base().position()
    }

    /// The movement of the unit.
    #[inline]
    pub fn movement(&self) -> Vector {
        self.base().movement()
    }

    /// true, if the unit is alive.
    #[inline]
    pub fn alive(&self) -> bool {
        self.base().alive()
    }

    /// Call this to continue the game with this unit after you are dead or when you hve created the
    /// unit.
    pub fn r#continue(&self) {
        todo!()
    }

    /// Call this to suicide (=self destroy).
    pub fn kill(&self) {
        todo!()
    }

    /// Call this to close the unit.
    pub async fn dispose(&self) -> Result<(), GameError> {
        self.cluster()
            .galaxy()
            .connection()
            .dispose_controllable(self.id())
            .await
    }

    /// true, if this objet still can be used. If the unit has been disposed this is false.
    #[inline]
    pub fn active(&self) -> bool {
        self.base().active()
    }

    pub fn base(&self) -> &ControllableBase {
        match self {
            Controllable::Classic { base, .. } => base,
        }
    }
}

#[derive(Debug)]
pub struct ClassicControls {}

impl Identifiable<ControllableId> for Controllable {
    #[inline]
    fn id(&self) -> ControllableId {
        self.base().id()
    }
}

impl NamedUnit for Controllable {
    #[inline]
    fn name(&self) -> impl Deref<Target = str> + '_ {
        self.base().name()
    }
}
