use crate::galaxy_hierarchy::{
    ControllableInfoBase, ControllableInfoId, Galaxy, Identifiable, NamedUnit, Player,
};
use crate::network::InvalidArgumentKind;
use crate::unit::UnitKind;
use crate::{GameError, GameErrorKind};
use std::ops::Deref;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub enum ControllableInfo {
    Classic { base: ControllableInfoBase },
    New { base: ControllableInfoBase },
}

impl ControllableInfo {
    /// The id of this ControllableInfo.
    #[inline]
    pub fn id(&self) -> ControllableInfoId {
        self.base().id()
    }

    /// The name of this controllable.
    #[inline]
    pub fn name(&self) -> &str {
        self.base().name()
    }

    /// The galaxy instance this ControllableInfo belongs to.
    #[inline]
    pub fn galaxy(&self) -> Arc<Galaxy> {
        self.base().galaxy()
    }

    /// The player this ControllableInfo belongs to.
    #[inline]
    pub fn player(&self) -> Arc<Player> {
        self.base().player()
    }

    /// true, if the corresponding PlayerUnit is alive.
    #[inline]
    pub fn alive(&self) -> bool {
        self.base().alive()
    }

    /// true, if the corresponding PlayerUnit is still in use.
    #[inline]
    pub fn active(&self) -> bool {
        self.base().active()
    }

    /// Specifies the kind of the PlayerUnit.
    pub fn kind(&self) -> UnitKind {
        match self {
            ControllableInfo::Classic { .. } => UnitKind::ClassicShipPlayerUnit,
            ControllableInfo::New { .. } => UnitKind::NewShipPlayerUnit,
        }
    }

    pub(crate) fn deactivate(&self) {
        self.base().deactivate();
    }

    pub(crate) fn set_alive(&self) {
        self.base().set_alive();
    }

    pub(crate) fn set_dead(&self) {
        self.base().set_dead();
    }

    pub fn base(&self) -> &ControllableInfoBase {
        match self {
            ControllableInfo::Classic { base, .. } => base,
            ControllableInfo::New { base, .. } => base,
        }
    }

    pub(crate) fn from_packet(
        kind: UnitKind,
        galaxy: Weak<Galaxy>,
        player: Weak<Player>,
        id: ControllableInfoId,
        name: String,
        alive: bool,
    ) -> Result<Self, GameError> {
        let base = ControllableInfoBase::new(galaxy, player, id, name, alive);
        match kind {
            UnitKind::ClassicShipPlayerUnit => Ok(Self::Classic { base }),
            UnitKind::NewShipPlayerUnit => Ok(Self::New { base }),
            _ => Err(GameErrorKind::InvalidArgument {
                reason: InvalidArgumentKind::Unknown(Default::default()),
                parameter: "kind".to_string(),
            }
            .into()),
        }
    }
}

impl Identifiable<ControllableInfoId> for ControllableInfo {
    #[inline]
    fn id(&self) -> ControllableInfoId {
        self.base().id()
    }
}

impl NamedUnit for ControllableInfo {
    #[inline]
    fn name(&self) -> impl Deref<Target = str> + '_ {
        self.base().name()
    }
}
