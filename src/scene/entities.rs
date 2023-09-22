use std::{ops::Mul, sync::Arc};

use super::{engine::Vector3d, material::Material};

pub enum Light {
    Ambient { intensity: f64 },
    Point { intensity: f64, position: Vector3d },
    Directional { intensity: f64, direction: Vector3d },
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl Color {
    pub fn mix(colors: &Vec<Color>) -> Color {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for col in colors.iter() {
            r += col.r as u64;
            g += col.g as u64;
            b += col.b as u64;
        }

        r /= colors.len() as u64;
        g /= colors.len() as u64;
        b /= colors.len() as u64;

        Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Triangle {
    pub v1: Vector3d,
    pub v2: Vector3d,
    pub v3: Vector3d,
    pub v1_tex_coords: Vector3d,
    pub v2_tex_coords: Vector3d,
    pub v3_tex_coords: Vector3d,
    pub v1_normal_coords: Vector3d,
    pub v2_normal_coords: Vector3d,
    pub v3_normal_coords: Vector3d,
    pub material: Arc<Material>,
}

#[derive(Debug, PartialEq)]
pub struct Texture {
    pub colours: Vec<Color>,
    pub width: usize,
    pub height: usize,
}
