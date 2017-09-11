
mod mobility;
mod unit_kind;
mod scan_info;
mod gravity_well;
mod orbiting_state;
mod gate_switch_info;
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
mod player_base;
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
pub use self::gate_switch_info::*;
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
pub use self::player_base::*;
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

pub fn unit_from_packet(connector: &Arc<Connector>, universe_group: &UniverseGroup, packet: &Packet) -> Result<Box<Unit>, Error> {
    let reader = &mut packet.read() as &mut BinaryReader;

    Ok(match packet.path_ship() as u8 {
        0x01 /*   1 */ => Box::new(SunData              ::from_reader(connector, universe_group, packet, reader)?),
        0x02 /*   2 */ => Box::new(PlanetData           ::from_reader(connector, universe_group, packet, reader)?),
        0x03 /*   3 */ => Box::new(MoonData             ::from_reader(connector, universe_group, packet, reader)?),
        0x04 /*   4 */ => Box::new(MeteoroidData        ::from_reader(connector, universe_group, packet, reader)?),
        0x05 /*   5 */ => Box::new(BuoyData             ::from_reader(connector, universe_group, packet, reader)?),
        0x18 /*  24 */ => Box::new(NebulaData           ::from_reader(connector, universe_group, packet, reader)?),
        0x20 /*  32 */ => Box::new(BlackHoleData        ::from_reader(connector, universe_group, packet, reader)?),
        0x21 /*  33 */ => Box::new(WormHoleData         ::from_reader(connector, universe_group, packet, reader)?),
        0x30 /*  48 */ => Box::new(MissionTargetData    ::from_reader(connector, universe_group, packet, reader)?),
        0x40 /*  64 */ => Box::new(PlayerShipData       ::from_reader(connector, universe_group, packet, reader)?),
        0x41 /*  65 */ => Box::new(PlayerPlatformData   ::from_reader(connector, universe_group, packet, reader)?),
        0x42 /*  66 */ => Box::new(PlayerProbeData      ::from_reader(connector, universe_group, packet, reader)?),
        0x43 /*  67 */ => Box::new(PlayerDroneData      ::from_reader(connector, universe_group, packet, reader)?),
        0x44 /*  67 */ => Box::new(PlayerBaseData       ::from_reader(connector, universe_group, packet, reader)?),
        0x60 /*  96 */ => Box::new(SwitchData           ::from_reader(connector, universe_group, packet, reader)?),
        0x61 /*  97 */ => Box::new(GateData             ::from_reader(connector, universe_group, packet, reader)?),
        0x62 /*  98 */ => Box::new(StormData            ::from_reader(connector, universe_group, packet, reader)?),
        0x63 /*  99 */ => Box::new(StormWhirlData       ::from_reader(connector, universe_group, packet, reader)?),
        0x64 /* 100 */ => Box::new(StormCommencingWhirlData      ::from_reader(connector, universe_group, packet, reader)?),
        0x68 /* 104 */ => Box::new(PixelData            ::from_reader(connector, universe_group, packet, reader)?),
        0x69 /* 105 */ => Box::new(PixelClusterData     ::from_reader(connector, universe_group, packet, reader)?),
        0x70 /* 112 */ => Box::new(EnergyRefreshingPowerUpData          ::from_reader(connector, universe_group, packet, reader)?),
        0x71 /* 113 */ => Box::new(ParticlesRefreshingPowerUpData       ::from_reader(connector, universe_group, packet, reader)?),
        0x72 /* 114 */ => Box::new(IonsRefreshingPowerUpData            ::from_reader(connector, universe_group, packet, reader)?),
        0x73 /* 115 */ => Box::new(HullRefreshingPowerUpData            ::from_reader(connector, universe_group, packet, reader)?),
        0x74 /* 116 */ => Box::new(ShieldRefreshingPowerUpData          ::from_reader(connector, universe_group, packet, reader)?),
        0x75 /* 117 */ => Box::new(ShotProductionRefreshingPowerUpData  ::from_reader(connector, universe_group, packet, reader)?),
        0x78 /* 120 */ => Box::new(TotalRefreshPowerUpData              ::from_reader(connector, universe_group, packet, reader)?),
        0x79 /* 121 */ => Box::new(HastePowerUpData             ::from_reader(connector, universe_group, packet, reader)?),
        0x7A /* 122 */ => Box::new(DoubleDamagePowerUpData      ::from_reader(connector, universe_group, packet, reader)?),
        0x7B /* 123 */ => Box::new(QuadDamagePowerUpData        ::from_reader(connector, universe_group, packet, reader)?),
        0x7C /* 124 */ => Box::new(CloakPowerUpData             ::from_reader(connector, universe_group, packet, reader)?),
        0x80 /* 128 */ => Box::new(ExplosionData                ::from_reader(connector, universe_group, packet, reader)?),
        0x81 /* 129 */ => Box::new(ShotData                     ::from_reader(connector, universe_group, packet, reader)?),
        0xA0 /* 160 */ => Box::new(SpaceJellyFishData           ::from_reader(connector, universe_group, packet, reader)?),
        0xA1 /* 161 */ => Box::new(SpaceJellyFishSlimeData      ::from_reader(connector, universe_group, packet, reader)?),
        0xA2 /* 162 */ => Box::new(AsteroidData                 ::from_reader(connector, universe_group, packet, reader)?),
        0xA8 /* 168 */ => Box::new(AiShipData                   ::from_reader(connector, universe_group, packet, reader)?),
        0xA9 /* 169 */ => Box::new(AiPlatformData               ::from_reader(connector, universe_group, packet, reader)?),
        0xAA /* 170 */ => Box::new(AiProbeData                  ::from_reader(connector, universe_group, packet, reader)?),
        0xAB /* 171 */ => Box::new(AiDroneData                  ::from_reader(connector, universe_group, packet, reader)?),
        0xAC /* 172 */ => Box::new(AiBaseData                   ::from_reader(connector, universe_group, packet, reader)?),
        id@_ => return Err(Error::UnknownUnitType(id)),
    })
}