use crate::galaxy_hierarchy::{Cluster, ControllableBase, ControllableId, Identifiable, NamedUnit};
use crate::network::{InvalidArgumentKind, PacketReader};
use crate::unit::UnitKind;
use crate::{GameError, GameErrorKind, Vector};
use std::ops::Deref;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub enum Controllable {
    Classic(ClassicControls),
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
            UnitKind::ClassicShipPlayerUnit => Ok(Self::Classic(ClassicControls { base })),
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
    pub async fn r#continue(&self) -> Result<(), GameError> {
        self.cluster()
            .galaxy()
            .connection()
            .continue_controllable(self.id())
            .await
    }

    /// Call this to suicide (=self destroy).
    pub async fn suicide(&self) -> Result<(), GameError> {
        self.cluster()
            .galaxy()
            .connection()
            .suicide_controllable(self.id())
            .await
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

    pub(crate) fn deceased(&self) {
        match self {
            Controllable::Classic(classic) => classic.deceased(),
        }
    }

    pub(crate) fn update(&self, reader: &mut dyn PacketReader) {
        match self {
            Controllable::Classic(classic) => classic.update(reader),
        }
    }

    pub fn base(&self) -> &ControllableBase {
        match self {
            Controllable::Classic(classic) => &classic.base,
        }
    }

    pub fn classic_controls(&self) -> Option<&ClassicControls> {
        match self {
            Controllable::Classic(classic) => Some(classic),
        }
    }
}

#[derive(Debug)]
pub struct ClassicControls {
    pub(crate) base: ControllableBase,
}

impl ClassicControls {
    pub(crate) fn deceased(&self) {
        self.base.deceased();
    }

    pub(crate) fn update(&self, reader: &mut dyn PacketReader) {
        self.base.update(reader);
    }

    /// Call this to move your ship. This vector will be the impulse your ship gets every tick until
    /// you specify a new vector. Length of 0 will turn off your engines.
    pub async fn r#move(&self, movement: Vector) -> Result<(), GameError> {
        if !self.base.active() {
            Err(GameErrorKind::SpecifiedElementNotFound.into())
        } else if !self.base.alive() {
            Err(GameErrorKind::YouNeedToContinueFirst.into())
        } else if movement.x.is_nan() || movement.y.is_nan() {
            Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::ContainedNaN,
                parameter: "movement".to_string(),
            }
            .into())
        } else {
            self.base
                .cluster()
                .galaxy()
                .connection()
                .classic_controllable_move(self.base.id(), movement)
                .await
        }
    }
}

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
