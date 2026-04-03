use crate::galaxy_hierarchy::{Cluster, Team};
use crate::network::PacketReader;
use crate::unit::{
    BlackHole, Buoy, CarbonCargoPowerUp, ClassicShipPlayerUnit, DominationPoint,
    EnergyChargePowerUp, Explosion, Flag, HullRepairPowerUp, HydrogenCargoPowerUp,
    IonChargePowerUp, MetalCargoPowerUp, Meteoroid, MissionTarget, MobileUnit, Mobility, Moon,
    NeutrinoChargePowerUp, Planet, PlayerUnit, PowerUp, Projectile, ShieldChargePowerUp, Shot,
    ShotChargePowerUp, SiliconCargoPowerUp, SteadyUnit, Storm, StormActiveWhirl,
    StormCommencingWhirl, StormWhirl, Sun, Switch, TargetUnit, UnitKind, WormHole,
};
use crate::utils::Atomic;
use crate::Vector;
use std::any::Any;
use std::fmt::Debug;
use std::sync::{Arc, Weak};

pub(crate) trait UnitInternal {
    fn parent(&self) -> &dyn Unit;

    #[inline]
    fn update_movement(&self, reader: &mut dyn PacketReader) {
        self.parent().update_movement(reader);
    }

    #[inline]
    fn update_state(&self, reader: &mut dyn PacketReader) {
        self.parent().update_state(reader);
    }

    #[inline]
    fn mark_full_state_known(&self) {
        self.parent().mark_full_state_known();
    }
}

#[allow(private_bounds)]
pub trait UnitHierarchy: UnitInternal {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        self.parent().as_steady_unit()
    }

    #[inline]
    fn as_target_unit(&self) -> Option<&dyn TargetUnit> {
        self.parent().as_target_unit()
    }

    #[inline]
    fn as_player_unit(&self) -> Option<&dyn PlayerUnit> {
        self.parent().as_player_unit()
    }

    #[inline]
    fn as_power_up(&self) -> Option<&dyn PowerUp> {
        self.parent().as_power_up()
    }

    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        self.parent().as_mobile_unit()
    }

    #[inline]
    fn as_storm_whirl(&self) -> Option<&dyn StormWhirl> {
        self.parent().as_storm_whirl()
    }

    #[inline]
    fn as_projectile(&self) -> Option<&dyn Projectile> {
        self.parent().as_projectile()
    }

    #[inline]
    fn as_sun(&self) -> Option<&Sun> {
        self.parent().as_sun()
    }

    #[inline]
    fn as_black_hole(&self) -> Option<&BlackHole> {
        self.parent().as_black_hole()
    }

    #[inline]
    fn as_storm(&self) -> Option<&Storm> {
        self.parent().as_storm()
    }

    #[inline]
    fn as_storm_commencing_whirl(&self) -> Option<&StormCommencingWhirl> {
        self.parent().as_storm_commencing_whirl()
    }

    #[inline]
    fn as_storm_active_whirl(&self) -> Option<&StormActiveWhirl> {
        self.parent().as_storm_active_whirl()
    }

    #[inline]
    fn as_planet(&self) -> Option<&Planet> {
        self.parent().as_planet()
    }

    #[inline]
    fn as_moon(&self) -> Option<&Moon> {
        self.parent().as_moon()
    }

    #[inline]
    fn as_meteoroid(&self) -> Option<&Meteoroid> {
        self.parent().as_meteoroid()
    }

    #[inline]
    fn as_buoy(&self) -> Option<&Buoy> {
        self.parent().as_buoy()
    }

    #[inline]
    fn as_worm_hole(&self) -> Option<&WormHole> {
        self.parent().as_worm_hole()
    }

    #[inline]
    fn as_mission_target(&self) -> Option<&MissionTarget> {
        self.parent().as_mission_target()
    }

    #[inline]
    fn as_flag(&self) -> Option<&Flag> {
        self.parent().as_flag()
    }

    #[inline]
    fn as_domination_point(&self) -> Option<&DominationPoint> {
        self.parent().as_domination_point()
    }

    #[inline]
    fn as_energy_charge_power_up(&self) -> Option<&EnergyChargePowerUp> {
        self.parent().as_energy_charge_power_up()
    }

    #[inline]
    fn as_ion_charge_power_up(&self) -> Option<&IonChargePowerUp> {
        self.parent().as_ion_charge_power_up()
    }

    #[inline]
    fn as_neutrino_charge_power_up(&self) -> Option<&NeutrinoChargePowerUp> {
        self.parent().as_neutrino_charge_power_up()
    }

    #[inline]
    fn as_metal_cargo_power_up(&self) -> Option<&MetalCargoPowerUp> {
        self.parent().as_metal_cargo_power_up()
    }

    #[inline]
    fn as_carbon_cargo_power_up(&self) -> Option<&CarbonCargoPowerUp> {
        self.parent().as_carbon_cargo_power_up()
    }

    #[inline]
    fn as_hydrogen_cargo_power_up(&self) -> Option<&HydrogenCargoPowerUp> {
        self.parent().as_hydrogen_cargo_power_up()
    }

    #[inline]
    fn as_silicon_cargo_power_up(&self) -> Option<&SiliconCargoPowerUp> {
        self.parent().as_silicon_cargo_power_up()
    }

    #[inline]
    fn as_shield_charge_power_up(&self) -> Option<&ShieldChargePowerUp> {
        self.parent().as_shield_charge_power_up()
    }

    #[inline]
    fn as_hull_repair_power_up(&self) -> Option<&HullRepairPowerUp> {
        self.parent().as_hull_repair_power_up()
    }

    #[inline]
    fn as_shot_charge_power_up(&self) -> Option<&ShotChargePowerUp> {
        self.parent().as_shot_charge_power_up()
    }

    #[inline]
    fn as_switch(&self) -> Option<&Switch> {
        self.parent().as_switch()
    }

    #[inline]
    fn as_shot(&self) -> Option<&Shot> {
        self.parent().as_shot()
    }

    #[inline]
    fn as_classic_ship(&self) -> Option<&ClassicShipPlayerUnit> {
        self.parent().as_classic_ship()
    }

    #[inline]
    fn as_explosion(&self) -> Option<&Explosion> {
        self.parent().as_explosion()
    }
}

/// Visible cluster-side mirror of one world unit.
/// Derived classes add the type-specific data that becomes available once the server has delivered the full state for
/// the unit.
#[allow(private_bounds)]
pub trait Unit: UnitInternal + UnitHierarchy + Debug + Send + Sync + Any {
    /// Stable protocol name of the unit inside its cluster.
    fn name(&self) -> &str {
        self.parent().name()
    }

    /// The radius of the unit.
    #[inline]
    fn radius(&self) -> f32 {
        self.parent().radius()
    }

    /// The position of the unit.
    #[inline]
    fn position(&self) -> Vector {
        self.parent().position()
    }

    /// The movement of the unit
    #[inline]
    fn movement(&self) -> Vector {
        self.parent().movement()
    }

    /// The direction the unit is looking into.
    #[inline]
    fn angle(&self) -> f32 {
        self.parent().angle()
    }

    /// If true, other units can hide behind this unit.
    #[inline]
    fn is_masking(&self) -> bool {
        self.parent().is_masking()
    }

    /// If true, a crash with this unit is lethal.
    #[inline]
    fn is_solid(&self) -> bool {
        self.parent().is_solid()
    }

    /// If true, the unit can be edited via map editor calls.
    #[inline]
    fn can_be_edited(&self) -> bool {
        self.parent().can_be_edited()
    }

    /// The gravity of this unit. This is how much this unit pulls others towards it.
    #[inline]
    fn gravity(&self) -> f32 {
        self.parent().gravity()
    }

    /// The mobility of the unit.
    #[inline]
    fn mobility(&self) -> Mobility {
        self.parent().mobility()
    }

    /// The kind of the unit for a better match experience.
    #[inline]
    fn kind(&self) -> UnitKind {
        self.parent().kind()
    }

    /// The cluster this unit is in.
    #[inline]
    fn cluster(&self) -> Arc<Cluster> {
        self.parent().cluster()
    }

    /// The team of the unit.
    #[inline]
    fn team(&self) -> Weak<Team> {
        self.parent().team()
    }

    /// Whether the connector has received the full state payload for this unit.
    #[inline]
    fn full_state_known(&self) -> bool {
        self.parent().full_state_known()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AbstractUnit {
    name: String,
    cluster: Weak<Cluster>,
    full_state_known: Atomic<bool>,
}

impl AbstractUnit {
    pub(crate) fn new(cluster: Weak<Cluster>, name: String) -> Self {
        Self {
            cluster,
            name,
            full_state_known: Atomic::from(false),
        }
    }
}

#[forbid(clippy::missing_trait_methods)]
impl UnitInternal for AbstractUnit {
    #[inline]
    fn parent(&self) -> &dyn Unit {
        unreachable!()
    }

    #[inline]
    fn update_movement(&self, reader: &mut dyn PacketReader) {
        let _ = reader;
    }

    #[inline]
    fn update_state(&self, reader: &mut dyn PacketReader) {
        let _ = reader;
        self.full_state_known.store(true);
    }

    #[inline]
    fn mark_full_state_known(&self) {
        self.full_state_known.store(true);
    }
}

#[forbid(clippy::missing_trait_methods)]
impl UnitHierarchy for AbstractUnit {
    #[inline]
    fn as_steady_unit(&self) -> Option<&dyn SteadyUnit> {
        None
    }

    #[inline]
    fn as_target_unit(&self) -> Option<&dyn TargetUnit> {
        None
    }

    #[inline]
    fn as_player_unit(&self) -> Option<&dyn PlayerUnit> {
        None
    }

    #[inline]
    fn as_power_up(&self) -> Option<&dyn PowerUp> {
        None
    }

    #[inline]
    fn as_mobile_unit(&self) -> Option<&dyn MobileUnit> {
        None
    }

    #[inline]
    fn as_storm_whirl(&self) -> Option<&dyn StormWhirl> {
        None
    }

    #[inline]
    fn as_projectile(&self) -> Option<&dyn Projectile> {
        None
    }

    #[inline]
    fn as_sun(&self) -> Option<&Sun> {
        None
    }

    #[inline]
    fn as_black_hole(&self) -> Option<&BlackHole> {
        None
    }

    #[inline]
    fn as_storm(&self) -> Option<&Storm> {
        None
    }

    #[inline]
    fn as_storm_commencing_whirl(&self) -> Option<&StormCommencingWhirl> {
        None
    }

    #[inline]
    fn as_storm_active_whirl(&self) -> Option<&StormActiveWhirl> {
        None
    }

    #[inline]
    fn as_planet(&self) -> Option<&Planet> {
        None
    }

    #[inline]
    fn as_moon(&self) -> Option<&Moon> {
        None
    }

    #[inline]
    fn as_meteoroid(&self) -> Option<&Meteoroid> {
        None
    }

    #[inline]
    fn as_buoy(&self) -> Option<&Buoy> {
        None
    }

    #[inline]
    fn as_worm_hole(&self) -> Option<&WormHole> {
        None
    }

    #[inline]
    fn as_mission_target(&self) -> Option<&MissionTarget> {
        None
    }

    #[inline]
    fn as_flag(&self) -> Option<&Flag> {
        None
    }

    #[inline]
    fn as_domination_point(&self) -> Option<&DominationPoint> {
        None
    }

    #[inline]
    fn as_energy_charge_power_up(&self) -> Option<&EnergyChargePowerUp> {
        None
    }

    #[inline]
    fn as_ion_charge_power_up(&self) -> Option<&IonChargePowerUp> {
        None
    }

    #[inline]
    fn as_neutrino_charge_power_up(&self) -> Option<&NeutrinoChargePowerUp> {
        None
    }

    #[inline]
    fn as_metal_cargo_power_up(&self) -> Option<&MetalCargoPowerUp> {
        None
    }

    #[inline]
    fn as_carbon_cargo_power_up(&self) -> Option<&CarbonCargoPowerUp> {
        None
    }

    #[inline]
    fn as_hydrogen_cargo_power_up(&self) -> Option<&HydrogenCargoPowerUp> {
        None
    }

    #[inline]
    fn as_silicon_cargo_power_up(&self) -> Option<&SiliconCargoPowerUp> {
        None
    }

    #[inline]
    fn as_shield_charge_power_up(&self) -> Option<&ShieldChargePowerUp> {
        None
    }

    #[inline]
    fn as_hull_repair_power_up(&self) -> Option<&HullRepairPowerUp> {
        None
    }

    #[inline]
    fn as_shot_charge_power_up(&self) -> Option<&ShotChargePowerUp> {
        None
    }

    #[inline]
    fn as_switch(&self) -> Option<&Switch> {
        None
    }

    #[inline]
    fn as_shot(&self) -> Option<&Shot> {
        None
    }

    #[inline]
    fn as_classic_ship(&self) -> Option<&ClassicShipPlayerUnit> {
        None
    }

    #[inline]
    fn as_explosion(&self) -> Option<&Explosion> {
        None
    }
}

#[forbid(clippy::missing_trait_methods)]
impl Unit for AbstractUnit {
    #[inline]
    fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    fn radius(&self) -> f32 {
        3.0
    }

    #[inline]
    fn position(&self) -> Vector {
        Vector::default()
    }

    #[inline]
    fn movement(&self) -> Vector {
        Vector::default()
    }

    #[inline]
    fn angle(&self) -> f32 {
        0.0
    }

    #[inline]
    fn is_masking(&self) -> bool {
        true
    }

    #[inline]
    fn is_solid(&self) -> bool {
        true
    }

    #[inline]
    fn can_be_edited(&self) -> bool {
        false
    }

    #[inline]
    fn gravity(&self) -> f32 {
        0.0
    }

    #[inline]
    fn mobility(&self) -> Mobility {
        Mobility::Still
    }

    #[inline]
    fn kind(&self) -> UnitKind {
        UnitKind::Sun
    }

    #[inline]
    fn cluster(&self) -> Arc<Cluster> {
        self.cluster
            .upgrade()
            .expect("Cluster should be valid for as long as the connector instance exist")
    }

    #[inline]
    fn team(&self) -> Weak<Team> {
        Weak::default()
    }

    #[inline]
    fn full_state_known(&self) -> bool {
        self.full_state_known.load()
    }
}
