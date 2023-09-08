use std::ops::Mul;

use super::engine::Vector3d;

pub enum Light {
    Ambient { intensity: f64 },
    Point { intensity: f64, position: Vector3d },
    Directional { intensity: f64, direction: Vector3d },
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        return {
            Color {
                r: (self.r as f64 * rhs) as u8,
                g: (self.g as f64 * rhs) as u8,
                b: (self.b as f64 * rhs) as u8,
            }
        };
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        return self.b as u32 + ((self.g as u32) << 8) + ((self.r as u32) << 16);
    }
}

pub struct Sphere {
    pub centre: Vector3d,
    pub radius: f64,
    pub color: Color,
    pub specular: f64,
}

pub struct Triangle {
    pub v1: Vector3d,
    pub v2: Vector3d,
    pub v3: Vector3d,
    pub color: Color,
}
