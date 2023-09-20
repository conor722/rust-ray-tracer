use crate::scene::{engine::Vector3d, entities::Triangle};

use super::{aabb::Aabb, octree::Octree};

pub struct RayTriangleIntersectionResult<'a> {
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub triangle: &'a Triangle,
}
pub struct RayAABBIntersectionResult {
    t: f64,
}

pub struct Ray {
    pub origin: Vector3d,
    pub direction: Vector3d,
}

impl Ray {
    pub fn intersect_aabb(&self, aabb: &Aabb) -> Option<RayAABBIntersectionResult> {
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
            return Some(RayAABBIntersectionResult { t: tmax });
        }

        return Some(RayAABBIntersectionResult { t: tmin });
    }

    pub fn intersect_with_triangle<'a>(
        &self,
        triangle: &'a Triangle,
    ) -> Option<RayTriangleIntersectionResult<'a>> {
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
            return Some(RayTriangleIntersectionResult { t, u, v, triangle });
        }

        return None;
    }

    pub fn intersect_with_octant<'a>(
        &self,
        octree: &'a Octree,
        octant_index: usize,
    ) -> Option<RayTriangleIntersectionResult<'a>> {
        if *octree.octant_triangle_count_map.get(&octant_index).unwrap() == 0 {
            return None;
        }

        let triangles_at_octant = octree.octant_triangle_map.get(&octant_index).unwrap();
        let mut intersected_triangle_in_octant: Option<RayTriangleIntersectionResult> = None;
        let mut closest_triangle_in_octant_distance = f64::INFINITY;

        for triangle_index in triangles_at_octant {
            let this_triangle = octree.triangles.get(*triangle_index).unwrap();
            let this_triangle_intersection = self.intersect_with_triangle(this_triangle);

            if let Some(tri) = this_triangle_intersection {
                if tri.t < closest_triangle_in_octant_distance {
                    closest_triangle_in_octant_distance = tri.t;
                    intersected_triangle_in_octant = Some(tri);
                }
            }
        }

        let child_octants = octree.octant_child_map.get(&octant_index).unwrap();

        let mut child_octant_intersection_distances = vec![];

        for coi in child_octants {
            let child_octant_aabb_index = octree.octant_aabb_map.get(&coi).unwrap();
            let child_octant_aabb = octree.aabbs.get(*child_octant_aabb_index).unwrap();
            let child_octant_ray_intersection = self.intersect_aabb(child_octant_aabb);

            if let Some(coirs) = child_octant_ray_intersection {
                child_octant_intersection_distances.push((coirs.t, coi));
            }
        }

        let mut intersected_triangle_in_child_octant: Option<RayTriangleIntersectionResult> = None;
        let mut intersected_triangle_in_child_octant_distance = f64::INFINITY;

        child_octant_intersection_distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for (_, coi) in child_octant_intersection_distances {
            let res = self.intersect_with_octant(octree, *coi);

            if let Some(rti) = res {
                intersected_triangle_in_child_octant_distance = rti.t;
                intersected_triangle_in_child_octant = Some(rti);

                break;
            }
        }

        if intersected_triangle_in_child_octant_distance < closest_triangle_in_octant_distance {
            return intersected_triangle_in_child_octant;
        } else {
            return intersected_triangle_in_octant;
        }
    }
}
