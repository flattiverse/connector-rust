
pub(crate) mod prelude {
    pub use Player;
    pub use unit::PlayerUnit;
    pub use unit::ControllableInfo;
    pub use unit::PlayerUnitTractorbeamInfo;

    pub(crate) use unit::PlayerUnitData;
    pub use unit::any_unit::prelude::*;
}

use std::ops::Deref;

use unit::PlayerBase;
use unit::PlayerShip;
use unit::PlayerDrone;
use unit::PlayerProbe;
use unit::PlayerPlatform;

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
            &AnyPlayerUnit::PlayerBase    (ref unit) => unit.deref(),
            &AnyPlayerUnit::PlayerDrone   (ref unit) => unit.deref(),
            &AnyPlayerUnit::PlayerPlatform(ref unit) => unit.deref(),
            &AnyPlayerUnit::PlayerProbe   (ref unit) => unit.deref(),
            &AnyPlayerUnit::PlayerShip    (ref unit) => unit.deref(),
        }
    }
}