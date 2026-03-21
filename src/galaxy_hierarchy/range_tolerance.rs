use crate::network::InvalidArgumentKind;
use crate::Vector;

pub struct RangeTolerance;

impl RangeTolerance {
    pub const LOWER_FACTOR: f32 = 0.999;
    pub const UPPER_FACTOR: f32 = 1.001;

    pub fn validated_f32(value: f32) -> Result<f32, InvalidArgumentKind> {
        if value.is_nan() {
            Err(InvalidArgumentKind::ContainedNaN)
        } else if value.is_infinite() {
            Err(InvalidArgumentKind::ConstrainedInfinity)
        } else {
            Ok(value)
        }
    }

    pub fn validated_vector(value: Vector) -> Result<Vector, InvalidArgumentKind> {
        if value.x.is_nan() || value.y.is_nan() {
            Err(InvalidArgumentKind::ContainedNaN)
        } else if value.x.is_infinite() || value.y.is_infinite() {
            Err(InvalidArgumentKind::ConstrainedInfinity)
        } else {
            Ok(value)
        }
    }

    pub fn clamped_maximum(value: f32, maximum: f32) -> Result<f32, InvalidArgumentKind> {
        debug_assert!(
            maximum >= 0.0 && maximum.is_finite(),
            "Invalid scalar maximum specified."
        );

        let value = Self::validated_f32(value)?;

        if value < 0.0 {
            Err(InvalidArgumentKind::TooSmall)
        } else if value > maximum * Self::UPPER_FACTOR {
            Err(InvalidArgumentKind::TooLarge)
        } else {
            Ok(value.min(maximum))
        }
    }

    pub fn clamped_range(
        value: f32,
        minimum: f32,
        maximum: f32,
    ) -> Result<f32, InvalidArgumentKind> {
        debug_assert!(
            minimum >= 0.0 && maximum >= minimum && minimum.is_finite() && maximum.is_finite()
        );

        let value = Self::validated_f32(value)?;

        if value < minimum * Self::LOWER_FACTOR {
            Err(InvalidArgumentKind::TooSmall)
        } else if value > maximum * Self::UPPER_FACTOR {
            Err(InvalidArgumentKind::TooLarge)
        } else if value < minimum {
            Ok(value)
        } else {
            Ok(value.min(maximum))
        }
    }

    pub fn clamped_maximum_vector(
        value: Vector,
        maximum: f32,
    ) -> Result<Vector, InvalidArgumentKind> {
        debug_assert!(maximum >= 0.0 && maximum.is_finite());

        let value = Self::validated_vector(value)?;
        let length = value.length();

        if length > (maximum * Self::UPPER_FACTOR) {
            Err(InvalidArgumentKind::TooLarge)
        } else if length <= maximum {
            Ok(value)
        } else {
            Ok(Self::clamped_length(value, length, maximum))
        }
    }

    pub fn clamped_range_vector(
        value: Vector,
        minimum: f32,
        maximum: f32,
    ) -> Result<Vector, InvalidArgumentKind> {
        debug_assert!(
            minimum >= 0.0 && maximum >= minimum && minimum.is_finite() && maximum.is_finite()
        );

        let value = Self::validated_vector(value)?;
        let length = value.length();

        if length < (minimum * Self::LOWER_FACTOR) {
            Err(InvalidArgumentKind::TooSmall)
        } else if length > (maximum * Self::UPPER_FACTOR) {
            Err(InvalidArgumentKind::TooLarge)
        } else if length < minimum {
            Ok(Self::clamped_length(value, length, minimum))
        } else if length > maximum {
            Ok(Self::clamped_length(value, length, maximum))
        } else {
            Ok(value)
        }
    }

    fn clamped_length(value: Vector, current_length: f32, target_length: f32) -> Vector {
        debug_assert!(target_length >= 0.0 && target_length.is_finite());

        if target_length == 0.0 || current_length == 0.0 {
            Vector::default()
        } else {
            let candidate = value.with_length(target_length);

            if candidate.length() > target_length {
                let safe_target_length = target_length.next_down();
                if safe_target_length <= 0.0 {
                    Vector::default()
                } else {
                    value.with_length(safe_target_length.next_down())
                }
            } else {
                candidate
            }
        }
    }
}
