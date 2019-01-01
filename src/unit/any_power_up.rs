
pub(crate) mod prelude {
    pub use crate::unit::PowerUp;

    pub(crate) use crate::unit::PowerUpData;
    pub use crate::unit::any_unit::prelude::*;
}

use std::ops::Deref;

use crate::unit::CloakPowerUp;
use crate::unit::HastePowerUp;
use crate::unit::QuadDamagePowerUp;
use crate::unit::DoubleDamagePowerUp;
use crate::unit::AnyRefreshingPowerUp;
use crate::unit::TotalRefreshingPowerUp;

use self::prelude::*;

#[derive(Clone)]
pub enum AnyPowerUp {
    CloakPowerUp        (Arc<CloakPowerUp>),
    DoubleDamagePowerUp (Arc<DoubleDamagePowerUp>),
    HastPowerUp         (Arc<HastePowerUp>),
    QuadDamagePowerUp   (Arc<QuadDamagePowerUp>),
    RefreshingPowerUp   (AnyRefreshingPowerUp),
    TotalRefreshPowerUp (Arc<TotalRefreshingPowerUp>),
}

impl Deref for AnyPowerUp {
    type Target = PowerUp;

    fn deref(&self) -> &Self::Target {
        match self {
            &AnyPowerUp::CloakPowerUp       (ref unit) => unit.deref(),
            &AnyPowerUp::DoubleDamagePowerUp(ref unit) => unit.deref(),
            &AnyPowerUp::HastPowerUp        (ref unit) => unit.deref(),
            &AnyPowerUp::QuadDamagePowerUp  (ref unit) => unit.deref(),
            &AnyPowerUp::RefreshingPowerUp  (ref unit) => match unit {
                &AnyRefreshingPowerUp::EnergyRefreshingPowerUp        (ref unit) => unit.deref(),
                &AnyRefreshingPowerUp::HullRefreshingPowerUp          (ref unit) => unit.deref(),
                &AnyRefreshingPowerUp::IonsRefreshingPowerUp          (ref unit) => unit.deref(),
                &AnyRefreshingPowerUp::ParticlesRefreshingPowerUp     (ref unit) => unit.deref(),
                &AnyRefreshingPowerUp::ShieldRefreshingPowerUp        (ref unit) => unit.deref(),
                &AnyRefreshingPowerUp::ShotProductionRefreshingPowerUp(ref unit) => unit.deref(),
            },
            &AnyPowerUp::TotalRefreshPowerUp(ref unit) => unit.deref(),
        }
    }
}