
use crate::Error;

#[derive(Copy, Clone)]
pub struct Color {
    pub red:   f32,
    pub green: f32,
    pub blue:  f32,
    pub alpha: f32,
}

impl Color {
    pub fn new_transparent() -> Color {
        Self::from_rgba(::std::f32::NAN, ::std::f32::NAN, ::std::f32::NAN, 0f32)
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Self::from_rgba(r, g, b, 1f32)
    }

    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            red:   r.max(0f32).min(1f32),
            green: g.max(0f32).min(1f32),
            blue:  b.max(0f32).min(1f32),
            alpha: a.max(0f32).min(1f32),
        }
    }

    pub fn from_hue(hue: f32) -> Result<Color, Error> {
        let hi = (hue / 60f32) as i32;
        let hs = (hue % 60f32) / 60f32;

        Ok(match hi {
            0|6 => Color::from_rgb(1f32, hs, 0f32),
            1 => Color::from_rgb(1f32-hs, 1f32, 0f32),
            2 => Color::from_rgb(0f32, 1f32, hs),
            3 => Color::from_rgb(0f32, 1f32-hs, 1f32),
            4 => Color::from_rgb(hs, 0f32, 1f32),
            5 => Color::from_rgb(1f32, 0f32, 1f32-hs),
            _ => return Err(Error::YouBrokeSomethingBro)
        })
    }

    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    pub fn red(&self) -> f32 {
        self.red
    }

    pub fn green(&self) -> f32 {
        self.green
    }

    pub fn blue(&self) -> f32 {
        self.blue
    }
}