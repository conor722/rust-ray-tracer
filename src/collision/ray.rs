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
        self.intersect_with_octant_with_max_t(octree, octant_index, f64::INFINITY)
    }

    pub fn intersect_with_octant_with_max_t<'a>(
        &self,
        octree: &'a Octree,
        octant_index: usize,
        max_t: f64,
    ) -> Option<RayTriangleIntersectionResult<'a>> {
        let node = &octree.nodes[octant_index];

        if node.triangle_count == 0 {
            return None;
        }

        let mut intersected_triangle_in_octant: Option<RayTriangleIntersectionResult> = None;
        let mut closest_triangle_in_octant_distance = max_t;

        for &triangle_index in &node.triangles {
            let this_triangle = &octree.triangles[triangle_index];
            let this_triangle_intersection = self.intersect_with_triangle(this_triangle);

            if let Some(tri) = this_triangle_intersection {
                if tri.t < closest_triangle_in_octant_distance {
                    closest_triangle_in_octant_distance = tri.t;
                    intersected_triangle_in_octant = Some(tri);
                }
            }
        }

        // Use a small fixed-size array to avoid heap allocation (octrees have at most 8 children)
        let mut child_octant_intersection_distances: [(f64, usize); 8] = [(0.0, 0); 8];
        let mut num_children = 0;

        for &coi in &node.children {
            let child_node = &octree.nodes[coi];
            let child_octant_aabb = &octree.aabbs[child_node.aabb_index];
            let child_octant_ray_intersection = self.intersect_aabb(child_octant_aabb);

            if let Some(coirs) = child_octant_ray_intersection {
                child_octant_intersection_distances[num_children] = (coirs.t, coi);
                num_children += 1;
            }
        }

        let children_slice = &mut child_octant_intersection_distances[..num_children];
        children_slice.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut intersected_triangle_in_child_octant: Option<RayTriangleIntersectionResult> = None;
        let mut intersected_triangle_in_child_octant_distance = f64::INFINITY;

        for &(_, coi) in children_slice.iter() {
            let res = self.intersect_with_octant(octree, coi);

            if let Some(rti) = res {
                intersected_triangle_in_child_octant_distance = rti.t;
                intersected_triangle_in_child_octant = Some(rti);

                break;
            }
        }

        if intersected_triangle_in_child_octant_distance < closest_triangle_in_octant_distance {
            intersected_triangle_in_child_octant
        } else {
            intersected_triangle_in_octant
        }
    }
}
