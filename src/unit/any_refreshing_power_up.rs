
pub(crate) mod prelude {
    pub use unit::RefreshingPowerUp;

    pub(crate) use unit::RefreshingPowerUpData;
    pub use unit::any_power_up::prelude::*;
}

use std::ops::Deref;

use unit::HullRefreshingPowerUp;
use unit::IonsRefreshingPowerUp;
use unit::EnergyRefreshingPowerUp;
use unit::ShieldRefreshingPowerUp;
use unit::ParticlesRefreshingPowerUp;
use unit::ShotProductionRefreshingPowerUp;

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
            &AnyRefreshingPowerUp::EnergyRefreshingPowerUp        (ref unit) => unit.deref(),
            &AnyRefreshingPowerUp::HullRefreshingPowerUp          (ref unit) => unit.deref(),
            &AnyRefreshingPowerUp::IonsRefreshingPowerUp          (ref unit) => unit.deref(),
            &AnyRefreshingPowerUp::ParticlesRefreshingPowerUp     (ref unit) => unit.deref(),
            &AnyRefreshingPowerUp::ShieldRefreshingPowerUp        (ref unit) => unit.deref(),
            &AnyRefreshingPowerUp::ShotProductionRefreshingPowerUp(ref unit) => unit.deref(),
        }
    }
}