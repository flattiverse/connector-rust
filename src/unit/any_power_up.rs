
pub(crate) mod prelude {
    pub use unit::PowerUp;

    pub(crate) use unit::PowerUpData;
    pub use unit::any_unit::prelude::*;
}

use std::ops::Deref;

use unit::CloakPowerUp;
use unit::HastePowerUp;
use unit::QuadDamagePowerUp;
use unit::DoubleDamagePowerUp;
use unit::AnyRefreshingPowerUp;
use unit::TotalRefreshingPowerUp;

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