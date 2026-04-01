use crate::galaxy_hierarchy::Cluster;
use crate::network::PacketReader;
use crate::unit::{
    AbstractSteadyUnit, SteadyUnit, SteadyUnitInternal, Unit, UnitHierarchy, UnitInternal,
};
use crate::utils::Atomic;
use crate::GameError;
use std::sync::Weak;

pub(crate) trait PowerUpInternal {
    fn parent(&self) -> &dyn PowerUp;
}

/// Shared base class for visible collectible power-ups.
#[allow(private_bounds)]
pub trait PowerUp: PowerUpInternal + SteadyUnit {
    /// The configured payload amount of this power-up.
    #[inline]
    fn amount(&self) -> f32 {
        PowerUpInternal::parent(self).amount()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AbstractPowerUp {
    parent: AbstractSteadyUnit,
    amount: Atomic<f32>,
}

impl AbstractPowerUp {
    pub(crate) fn new(
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Self, GameError> {
        Ok(Self {
            parent: AbstractSteadyUnit::new(cluster, name, reader)?,
            amount: Atomic::from(0.0),
        })
    }
}

impl UnitInternal for AbstractPowerUp {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        &self.parent
    }

    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent.update_state(reader);
        self.amount.read(reader);
    }
}

impl UnitHierarchy for AbstractPowerUp {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        Some(self)
    }

    #[inline]
    fn as_power_up(&self) -> Option<&dyn PowerUp> {
        Some(self)
    }
}

impl Unit for AbstractPowerUp {
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
}

impl SteadyUnitInternal for AbstractPowerUp {}

impl SteadyUnit for AbstractPowerUp {}

impl PowerUpInternal for AbstractPowerUp {
    #[inline]
    fn parent(&self) -> &dyn PowerUp {
        unreachable!()
    }
}

#[forbid(clippy::missing_trait_methods)]
impl PowerUp for AbstractPowerUp {
    #[inline]
    fn amount(&self) -> f32 {
        self.amount.load()
    }
}
