use crate::Vector;

/// One orbit segment around the configured center or the preceding orbit segment.
#[derive(Debug, Clone)]
pub struct Orbit {
    distance: f32,
    start_angle: f32,
    rotation_ticks: i32,
}

impl Orbit {
    #[inline]
    pub(crate) fn new(distance: f32, start_angle: f32, rotation_ticks: i32) -> Self {
        Self {
            distance,
            start_angle,
            rotation_ticks,
        }
    }

    /// Distance from the current orbit center to the orbiting body.
    #[inline]
    pub fn distance(&self) -> f32 {
        self.distance
    }

    /// Angle at galaxy tick 0 in degrees.
    #[inline]
    pub fn start_angle(&self) -> f32 {
        self.start_angle
    }

    /// Full rotation period in ticks. Negative values rotate in the opposite direction.
    #[inline]
    pub fn rotation_ticks(&self) -> i32 {
        self.rotation_ticks
    }

    /// Calculates the offset contributed by this orbit segment at the given galaxy tick.
    /// The caller applies this offset relative to the current orbit center in the chain.
    pub fn calculate_offset(&self, tick: u32) -> Vector {
        #[allow(clippy::cast_abs_to_unsigned)]
        let interval = i64::from(self.rotation_ticks).abs() as u32;
        let phase_tick = tick % interval;

        let mut angle =
            (self.start_angle + 360.0 * (phase_tick as f32) / (self.rotation_ticks as f32)) % 360.0;

        if angle < 0.0 {
            angle += 360.0;
        }

        Vector::from_angle_length(angle, self.distance)
    }
}
