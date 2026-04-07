use crate::galaxy_hierarchy::SubsystemKind;

pub struct SubsystemTierInfo {
    system_kind: SubsystemKind,
    tier: i32,
    structural_load: f32,
    // TODO resource_usage: Vec<SubsystemResourceUsageFormula>,
    // TODO upgrade_cost: Costs,
    // TODO downgrade_cost: Costs,
    // TODO properties: Vec<SubsystemPropertyInfo>,
    description: String,
}

/// Identifies one configurable or runtime-relevant subsystem component.
#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    num_enum::FromPrimitive,
    num_enum::IntoPrimitive,
    strum::EnumIter,
    strum::AsRefStr,
)]
pub enum SubsystemComponentKind {
    /// A constant base term that does not depend on a configurable input.
    Base,
    /// A normalized power fraction in the range 0..1.
    NormalizedPower,
    /// The scan width component.
    Width,
    /// The scan range component.
    Range,
    /// The projectile relative-speed component.
    RelativeSpeed,
    /// The projectile lifetime-in-ticks component.
    Ticks,
    /// The projectile or explosion load component.
    ExplosionLoad,
    /// The projectile damage component.
    Damage,

    #[num_enum(catch_all)]
    Unknown(u8),
}
