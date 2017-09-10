
mod mobility;
mod unit_kind;
mod scan_info;
mod gravity_well;
mod orbiting_state;
mod controllable_info;


mod sun;
mod buoy;
mod unit;
mod moon;
mod shot;
mod gate;
mod storm;
mod pixel;
mod planet;
mod switch;
mod nebula;
mod corona;
mod ai_unit;
mod ai_base;
mod ai_ship;
mod ai_probe;
mod ai_drone;
mod asteroid;
mod power_up;
mod explosion;
mod meteoroid;
mod worm_hole;
mod black_hole;
mod storm_whirl;
mod ai_platform;
mod player_unit;
mod player_ship;
mod player_probe;
mod player_drone;
mod pixel_cluster;
mod mission_target;
mod haste_power_up;
mod cloak_power_up;
mod player_platform;
mod space_jelly_fish;
mod refreshing_power_up;
mod quad_damage_power_up;
mod storm_commencing_whirl;
mod space_jelly_fish_slime;
mod double_damage_power_up;
mod hull_refreshing_power_up;
mod ions_refreshing_power_up;
mod total_refreshing_power_up;
mod shield_refreshing_power_up;
mod energy_refreshing_power_up;
mod player_unit_tractorbeam_info;
mod particles_refreshing_power_up;
mod shot_production_refreshing_power_up;



pub use self::mobility::*;
pub use self::unit_kind::*;
pub use self::scan_info::*;
pub use self::gravity_well::*;
pub use self::orbiting_state::*;
pub use self::controllable_info::*;

pub use self::sun::*;
pub use self::unit::*;
pub use self::buoy::*;
pub use self::moon::*;
pub use self::shot::*;
pub use self::gate::*;
pub use self::storm::*;
pub use self::pixel::*;
pub use self::planet::*;
pub use self::switch::*;
pub use self::nebula::*;
pub use self::corona::*;
pub use self::ai_unit::*;
pub use self::ai_base::*;
pub use self::ai_ship::*;
pub use self::ai_probe::*;
pub use self::ai_drone::*;
pub use self::asteroid::*;
pub use self::power_up::*;
pub use self::explosion::*;
pub use self::meteoroid::*;
pub use self::worm_hole::*;
pub use self::black_hole::*;
pub use self::storm_whirl::*;
pub use self::ai_platform::*;
pub use self::player_unit::*;
pub use self::player_ship::*;
pub use self::player_probe::*;
pub use self::player_drone::*;
pub use self::pixel_cluster::*;
pub use self::mission_target::*;
pub use self::haste_power_up::*;
pub use self::cloak_power_up::*;
pub use self::player_platform::*;
pub use self::space_jelly_fish::*;
pub use self::refreshing_power_up::*;
pub use self::quad_damage_power_up::*;
pub use self::storm_commencing_whirl::*;
pub use self::space_jelly_fish_slime::*;
pub use self::double_damage_power_up::*;
pub use self::hull_refreshing_power_up::*;
pub use self::ions_refreshing_power_up::*;
pub use self::total_refreshing_power_up::*;
pub use self::shield_refreshing_power_up::*;
pub use self::energy_refreshing_power_up::*;
pub use self::player_unit_tractorbeam_info::*;
pub use self::particles_refreshing_power_up::*;
pub use self::shot_production_refreshing_power_up::*;




use std::sync::Arc;
use std::sync::RwLock;

use Error;
use Connector;
use UniverseGroup;
use net::Packet;
use net::BinaryReader;

pub fn unit_from_packet(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet) -> Result<Arc<RwLock<Unit>>, Error> {
    let reader = &mut packet.read() as &mut BinaryReader;

    Ok(match packet.path_ship() as u8 {
        0x01 /*   1 */ => Arc::new(RwLock::new(SunData              ::from_reader(connector, universe_group, packet, reader)?)),
        0x02 /*   2 */ => Arc::new(RwLock::new(PlanetData           ::from_reader(connector, universe_group, packet, reader)?)),
        0x03 /*   3 */ => Arc::new(RwLock::new(MoonData             ::from_reader(connector, universe_group, packet, reader)?)),
        0x04 /*   4 */ => Arc::new(RwLock::new(MeteoroidData        ::from_reader(connector, universe_group, packet, reader)?)),
        0x05 /*   5 */ => Arc::new(RwLock::new(BuoyData             ::from_reader(connector, universe_group, packet, reader)?)),
        0x18 /*  24 */ => Arc::new(RwLock::new(NebulaData           ::from_reader(connector, universe_group, packet, reader)?)),
        0x20 /*  32 */ => Arc::new(RwLock::new(BlackHoleData        ::from_reader(connector, universe_group, packet, reader)?)),
        0x21 /*  33 */ => Arc::new(RwLock::new(WormHoleData         ::from_reader(connector, universe_group, packet, reader)?)),
        0x30 /*  48 */ => Arc::new(RwLock::new(MissionTargetData    ::from_reader(connector, universe_group, packet, reader)?)),
        0x40 /*  64 */ => Arc::new(RwLock::new(PlayerShipData       ::from_reader(connector, universe_group, packet, reader)?)),
        0x41 /*  65 */ => Arc::new(RwLock::new(PlayerPlatformData   ::from_reader(connector, universe_group, packet, reader)?)),
        0x42 /*  66 */ => Arc::new(RwLock::new(PlayerProbeData      ::from_reader(connector, universe_group, packet, reader)?)),
        0x43 /*  67 */ => Arc::new(RwLock::new(PlayerDroneData      ::from_reader(connector, universe_group, packet, reader)?)),
        0x44 /*  67 */ => Arc::new(RwLock::new(PlayerBaseData       ::from_reader(connector, universe_group, packet, reader)?)),
        0x60 /*  96 */ => Arc::new(RwLock::new(SwitchData           ::from_reader(connector, universe_group, packet, reader)?)),
        0x61 /*  97 */ => Arc::new(RwLock::new(GateData             ::from_reader(connector, universe_group, packet, reader)?)),
        0x62 /*  98 */ => Arc::new(RwLock::new(StormData            ::from_reader(connector, universe_group, packet, reader)?)),
        0x63 /*  99 */ => Arc::new(RwLock::new(StormWhirlData       ::from_reader(connector, universe_group, packet, reader)?)),
        0x64 /* 100 */ => Arc::new(RwLock::new(StormCommencingWhirl ::from_reader(connector, universe_group, packet, reader)?)),
        0x68 /* 104 */ => Arc::new(RwLock::new(PixelData            ::from_reader(connector, universe_group, packet, reader)?)),
        0x69 /* 105 */ => Arc::new(RwLock::new(PixelClusterData     ::from_reader(connector, universe_group, packet, reader)?)),
        0x70 /* 112 */ => Arc::new(RwLock::new(EnergyRefreshingPowerUpData          ::from_reader(connector, universe_group, packet, reader)?)),
        0x71 /* 113 */ => Arc::new(RwLock::new(ParticlesRefreshingPowerUpData       ::from_reader(connector, universe_group, packet, reader)?)),
        0x72 /* 114 */ => Arc::new(RwLock::new(IonsRefreshingPowerUpData            ::from_reader(connector, universe_group, packet, reader)?)),
        0x73 /* 115 */ => Arc::new(RwLock::new(HullRefreshingPowerUpData            ::from_reader(connector, universe_group, packet, reader)?)),
        0x74 /* 116 */ => Arc::new(RwLock::new(ShieldRefreshingPowerUpData          ::from_reader(connector, universe_group, packet, reader)?)),
        0x75 /* 117 */ => Arc::new(RwLock::new(ShotProductionRefreshingPowerUpData  ::from_reader(connector, universe_group, packet, reader)?)),
        0x78 /* 120 */ => Arc::new(RwLock::new(TotalRefreshPowerUpData              ::from_reader(connector, universe_group, packet, reader)?)),
        0x79 /* 121 */ => Arc::new(RwLock::new(HastePowerUpData             ::from_reader(connector, universe_group, packet, reader)?)),
        0x7A /* 122 */ => Arc::new(RwLock::new(DoubleDamagePowerUpData      ::from_reader(connector, universe_group, packet, reader)?)),
        0x7B /* 123 */ => Arc::new(RwLock::new(QuadDamagePowerUpData        ::from_reader(connector, universe_group, packet, reader)?)),
        0x7C /* 124 */ => Arc::new(RwLock::new(CloakPowerUpData             ::from_reader(connector, universe_group, packet, reader)?)),
        0x80 /* 128 */ => Arc::new(RwLock::new(ExplosionData                ::from_reader(connector, universe_group, packet, reader)?)),
        0x81 /* 129 */ => Arc::new(RwLock::new(ShotData                     ::from_reader(connector, universe_group, packet, reader)?)),
        0xA0 /* 160 */ => Arc::new(RwLock::new(SpaceJellyFishData           ::from_reader(connector, universe_group, packet, reader)?)),
        0xA1 /* 161 */ => Arc::new(RwLock::new(SpaceJellyFishSlimeData      ::from_reader(connector, universe_group, packet, reader)?)),
        0xA2 /* 162 */ => Arc::new(RwLock::new(AsteroidData                 ::from_reader(connector, universe_group, packet, reader)?)),
        0xA8 /* 168 */ => Arc::new(RwLock::new(AiShipData                   ::from_reader(connector, universe_group, packet, reader)?)),
        0xA9 /* 169 */ => Arc::new(RwLock::new(AiPlatformData               ::from_reader(connector, universe_group, packet, reader)?)),
        0xAA /* 170 */ => Arc::new(RwLock::new(AiProbeData                  ::from_reader(connector, universe_group, packet, reader)?)),
        0xAB /* 171 */ => Arc::new(RwLock::new(AiDronData                   ::from_reader(connector, universe_group, packet, reader)?)),
        0xAC /* 172 */ => Arc::new(RwLock::new(AiBaseData                   ::from_reader(connector, universe_group, packet, reader)?)),
        id@_ => return Err(Error::UnknownUnitType(id)),
    })
}