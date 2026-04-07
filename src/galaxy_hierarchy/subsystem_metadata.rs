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
