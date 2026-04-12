pub struct ShipBalancing;

impl ShipBalancing {
    const GRAVITY_BASE: f32 = 0.0023762376;
    const GRAVITY_STRUCTURAL_LOAD_FACTOR: f32 = 0.037128713;

    pub const fn calculate_radius(effective_load: f32) -> f32 {
        1. + 47. * effective_load / 100.
    }

    pub const fn calculate_gravity(effective_load: f32) -> f32 {
        Self::GRAVITY_BASE + Self::GRAVITY_STRUCTURAL_LOAD_FACTOR * effective_load / 100.0
    }

    pub fn calculate_classic_speed_limit(effective_load: f32) -> f32 {
        6f32 - 2. * (effective_load / 100f32).powf(0.8f32)
    }

    pub fn calculate_modern_speed_limit(effective_load: f32) -> f32 {
        6.5f32 - 2. * (effective_load / 100f32).powf(0.8f32)
    }

    pub fn calculate_engine_efficiency(effective_load: f32) -> f32 {
        1.2_f32 - 0.45 * (effective_load / 100.0f32).powf(0.85)
    }

    pub fn calculate_engine_energy(value: f32, maximum: f32, full_cost: f32) -> f32 {
        debug_assert!(
            value.is_normal() && value >= 0.0,
            "Invalid engine value specified: {value}."
        );
        debug_assert!(
            maximum.is_normal() && maximum >= 0.0,
            "Invalid engine maximum specified: {value}."
        );
        debug_assert!(
            full_cost.is_normal() && full_cost >= 0.0,
            "Invalid engine full cost specified: {value}."
        );

        if maximum <= 0.0 || value <= 0.0 || full_cost == 0.0 {
            0.0
        } else {
            let power01 = value / maximum;
            full_cost * (0.30 * power01 + 0.70 * power01 * power01 * power01)
        }
    }

    pub const fn calculate_shield_energy(
        tier: u8,
        rate: f32,
        maximum_rate: f32,
        full_cost: f32,
    ) -> f32 {
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

    pub fn calculate_repair_energy(tier: u8, rate: f32, maximum_rate: f32) -> f32 {
        if rate <= 0.0 || maximum_rate <= 0.0 {
            0.0
        } else {
            let power01 = rate / maximum_rate;
            match tier {
                1 => 18.0 * power01.powf(0.35) + 7.0 * power01 * power01 * power01,
                2 => 24.0 * power01.powf(0.35) + 12.0 * power01 * power01 * power01,
                3 => 32.0 * power01.powf(0.35) + 20.0 * power01 * power01 * power01,
                4 => 44.0 * power01.powf(0.35) + 32.0 * power01 * power01 * power01,
                5 => 58.0 * power01.powf(0.35) + 50.0 * power01 * power01 * power01,
                _ => 0.0,
            }
        }
    }

    pub fn calculate_scanner_energy(width: f32, length: f32) -> f32 {
        debug_assert!(
            width.is_normal() && width >= 0.0,
            "Invalid scanner width specified."
        );
        debug_assert!(
            length.is_normal() && length >= 0.0,
            "Invalid scanner length specified."
        );

        if width <= 0.0 || length <= 0.0 {
            0.0
        } else {
            #[allow(clippy::approx_constant)]
            let length_cost =
                0.3926 * length.powf(0.5) + 2.76e-10 * length * length * length * length - 0.617;
            let width_cost = 0.141176 * width - 0.705882;
            let energy = length_cost + width_cost;

            if energy > 0.0 {
                energy
            } else {
                0.0
            }
        }
    }

    pub fn calculate_shot_launch_energy(speed: f32, ticks: u16, load: f32, damage: f32) -> f32 {
        debug_assert!(
            speed.is_normal() && speed >= 0.0,
            "Invalid shot speed specified"
        );
        debug_assert!(
            load.is_normal() && load >= 0.0,
            "Invalid shot load specified."
        );
        debug_assert!(
            damage.is_normal() && damage >= 0.0,
            "Invalid shot damage specified."
        );

        let energy = 20.0 + 60.0 * speed + 3.0 * f32::from(ticks) + 15.0 * load + 20.0 * damage;

        if energy.is_normal() {
            energy
        } else {
            0.0
        }
    }
}
