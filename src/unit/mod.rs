mod kind;
pub use kind::*;

mod steady;
pub use steady::*;

mod mobility;
pub use mobility::*;

mod sun;
pub use sun::*;

mod black_hole;
pub use black_hole::*;

mod moon;
pub use moon::*;

mod meteoroid;
pub use meteoroid::*;

mod buoy;
pub use buoy::*;

mod worm_hole;
pub use worm_hole::*;

mod mission_target;
pub use mission_target::*;

mod flag;
pub use flag::*;

mod domination_point;
pub use domination_point::*;

mod planet;
pub use planet::*;

mod mobile_unit;
pub use mobile_unit::*;

mod storm_whirl;
pub use storm_whirl::*;

mod nebula;
pub use nebula::*;

mod storm;
pub use storm::*;

mod storm_commencing_whirl;
pub use storm_commencing_whirl::*;

mod storm_active_whirl;
pub use storm_active_whirl::*;

mod player_unit;
pub use player_unit::*;

mod classic_ship_player_unit;
pub use classic_ship_player_unit::*;

mod power_up;
pub use power_up::*;

mod energy_charge_power_up;
pub use energy_charge_power_up::*;

mod ion_charge_power_up;
pub use ion_charge_power_up::*;

mod neutrino_charge_power_up;
pub use neutrino_charge_power_up::*;

mod metal_cargo_power_up;
pub use metal_cargo_power_up::*;

mod carbon_cargo_power_up;
pub use carbon_cargo_power_up::*;

mod hydrogen_cargo_power_up;
pub use hydrogen_cargo_power_up::*;

mod silicon_cargo_power_up;
pub use silicon_cargo_power_up::*;

mod shield_charge_power_up;
pub use shield_charge_power_up::*;

mod hull_repair_power_up;
pub use hull_repair_power_up::*;

mod shot_charge_power_up;
pub use shot_charge_power_up::*;

mod explosion;
pub use explosion::*;

mod switch;
pub use switch::*;

mod switch_mode;
pub use switch_mode::*;

mod projectile;
pub use projectile::*;

mod rail;
pub use rail::*;

mod shot;
pub use shot::*;

mod target;
pub use target::*;

mod armor_subsystem_info;
pub use armor_subsystem_info::*;

mod cargo_subsystem_info;
pub use cargo_subsystem_info::*;

mod battery_subsystem_info;
pub use battery_subsystem_info::*;

mod classic_ship_engine_subsystem_info;
pub use classic_ship_engine_subsystem_info::*;

mod energy_cell_subsystem_info;
pub use energy_cell_subsystem_info::*;

mod dynamic_scanner_subsystem_info;
pub use dynamic_scanner_subsystem_info::*;

mod dynamic_shot_launcher_subsystem_info;
pub use dynamic_shot_launcher_subsystem_info::*;

mod hull_subsystem_info;
pub use hull_subsystem_info::*;

mod jump_drive_subsystem_info;
pub use jump_drive_subsystem_info::*;

mod resource_miner_subsystem_info;
pub use resource_miner_subsystem_info::*;

mod nebula_collector_subsystem_info;
pub use nebula_collector_subsystem_info::*;

mod railgun_subsystem_info;
pub use railgun_subsystem_info::*;

mod repair_subsystem_info;
pub use repair_subsystem_info::*;

mod shield_subsystem_info;
pub use shield_subsystem_info::*;

mod dynamic_shot_magazine_subsystem_info;
pub use dynamic_shot_magazine_subsystem_info::*;

mod dynamic_shot_fabricator_subsystem_info;
pub use dynamic_shot_fabricator_subsystem_info::*;

mod orbit;
pub use orbit::*;

#[allow(clippy::module_inception)]
mod unit;
pub use unit::*;

mod internal {
    use crate::galaxy_hierarchy::Cluster;
    use crate::network::{InvalidArgumentKind, PacketReader};
    use crate::unit::{
        BlackHole, Buoy, CarbonCargoPowerUp, ClassicShipPlayerUnit, DominationPoint,
        EnergyChargePowerUp, Explosion, Flag, HullRepairPowerUp, HydrogenCargoPowerUp,
        IonChargePowerUp, MetalCargoPowerUp, Meteoroid, MissionTarget, Moon, Nebula,
        NeutrinoChargePowerUp, Planet, Rail, ShieldChargePowerUp, Shot, ShotChargePowerUp,
        SiliconCargoPowerUp, Storm, StormActiveWhirl, StormCommencingWhirl, Sun, Switch, Unit,
        UnitKind, WormHole,
    };
    use crate::{GameError, GameErrorKind};
    use std::sync::{Arc, Weak};

    pub(crate) fn try_read(
        kind: UnitKind,
        cluster: Weak<Cluster>,
        name: String,
        reader: &mut dyn PacketReader,
    ) -> Result<Arc<dyn Unit>, GameError> {
        Ok(match kind {
            UnitKind::Sun => Sun::new(cluster, name, reader)?,
            UnitKind::BlackHole => BlackHole::new(cluster, name, reader)?,
            UnitKind::Nebula => Nebula::new(cluster, name, reader)?,
            UnitKind::Storm => Storm::new(cluster, name, reader)?,
            UnitKind::StormCommencingWhirl => StormCommencingWhirl::new(cluster, name, reader)?,
            UnitKind::StormActiveWhirl => StormActiveWhirl::new(cluster, name, reader)?,
            UnitKind::Planet => Planet::new(cluster, name, reader)?,
            UnitKind::Moon => Moon::new(cluster, name, reader)?,
            UnitKind::Meteoroid => Meteoroid::new(cluster, name, reader)?,
            UnitKind::Buoy => Buoy::new(cluster, name, reader)?,
            UnitKind::WormHole => WormHole::new(cluster, name, reader)?,
            UnitKind::MissionTarget => MissionTarget::new(cluster, name, reader)?,
            UnitKind::Flag => Flag::new(cluster, name, reader)?,
            UnitKind::DominationPoint => DominationPoint::new(cluster, name, reader)?,
            UnitKind::EnergyChargePowerUp => EnergyChargePowerUp::new(cluster, name, reader)?,
            UnitKind::IonChargePowerUp => IonChargePowerUp::new(cluster, name, reader)?,
            UnitKind::NeutrinoChargePowerUp => NeutrinoChargePowerUp::new(cluster, name, reader)?,
            UnitKind::MetalCargoPowerUp => MetalCargoPowerUp::new(cluster, name, reader)?,
            UnitKind::CarbonCargoPowerUp => CarbonCargoPowerUp::new(cluster, name, reader)?,
            UnitKind::HydrogenCargoPowerUp => HydrogenCargoPowerUp::new(cluster, name, reader)?,
            UnitKind::SiliconCargoPowerUp => SiliconCargoPowerUp::new(cluster, name, reader)?,
            UnitKind::ShieldChargePowerUp => ShieldChargePowerUp::new(cluster, name, reader)?,
            UnitKind::HullRepairPowerUp => HullRepairPowerUp::new(cluster, name, reader)?,
            UnitKind::ShotChargePowerUp => ShotChargePowerUp::new(cluster, name, reader)?,
            UnitKind::Switch => Switch::new(cluster, name, reader)?,
            UnitKind::Shot => Shot::new(cluster, name, reader)?,
            UnitKind::Rail => Rail::new(cluster, name, reader)?,
            UnitKind::ClassicShipPlayerUnit => ClassicShipPlayerUnit::new(cluster, name, reader)?,
            UnitKind::Explosion => Explosion::new(cluster, name, reader)?,
            // TODO this should not be necessary
            _ => {
                return Err(GameErrorKind::InvalidArgument {
                    reason: InvalidArgumentKind::Unknown(0xFF),
                    parameter: "kind".to_string(),
                }
                .into())
            }
        })
    }
}
pub(crate) use internal::*;
