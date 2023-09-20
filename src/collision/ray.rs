use crate::scene::{engine::Vector3d, entities::Triangle};

use super::AABB::AABB;

enum RayIntersectionResult {
    TriangleResult { t: f64, u: f64, v: f64 },
    AABBResult { t: f64 },
}

struct Ray {
    origin: Vector3d,
    direction: Vector3d,
}

impl Ray {
    pub fn intersect_AABB(&self, aabb: &AABB) -> Option<RayIntersectionResult> {
        let t1: f64 = (aabb.min_coords.x - self.origin.x) / self.direction.x;
        let t2: f64 = (aabb.max_coords.x - self.origin.x) / self.direction.x;
        let t3: f64 = (aabb.min_coords.y - self.origin.y) / self.direction.y;
        let t4: f64 = (aabb.max_coords.y - self.origin.y) / self.direction.y;
        let t5: f64 = (aabb.min_coords.z - self.origin.z) / self.direction.z;
        let t6: f64 = (aabb.max_coords.z - self.origin.z) / self.direction.z;

        let tmin = f64::max(
            f64::max(f64::min(t1, t2), f64::min(t3, t4)),
            f64::min(t5, t6),
        );
        let tmax = f64::min(
            f64::min(f64::max(t1, t2), f64::max(t3, t4)),
            f64::max(t5, t6),
        );

        // Intersection could have happened, but if so its behind the origin.
        if tmax < 0.0 {
            return None;
        }

        // No intersection
        if tmin > tmax {
            return None;
        }

        // Intersection has occured but O + D * tmin will be behind origin, so use tmax for closest intersection point
        if tmin < 0.0 {
            return Some(RayIntersectionResult::AABBResult { t: tmax });
        }

        return Some(RayIntersectionResult::AABBResult { t: tmin });
    }

    pub fn intersect_ray_with_triangle(
        &self,
        triangle: &Triangle,
    ) -> Option<RayIntersectionResult> {
        let edge1 = triangle.v2 - triangle.v1;
        let edge2 = triangle.v3 - triangle.v1;
        let h = self.direction.cross(&edge2);

        let a = edge1.dot(&h);

        if a > -f64::EPSILON && a < f64::EPSILON {
            // This ray is parallel to this triangle.
            return None;
        }

        let f = 1.0 / a;
        let s = self.origin - triangle.v1;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * self.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(&q);

        if t > f64::EPSILON {
            return Some(RayIntersectionResult::TriangleResult { t, u, v });
        }

        return None;
    }
}
