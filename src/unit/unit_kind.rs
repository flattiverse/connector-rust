
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum UnitKind {
    Unknown = 0xFFu8,
    Sun = 1,
    Planet = 2,
    Moon = 3,
    Meteoroid = 4,
    Buoy = 5,
    Nebula = 24,
    BlackHole = 32,
    WormHole = 33,
    MissionTarget = 48,
    PlayerShip = 64,
    PlayerPlatform = 65,
    PlayerProbe = 66,
    PlayerDrone = 67,
    PlayerBase = 68,
    Switch = 96,
    Gate = 97,
    /// Plasma storm
    Storm = 98,
    StormWhirl = 99,
    /// A commencing whirl, which will
    /// become a storm-whirl
    StormCommencingWhirl = 100,
    Pixel = 104,
    /// A cluster of 16x16 pixels
    PixelCluster = 105,
    EnergyPowerUp = 112,
    ParticlesPowerUp = 113,
    IonsPowerUp = 114,
    HullPowerUp = 115,
    ShieldPowerUp = 116,
    ShotProductionPowerUp = 117,
    TotalRefreshPowerUp = 120,
    HastePowerUp = 121,
    DoubleDamagePowerUp = 122,
    QuadDamagePowerUp = 123,
    CloakPowerUp = 124,
    /// This is an internal unit for administrative purpose,
    /// which will never by received by a regular player.
    Spawner = 127,
    Explosion = 128,
    Shot = 129,
    SpaceJellyFish = 160,
    SpaceJellyFishSlime = 161,
    Asteroid = 162,
    AiShip = 168,
    AiPlatform = 169,
    AiProbe = 170,
    AiDrone = 171,
    AiBase = 172,
}

impl UnitKind {
    pub fn from_id(id: u8) -> UnitKind {
        match id {
            1 => UnitKind::Sun,
            2 => UnitKind::Planet,
            3 => UnitKind::Moon,
            4 => UnitKind::Meteoroid,
            5 => UnitKind::Buoy,

            24 => UnitKind::Nebula,
            32 => UnitKind::BlackHole,
            33 => UnitKind::WormHole,
            48 => UnitKind::MissionTarget,
            64 => UnitKind::PlayerShip,
            65 => UnitKind::PlayerPlatform,
            66 => UnitKind::PlayerProbe,
            67 => UnitKind::PlayerDrone,
            68 => UnitKind::PlayerBase,
            96 => UnitKind::Switch,
            97 => UnitKind::Gate,
            98 => UnitKind::Storm,
            99 => UnitKind::StormWhirl,

            100 => UnitKind::StormCommencingWhirl,
            104 => UnitKind::Pixel,
            112 => UnitKind::EnergyPowerUp,
            113 => UnitKind::ParticlesPowerUp,
            114 => UnitKind::IonsPowerUp,
            115 => UnitKind::HullPowerUp,
            116 => UnitKind::ShieldPowerUp,
            117 => UnitKind::ShotProductionPowerUp,
            120 => UnitKind::TotalRefreshPowerUp,
            121 => UnitKind::HastePowerUp,
            122 => UnitKind::DoubleDamagePowerUp,
            123 => UnitKind::QuadDamagePowerUp,
            124 => UnitKind::CloakPowerUp,
            127 => UnitKind::Spawner,
            128 => UnitKind::Explosion,
            129 => UnitKind::Shot,
            160 => UnitKind::SpaceJellyFish,
            161 => UnitKind::SpaceJellyFishSlime,
            162 => UnitKind::Asteroid,
            168 => UnitKind::AiShip,
            169 => UnitKind::AiPlatform,
            170 => UnitKind::AiProbe,
            171 => UnitKind::AiDrone,
            172 => UnitKind::AiBase,

            _ => UnitKind::Unknown
        }
    }
}