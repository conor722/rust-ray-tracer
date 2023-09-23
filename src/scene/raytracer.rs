use crate::collision::ray::{Ray, RayTriangleIntersectionResult};

use super::{
    engine::Vector3d,
    entities::{Color, Light},
    material::Material,
    scenedata::SceneData,
};

static WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};

/// A tiny delta to shift the origin point by when checking for triangles in between two points.
/// so we don't just return the value at the original point.
static ORIGIN_SHIFT_AMOUNT_FOR_FINDING_TRIANGLES_BETWEEN_POINTS: f64 = 0.0001;

pub struct RayTracer {
    pub scene_data: SceneData,
    pub lights: Vec<Light>,
    pub origin: Vector3d,
}

impl RayTracer {
    pub fn get_ray_colour(&self, origin: Vector3d, direction: Vector3d) -> Color {
        let ray = Ray { origin, direction };

        let triangle_intersection = ray.intersect_with_octant(&self.scene_data.octree, 0);

        if let Some(intersection) = triangle_intersection {
            let p = origin + direction * intersection.t;

            let tex = &intersection.triangle.material.texture;

            let w = 1.0 - intersection.u - intersection.v;

            let tex_x = intersection.triangle.v2_tex_coords.x * intersection.u
                + intersection.triangle.v3_tex_coords.x * intersection.v
                + intersection.triangle.v1_tex_coords.x * w;
            let tex_y = intersection.triangle.v2_tex_coords.y * intersection.u
                + intersection.triangle.v3_tex_coords.y * intersection.v
                + intersection.triangle.v1_tex_coords.y * w;

            let tex_x_index = ((tex_x * tex.width as f64) as usize) % tex.width;
            let tex_y_index = ((tex_y * tex.height as f64) as usize) % tex.height;

            let col = tex.colours[tex.width * tex_y_index + tex_x_index];

            let n = self.get_normal_at_intersection(&intersection, tex_x_index, tex_y_index);

            let lighting_intensity = self.compute_lighting_intensity(
                &p,
                &n,
                &-direction,
                &intersection.triangle.material,
            );

            return Color {
                r: (col.r as f64 * lighting_intensity.x) as u8,
                g: (col.g as f64 * lighting_intensity.y) as u8,
                b: (col.b as f64 * lighting_intensity.z) as u8,
            };
        } else {
            return WHITE; // nothing, void
        }
    }

    pub fn get_normal_at_intersection(
        &self,
        intersection: &RayTriangleIntersectionResult,
        tex_x_index: usize,
        tex_y_index: usize,
    ) -> Vector3d {
        let w = 1.0 - intersection.u - intersection.v;

        let mut n = intersection.triangle.v2_normal_coords * intersection.u
            + intersection.triangle.v3_normal_coords * intersection.v
            + intersection.triangle.v1_normal_coords * w;

        if let Some(bump_map) = &intersection.triangle.material.bump_map {
            let mut bump_vector: Vector3d =
                bump_map.colours[bump_map.width * tex_y_index + tex_x_index].into();
            bump_vector = bump_vector.normalised();
            bump_vector = (bump_vector * 2.0)
                - Vector3d {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                };

            let mut t = n.cross(&Vector3d {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });

            if t.length() == 0.0 {
                t = n.cross(&Vector3d {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                });
            }

            t = t.normalised();
            let b = n.cross(&t).normalised();

            n = Vector3d {
                x: bump_vector.dot(&t),
                y: bump_vector.dot(&b),
                z: bump_vector.dot(&n),
            };
        }

        return n.normalised();
    }

    fn triangle_exists_between_points(&self, origin: &Vector3d, target: &Vector3d) -> bool {
        let direction = *target - *origin;
        let new_origin =
            *origin + (direction * ORIGIN_SHIFT_AMOUNT_FOR_FINDING_TRIANGLES_BETWEEN_POINTS);

        let ray = Ray {
            origin: new_origin,
            direction: direction,
        };

        let max_t = direction.length();

        let tri = ray.intersect_with_octant_with_max_t(&self.scene_data.octree, 0, max_t);

        if let None = tri {
            return true;
        }

        return false;
    }

    /// Given all the lights in the scene, calculate a vector of intensities
    /// for an r, g, b colour value.
    fn compute_lighting_intensity(
        &self,
        point: &Vector3d,
        normal: &Vector3d,
        v: &Vector3d,
        material: &Material,
    ) -> Vector3d {
        let mut i = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        for light in &self.lights {
            match light {
                Light::Ambient { intensity } => {
                    i += material.ambient_color_coefficient * *intensity;
                }
                Light::Directional {
                    intensity,
                    direction,
                } => {
                    let n_dot_l = normal.dot(direction);

                    i += self.compute_diffuse_lighting_intensity(
                        *intensity, n_dot_l, normal, direction, material,
                    );
                    i += self.compute_specular_lighting_intensity(
                        material.specular_weight,
                        *intensity,
                        normal,
                        v,
                        direction,
                        material,
                    );
                }
                Light::Point {
                    intensity,
                    position,
                } => {
                    let light_hits_point = self.triangle_exists_between_points(point, position);

                    if !light_hits_point {
                        break;
                    }

                    let l = *position - *point;
                    let n_dot_l = normal.dot(&l);

                    i += self.compute_diffuse_lighting_intensity(
                        *intensity, n_dot_l, normal, &l, material,
                    );
                    i += self.compute_specular_lighting_intensity(
                        material.specular_weight,
                        *intensity,
                        normal,
                        v,
                        &l,
                        material,
                    );
                }
            }
        }

        return i;
    }

    fn compute_diffuse_lighting_intensity(
        &self,
        intensity: f64,
        n_dot_l: f64,
        normal: &Vector3d,
        l: &Vector3d,
        material: &Material,
    ) -> Vector3d {
        if n_dot_l <= 0.0 {
            return Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }

        material.diffuse_color_coefficient * intensity * n_dot_l / (normal.length() * l.length())
    }

    fn compute_specular_lighting_intensity(
        &self,
        s: f64,
        intensity: f64,
        normal: &Vector3d,
        v: &Vector3d,
        l: &Vector3d,
        material: &Material,
    ) -> Vector3d {
        if s != -1.0 {
            let r = (*normal * 2.0) * normal.dot(&l) - *l;
            let r_dot_v = r.dot(&v);

            if r_dot_v > 0.0 {
                return material.specular_color_coefficient
                    * intensity
                    * (r_dot_v / (r.length() * v.length())).powf(s);
            }
        }

        Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
