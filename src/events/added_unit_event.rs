use crate::events::Completable;
use crate::units::unit::Unit;
use crate::units::unit_kind::UnitKind;
use crate::universe_group::UniverseGroup;
use serde_derive::{Deserialize, Serialize};

/// This event informs of the addition of a unit to the [`UniverseGroup`].
///
/// [`UniverseGroup`]: crate::universe_group::UniverseGroup
#[derive(Debug, Serialize, Deserialize)]
pub struct AddedUnitEvent {
    pub universe: usize,
    pub unit: Unit,
}

impl Completable<UniverseGroup> for AddedUnitEvent {
    fn complete(&mut self, group: &UniverseGroup) {
        if let UnitKind::PlayerUnit(player_unit) = &mut self.unit.kind {
            player_unit.systems.complete(group);
        }
    }
}
