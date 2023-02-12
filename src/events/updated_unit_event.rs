use crate::events::Completable;
use crate::units::unit::Unit;
use crate::units::unit_kind::UnitKind;
use crate::universe_group::UniverseGroup;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatedUnitEvent {
    pub unit: Unit,
}

impl Completable<UniverseGroup> for UpdatedUnitEvent {
    fn complete(&mut self, source: &mut UniverseGroup) {
        if let UnitKind::PlayerUnit(player_unit) = &mut self.unit.kind {
            // TODO
        }
    }
}
