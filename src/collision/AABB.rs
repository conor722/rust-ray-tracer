use crate::scene::{engine::Vector3d, entities::Triangle};

#[derive(Debug, PartialEq, Clone)]
pub struct AABB {
    pub min_coords: Vector3d,
    pub max_coords: Vector3d,
}

impl AABB {
    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64, min_z: f64, max_z: f64) -> AABB {
        AABB {
            min_coords: Vector3d {
                x: min_x,
                y: min_y,
                z: min_z,
            },
            max_coords: Vector3d {
                x: max_x,
                y: max_y,
                z: max_z,
            },
        }
    }

    pub fn from_triangle<'a>(triangle: &Triangle) -> AABB {
        let min_x = f64::min(triangle.v1.x, f64::min(triangle.v2.x, triangle.v3.x));
        let max_x = f64::max(triangle.v1.x, f64::max(triangle.v2.x, triangle.v3.x));

        let min_y = f64::min(triangle.v1.y, f64::min(triangle.v2.y, triangle.v3.y));
        let max_y = f64::max(triangle.v1.y, f64::max(triangle.v2.y, triangle.v3.y));

        let min_z = f64::min(triangle.v1.z, f64::min(triangle.v2.z, triangle.v3.z));
        let max_z = f64::max(triangle.v1.z, f64::max(triangle.v2.z, triangle.v3.z));

        AABB {
            min_coords: Vector3d {
                x: min_x,
                y: min_y,
                z: min_z,
            },
            max_coords: Vector3d {
                x: max_x,
                y: max_y,
                z: max_z,
            },
        }
    }

    pub fn intersects(self, other: &Self) -> bool {
        if self.max_coords.x < other.min_coords.x || self.min_coords.x > other.max_coords.x {
            return false;
        }
        if self.max_coords.y < other.min_coords.y || self.min_coords.y > other.max_coords.y {
            return false;
        }
        if self.max_coords.z < other.min_coords.z || self.min_coords.z > other.max_coords.z {
            return false;
        }
        return true;
    }
}
