use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Mul;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonCommand {
    pub id: String,
    pub command: String,
    #[serde(flatten)]
    pub parameters: HashMap<String, serde_json::Value>,
}

impl JsonCommand {
    pub fn new(id: String) -> Self {
        Self {
            id,
            command: String::default(),
            parameters: HashMap::default(),
        }
    }

    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.command = command.into();
        self
    }

    pub fn with_parameter(mut self, name: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.parameters.insert(name.into(), value.into());
        self
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    #[serde(skip_serializing, default)]
    last_angle: f64,
}

impl Vector {
    pub fn from_xy(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            last_angle: 0.0,
        }
    }

    pub fn from_angle_length(angle: f64, length: f64) -> Self {
        Self::from_xy(
            angle.to_radians().cos() * length,
            angle.to_radians().sin() * length,
        )
    }

    pub fn angle(&self) -> f64 {
        if self.x == 0.0 && self.y == 0.0 {
            self.last_angle
        } else {
            (self.y.atan2(self.x).to_degrees() + 360.0) % 360.0
        }
    }

    pub fn set_angle(&mut self, value: f64) {
        let alpha = value * std::f64::consts::PI / 180.0;
        let length = self.length();

        self.x = length * alpha.cos();
        self.y = length * alpha.sin();
    }

    #[inline]
    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn set_length(&mut self, length: f64) {
        if length == 0.0 {
            self.last_angle = self.angle();
        }

        if self.x == 0.0 && self.y == 0.0 {
            let alpha = self.last_angle.to_radians();
            self.x = length * alpha.cos();
            self.y = length * alpha.sin();
        } else {
            let length_factor = length / self.length();
            self.x *= length_factor;
            self.y *= length_factor;
        }
    }

    pub fn rotated_by(&self, degree: f64) -> Self {
        let alpha = degree.to_radians();
        Self::from_xy(
            alpha.cos().mul(self.x) - alpha.sin().mul(self.y),
            alpha.sin().mul(self.x) + alpha.cos().mul(self.y),
        )
    }

    pub fn angle_from(&self, other: &Vector) -> f64 {
        let mut degree = other.last_angle - self.last_angle;
        if degree < 0.0 {
            degree += 360.0;
        }
        degree
    }

    pub fn is_damaged(&self) -> bool {
        self.x.is_infinite() || self.x.is_nan() || self.y.is_infinite() || self.y.is_nan()
    }
}
