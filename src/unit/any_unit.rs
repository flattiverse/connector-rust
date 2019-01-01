
pub(crate) mod prelude {
    pub use ::std::sync::Arc;
    pub use ::std::sync::Weak;
    pub use ::std::sync::RwLock;
    pub use ::std::sync::RwLockReadGuard;

    pub use crate::Team;
    pub use crate::Vector;
    pub use crate::Scores;
    pub use crate::Universe;
    pub use crate::UniverseGroup;

    pub use crate::unit::Unit;
    pub use crate::unit::UnitKind;
    pub use crate::unit::ScanInfo;
    pub use crate::unit::Mobility;
    pub use crate::unit::OrbitingState;

    pub(crate) use crate::unit::UnitData;

    pub use crate::item::AnyCargoItem;
    pub use crate::item::CrystalCargoItem;

    pub use crate::controllable::AnyControllable;

    pub use crate::controllable::EnergyCost;
    pub use crate::controllable::ScanEnergyCost;
    pub use crate::controllable::WeaponEnergyCost;
}

use std::ops::Deref;

use crate::Error;
use crate::Connector;

use crate::net::Packet;
use crate::net::BinaryReader;

use crate::unit::*;


use self::prelude::*;

#[derive(Clone)]
pub enum AnyUnit {
    AiUnit          (AnyAiUnit),
    Asteroid        (Arc<Asteroid>),
    BlackHole       (Arc<BlackHole>),
    Buoy            (Arc<Buoy>),
    Explosion       (Arc<Explosion>),
    Gate            (Arc<Gate>),
    Meteoroid       (Arc<Meteoroid>),
    MissionTarget   (Arc<MissionTarget>),
    Moon            (Arc<Moon>),
    Nebula          (Arc<Nebula>),
    Pixel           (Arc<Pixel>),
    PixelCluster    (Arc<PixelCluster>),
    Planet          (Arc<Planet>),
    PlayerUnit      (AnyPlayerUnit),
    PowerUp         (AnyPowerUp),
    Shot            (Arc<Shot>),
    SpaceJellyFish      (Arc<SpaceJellyFish>),
    SpaceJellyFishSlime (Arc<SpaceJellyFishSlime>),
    Storm               (Arc<Storm>),
    StormCommencingWhirl(Arc<StormCommencingWhirl>),
    StormWhirl          (Arc<StormWhirl>),
    Sun             (Arc<Sun>),
    Switch          (Arc<Switch>),
    WormHole        (Arc<WormHole>),
}

impl AnyUnit {
    pub fn from_reader(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet, reader: &mut BinaryReader) -> Result<AnyUnit, Error> {
        Ok(match packet.path_ship() as u8 {
            0x01 /*   1 */ => AnyUnit::Sun      (Arc::new(Sun       ::from_reader(connector, universe_group, packet, reader)?)),
            0x02 /*   2 */ => AnyUnit::Planet   (Arc::new(Planet    ::from_reader(connector, universe_group, packet, reader)?)),
            0x03 /*   3 */ => AnyUnit::Moon     (Arc::new(Moon      ::from_reader(connector, universe_group, packet, reader)?)),
            0x04 /*   4 */ => AnyUnit::Meteoroid(Arc::new(Meteoroid ::from_reader(connector, universe_group, packet, reader)?)),
            0x05 /*   5 */ => AnyUnit::Buoy     (Arc::new(Buoy      ::from_reader(connector, universe_group, packet, reader)?)),
            0x18 /*  24 */ => AnyUnit::Nebula   (Arc::new(Nebula    ::from_reader(connector, universe_group, packet, reader)?)),
            0x20 /*  32 */ => AnyUnit::BlackHole(Arc::new(BlackHole ::from_reader(connector, universe_group, packet, reader)?)),
            0x21 /*  33 */ => AnyUnit::WormHole (Arc::new(WormHole  ::from_reader(connector, universe_group, packet, reader)?)),
            0x30 /*  48 */ => AnyUnit::MissionTarget(Arc::new(MissionTarget ::from_reader(connector, universe_group, packet, reader)?)),
            0x40 /*  64 */ => AnyUnit::PlayerUnit(AnyPlayerUnit::PlayerShip     (Arc::new(PlayerShip    ::from_reader(connector, universe_group, packet, reader)?))),
            0x41 /*  65 */ => AnyUnit::PlayerUnit(AnyPlayerUnit::PlayerPlatform (Arc::new(PlayerPlatform::from_reader(connector, universe_group, packet, reader)?))),
            0x42 /*  66 */ => AnyUnit::PlayerUnit(AnyPlayerUnit::PlayerProbe    (Arc::new(PlayerProbe   ::from_reader(connector, universe_group, packet, reader)?))),
            0x43 /*  67 */ => AnyUnit::PlayerUnit(AnyPlayerUnit::PlayerDrone    (Arc::new(PlayerDrone   ::from_reader(connector, universe_group, packet, reader)?))),
            0x44 /*  67 */ => AnyUnit::PlayerUnit(AnyPlayerUnit::PlayerBase     (Arc::new(PlayerBase    ::from_reader(connector, universe_group, packet, reader)?))),
            0x60 /*  96 */ => AnyUnit::Switch   (Arc::new(Switch    ::from_reader(connector, universe_group, packet, reader)?)),
            0x61 /*  97 */ => AnyUnit::Gate     (Arc::new(Gate      ::from_reader(connector, universe_group, packet, reader)?)),
            0x62 /*  98 */ => AnyUnit::Storm                (Arc::new(Storm                 ::from_reader(connector, universe_group, packet, reader)?)),
            0x63 /*  99 */ => AnyUnit::StormWhirl           (Arc::new(StormWhirl            ::from_reader(connector, universe_group, packet, reader)?)),
            0x64 /* 100 */ => AnyUnit::StormCommencingWhirl (Arc::new(StormCommencingWhirl  ::from_reader(connector, universe_group, packet, reader)?)),
            0x68 /* 104 */ => AnyUnit::Pixel                (Arc::new(Pixel                 ::from_reader(connector, universe_group, packet, reader)?)),
            0x69 /* 105 */ => AnyUnit::PixelCluster         (Arc::new(PixelCluster          ::from_reader(connector, universe_group, packet, reader)?)),
            0x70 /* 112 */ => AnyUnit::PowerUp(AnyPowerUp::RefreshingPowerUp(AnyRefreshingPowerUp::EnergyRefreshingPowerUp        (Arc::new(EnergyRefreshingPowerUp        ::from_reader(connector, universe_group, packet, reader)?)))),
            0x71 /* 113 */ => AnyUnit::PowerUp(AnyPowerUp::RefreshingPowerUp(AnyRefreshingPowerUp::ParticlesRefreshingPowerUp     (Arc::new(ParticlesRefreshingPowerUp     ::from_reader(connector, universe_group, packet, reader)?)))),
            0x72 /* 114 */ => AnyUnit::PowerUp(AnyPowerUp::RefreshingPowerUp(AnyRefreshingPowerUp::IonsRefreshingPowerUp          (Arc::new(IonsRefreshingPowerUp          ::from_reader(connector, universe_group, packet, reader)?)))),
            0x73 /* 115 */ => AnyUnit::PowerUp(AnyPowerUp::RefreshingPowerUp(AnyRefreshingPowerUp::HullRefreshingPowerUp          (Arc::new(HullRefreshingPowerUp          ::from_reader(connector, universe_group, packet, reader)?)))),
            0x74 /* 116 */ => AnyUnit::PowerUp(AnyPowerUp::RefreshingPowerUp(AnyRefreshingPowerUp::ShieldRefreshingPowerUp        (Arc::new(ShieldRefreshingPowerUp        ::from_reader(connector, universe_group, packet, reader)?)))),
            0x75 /* 117 */ => AnyUnit::PowerUp(AnyPowerUp::RefreshingPowerUp(AnyRefreshingPowerUp::ShotProductionRefreshingPowerUp(Arc::new(ShotProductionRefreshingPowerUp::from_reader(connector, universe_group, packet, reader)?)))),
            0x78 /* 120 */ => AnyUnit::PowerUp(AnyPowerUp::TotalRefreshPowerUp  (Arc::new(TotalRefreshingPowerUp::from_reader(connector, universe_group, packet, reader)?))),
            0x79 /* 121 */ => AnyUnit::PowerUp(AnyPowerUp::HastPowerUp          (Arc::new(HastePowerUp          ::from_reader(connector, universe_group, packet, reader)?))),
            0x7A /* 122 */ => AnyUnit::PowerUp(AnyPowerUp::DoubleDamagePowerUp  (Arc::new(DoubleDamagePowerUp   ::from_reader(connector, universe_group, packet, reader)?))),
            0x7B /* 123 */ => AnyUnit::PowerUp(AnyPowerUp::QuadDamagePowerUp    (Arc::new(QuadDamagePowerUp     ::from_reader(connector, universe_group, packet, reader)?))),
            0x7C /* 124 */ => AnyUnit::PowerUp(AnyPowerUp::CloakPowerUp         (Arc::new(CloakPowerUp          ::from_reader(connector, universe_group, packet, reader)?))),
            0x80 /* 128 */ => AnyUnit::Explosion          (Arc::new(Explosion           ::from_reader(connector, universe_group, packet, reader)?)),
            0x81 /* 129 */ => AnyUnit::Shot               (Arc::new(Shot                ::from_reader(connector, universe_group, packet, reader)?)),
            0xA0 /* 160 */ => AnyUnit::SpaceJellyFish     (Arc::new(SpaceJellyFish      ::from_reader(connector, universe_group, packet, reader)?)),
            0xA1 /* 161 */ => AnyUnit::SpaceJellyFishSlime(Arc::new(SpaceJellyFishSlime ::from_reader(connector, universe_group, packet, reader)?)),
            0xA2 /* 162 */ => AnyUnit::Asteroid           (Arc::new(Asteroid            ::from_reader(connector, universe_group, packet, reader)?)),
            0xA8 /* 168 */ => AnyUnit::AiUnit(AnyAiUnit::AiShip     (Arc::new(AiShip    ::from_reader(connector, universe_group, packet, reader)?))),
            0xA9 /* 169 */ => AnyUnit::AiUnit(AnyAiUnit::AiPlatform (Arc::new(AiPlatform::from_reader(connector, universe_group, packet, reader)?))),
            0xAA /* 170 */ => AnyUnit::AiUnit(AnyAiUnit::AiProbe    (Arc::new(AiProbe   ::from_reader(connector, universe_group, packet, reader)?))),
            0xAB /* 171 */ => AnyUnit::AiUnit(AnyAiUnit::AiDrone    (Arc::new(AiDrone   ::from_reader(connector, universe_group, packet, reader)?))),
            0xAC /* 172 */ => AnyUnit::AiUnit(AnyAiUnit::AiBase     (Arc::new(AiBase    ::from_reader(connector, universe_group, packet, reader)?))),
            id => return Err(Error::UnknownUnitType(id)),
        })
    }
}


impl Deref for AnyUnit {
    type Target = Unit;

    fn deref(&self) -> &Self::Target {
        match self {
            AnyUnit::AiUnit(ref unit) => match unit {
                AnyAiUnit::AiBase       (ref unit) => unit.deref(),
                AnyAiUnit::AiDrone      (ref unit) => unit.deref(),
                AnyAiUnit::AiPlatform   (ref unit) => unit.deref(),
                AnyAiUnit::AiProbe      (ref unit) => unit.deref(),
                AnyAiUnit::AiShip       (ref unit) => unit.deref(),
            },
            AnyUnit::Asteroid      (ref unit) => unit.deref(),
            AnyUnit::BlackHole     (ref unit) => unit.deref(),
            AnyUnit::Buoy          (ref unit) => unit.deref(),
            AnyUnit::Explosion     (ref unit) => unit.deref(),
            AnyUnit::Gate          (ref unit) => unit.deref(),
            AnyUnit::Meteoroid     (ref unit) => unit.deref(),
            AnyUnit::MissionTarget (ref unit) => unit.deref(),
            AnyUnit::Moon          (ref unit) => unit.deref(),
            AnyUnit::Nebula        (ref unit) => unit.deref(),
            AnyUnit::Pixel         (ref unit) => unit.deref(),
            AnyUnit::PixelCluster  (ref unit) => unit.deref(),
            AnyUnit::Planet        (ref unit) => unit.deref(),
            AnyUnit::PlayerUnit    (ref unit) => match unit {
                AnyPlayerUnit::PlayerBase    (ref unit) => unit.deref(),
                AnyPlayerUnit::PlayerDrone   (ref unit) => unit.deref(),
                AnyPlayerUnit::PlayerPlatform(ref unit) => unit.deref(),
                AnyPlayerUnit::PlayerProbe   (ref unit) => unit.deref(),
                AnyPlayerUnit::PlayerShip    (ref unit) => unit.deref(),
            },
            AnyUnit::PowerUp(ref unit) => match unit {
                AnyPowerUp::CloakPowerUp        (ref unit) => unit.deref(),
                AnyPowerUp::DoubleDamagePowerUp (ref unit) => unit.deref(),
                AnyPowerUp::HastPowerUp         (ref unit) => unit.deref(),
                AnyPowerUp::QuadDamagePowerUp   (ref unit) => unit.deref(),
                AnyPowerUp::RefreshingPowerUp   (ref unit) => match unit {
                    AnyRefreshingPowerUp::EnergyRefreshingPowerUp        (ref unit) => unit.deref(),
                    AnyRefreshingPowerUp::HullRefreshingPowerUp          (ref unit) => unit.deref(),
                    AnyRefreshingPowerUp::IonsRefreshingPowerUp          (ref unit) => unit.deref(),
                    AnyRefreshingPowerUp::ParticlesRefreshingPowerUp     (ref unit) => unit.deref(),
                    AnyRefreshingPowerUp::ShieldRefreshingPowerUp        (ref unit) => unit.deref(),
                    AnyRefreshingPowerUp::ShotProductionRefreshingPowerUp(ref unit) => unit.deref(),
                },
                AnyPowerUp::TotalRefreshPowerUp (ref unit) => unit.deref(),
            },
            AnyUnit::Shot          (ref unit) => unit.deref(),
            AnyUnit::SpaceJellyFish        (ref unit) => unit.deref(),
            AnyUnit::SpaceJellyFishSlime   (ref unit) => unit.deref(),
            AnyUnit::Storm                 (ref unit) => unit.deref(),
            AnyUnit::StormCommencingWhirl  (ref unit) => unit.deref(),
            AnyUnit::StormWhirl            (ref unit) => unit.deref(),
            AnyUnit::Sun           (ref unit) => unit.deref(),
            AnyUnit::Switch        (ref unit) => unit.deref(),
            AnyUnit::WormHole      (ref unit) => unit.deref(),
        }
    }
}