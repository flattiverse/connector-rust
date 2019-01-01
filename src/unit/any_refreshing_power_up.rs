
pub(crate) mod prelude {
    pub use crate::unit::RefreshingPowerUp;

    pub(crate) use crate::unit::RefreshingPowerUpData;
    pub use crate::unit::any_power_up::prelude::*;
}

use std::ops::Deref;

use crate::unit::HullRefreshingPowerUp;
use crate::unit::IonsRefreshingPowerUp;
use crate::unit::EnergyRefreshingPowerUp;
use crate::unit::ShieldRefreshingPowerUp;
use crate::unit::ParticlesRefreshingPowerUp;
use crate::unit::ShotProductionRefreshingPowerUp;

use self::prelude::*;

#[derive(Clone)]
pub enum AnyRefreshingPowerUp {
    EnergyRefreshingPowerUp         (Arc<EnergyRefreshingPowerUp>),
    HullRefreshingPowerUp           (Arc<HullRefreshingPowerUp>),
    IonsRefreshingPowerUp           (Arc<IonsRefreshingPowerUp>),
    ParticlesRefreshingPowerUp      (Arc<ParticlesRefreshingPowerUp>),
    ShieldRefreshingPowerUp         (Arc<ShieldRefreshingPowerUp>),
    ShotProductionRefreshingPowerUp (Arc<ShotProductionRefreshingPowerUp>),
}

impl Deref for AnyRefreshingPowerUp {
    type Target = RefreshingPowerUp;

    fn deref(&self) -> &Self::Target {
        match self {
            AnyRefreshingPowerUp::EnergyRefreshingPowerUp        (ref unit) => unit.deref(),
            AnyRefreshingPowerUp::HullRefreshingPowerUp          (ref unit) => unit.deref(),
            AnyRefreshingPowerUp::IonsRefreshingPowerUp          (ref unit) => unit.deref(),
            AnyRefreshingPowerUp::ParticlesRefreshingPowerUp     (ref unit) => unit.deref(),
            AnyRefreshingPowerUp::ShieldRefreshingPowerUp        (ref unit) => unit.deref(),
            AnyRefreshingPowerUp::ShotProductionRefreshingPowerUp(ref unit) => unit.deref(),
        }
    }
}