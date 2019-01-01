
use crate::Error;
use crate::net::BinaryReader;

pub struct OrbitingState {
    distance: f32,
    start_angle: f32,
    angle: f32,
    rotation_interval: i16,
    rotation: i16
}

impl OrbitingState {
    pub fn from_reader(reader: &mut BinaryReader) -> Result<OrbitingState, Error> {
        let distance = reader.read_single()?;
        let start_angle = reader.read_single()?;
        let rotation_interval = reader.read_short()?;
        let rotation = reader.read_short()?;
        let angle;

        if rotation_interval < 0i16 {
            angle = (0f32-rotation as f32) / (rotation_interval as f32 * 360f32 + start_angle);
        } else {
            angle = (rotation as f32) / (rotation_interval as f32 * 360f32 + start_angle);
        }

        Ok(OrbitingState {
            distance,
            start_angle,
            rotation_interval,
            rotation,
            angle: angle % 360f32
        })
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn start_angle(&self) -> f32 {
        self.start_angle
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn rotation_interval(&self) -> i16 {
        self.rotation_interval
    }

    pub fn rotation(&self) -> i16 {
        self.rotation
    }
}