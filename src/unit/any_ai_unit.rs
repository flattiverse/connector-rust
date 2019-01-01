
pub(crate) mod prelude {
    pub use ::std::sync::Arc;
    pub use ::std::sync::Weak;

    pub use crate::Team;
    pub use crate::Vector;
    pub use crate::Connector;
    pub use crate::UniverseGroup;

    pub use crate::unit::Unit;
    pub use crate::unit::UnitKind;
    pub use crate::unit::Mobility;
    pub use crate::unit::OrbitingState;

    pub use crate::unit::AiUnit;
    pub(crate) use crate::unit::AiUnitData;

}

use std::ops::Deref;

use crate::unit::AiBase;
use crate::unit::AiShip;
use crate::unit::AiDrone;
use crate::unit::AiProbe;
use crate::unit::AiPlatform;

use self::prelude::*;

#[derive(Clone)]
pub enum AnyAiUnit {
    AiBase      (Arc<AiBase>),
    AiDrone     (Arc<AiDrone>),
    AiPlatform  (Arc<AiPlatform>),
    AiProbe     (Arc<AiProbe>),
    AiShip      (Arc<AiShip>),
}

impl Deref for AnyAiUnit {
    type Target = AiUnit;

    fn deref(&self) -> &Self::Target {
        match self {
            &AnyAiUnit::AiBase    (ref unit) => unit.deref(),
            &AnyAiUnit::AiDrone   (ref unit) => unit.deref(),
            &AnyAiUnit::AiPlatform(ref unit) => unit.deref(),
            &AnyAiUnit::AiProbe   (ref unit) => unit.deref(),
            &AnyAiUnit::AiShip    (ref unit) => unit.deref(),
        }
    }
}