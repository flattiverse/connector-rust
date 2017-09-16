
pub(crate) mod prelude {
    pub use ::std::sync::Arc;
    pub use ::std::sync::Weak;

    pub use Team;
    pub use Vector;
    pub use Connector;
    pub use UniverseGroup;

    pub use unit::Unit;
    pub use unit::UnitKind;
    pub use unit::Mobility;
    pub use unit::OrbitingState;

    pub use unit::AiUnit;
    pub(crate) use unit::AiUnitData;

}

use std::ops::Deref;

use unit::AiBase;
use unit::AiShip;
use unit::AiDrone;
use unit::AiProbe;
use unit::AiPlatform;

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