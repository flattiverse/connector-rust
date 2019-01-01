
pub(crate) mod prelude {
    pub use crate::Player;
    pub use crate::unit::PlayerUnit;
    pub use crate::unit::ControllableInfo;
    pub use crate::unit::PlayerUnitTractorbeamInfo;

    pub(crate) use crate::unit::PlayerUnitData;
    pub use crate::unit::any_unit::prelude::*;
}

use std::ops::Deref;

use crate::unit::PlayerBase;
use crate::unit::PlayerShip;
use crate::unit::PlayerDrone;
use crate::unit::PlayerProbe;
use crate::unit::PlayerPlatform;

use self::prelude::*;

#[derive(Clone)]
pub enum AnyPlayerUnit {
    PlayerBase    (Arc<PlayerBase>),
    PlayerDrone   (Arc<PlayerDrone>),
    PlayerPlatform(Arc<PlayerPlatform>),
    PlayerProbe   (Arc<PlayerProbe>),
    PlayerShip    (Arc<PlayerShip>),
}

impl Deref for AnyPlayerUnit {
    type Target = PlayerUnit;

    fn deref(&self) -> &Self::Target {
        match self {
            AnyPlayerUnit::PlayerBase    (ref unit) => unit.deref(),
            AnyPlayerUnit::PlayerDrone   (ref unit) => unit.deref(),
            AnyPlayerUnit::PlayerPlatform(ref unit) => unit.deref(),
            AnyPlayerUnit::PlayerProbe   (ref unit) => unit.deref(),
            AnyPlayerUnit::PlayerShip    (ref unit) => unit.deref(),
        }
    }
}