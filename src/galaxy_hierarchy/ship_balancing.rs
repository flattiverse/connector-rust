pub struct ShipBalancing;

impl ShipBalancing {
    pub fn calculate_shield_energy(tier: u8, rate: f32, maximum_rate: f32, full_cost: f32) -> f32 {
        debug_assert!(
            maximum_rate >= 0.0,
            "Invalid shield maximum rate specified."
        );

        if rate <= 0.0 || maximum_rate <= 0.0 || full_cost <= 0.0 {
            0.0
        } else {
            let power01 = rate / maximum_rate;
            let curve = match tier {
                1 => 0.70 * power01 + 0.30 * power01 * power01 * power01,
                2 => 0.55 * power01 + 0.45 * power01 * power01 * power01,
                3 => 0.40 * power01 + 0.60 * power01 * power01 * power01,
                4 => 0.46 * power01 + 0.54 * power01 * power01 * power01,
                5 => 0.52 * power01 + 0.48 * power01 * power01 * power01,
                _ => 0.0,
            };

            full_cost * curve
        }
    }
}
