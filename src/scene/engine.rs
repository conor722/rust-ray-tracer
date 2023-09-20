use crate::{collision::ray::Ray, file_management::utils::SceneData};

use super::entities::{Color, Light, Triangle};
use minifb::{Window, WindowOptions};
use std::ops::{Add, Div, Mul, Neg, Sub};

static WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vector3d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3d {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector3d {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Neg for Vector3d {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Div<f64> for Vector3d {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Vector3d {
    pub fn dot(&self, other: &Self) -> f64 {
        return (self.x * other.x) + (self.y * other.y) + (self.z * other.z);
    }

    pub fn length(&self) -> f64 {
        return f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2));
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vector3d {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[derive(Copy, Clone)]
pub struct IntersectionResult<'a> {
    t: f64,
    u: f64,
    v: f64,
    triangle: &'a Triangle,
}

struct Viewport {
    width: f64,
    height: f64,
    distance: f64,
}

impl Viewport {
    pub fn default() -> Self {
        Viewport {
            width: 1.0,
            height: 1.0,
            distance: 1.0,
        }
    }
}

/// A very simple canvas that can be drawn to and rendered
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub window: Window,
    buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            buffer: vec![0; width * height],
            window: Window::new("Hello", width, height, WindowOptions::default()).unwrap_or_else(
                |e| {
                    panic!("{}", e);
                },
            ),
        }
    }

    /// Put the color at the coordinate given by (x, y) using normal coordinates.
    /// i.e (0,0) is the pixel in the centre of the screen.
    pub fn put_pixel(&mut self, x: i32, y: i32, color: u32) {
        let new_x = x + (self.width as i32) / 2;

        // Minus from self.height as y=0 is the top of the screen, if we don't the image will be upside down.
        let new_y = self.height as i32 - (y + (self.height as i32) / 2);

        if new_x < 0 || new_x >= self.width as i32 || new_y < 0 || new_y >= self.height as i32 {
            // Coordinates are out of bounds (will crash if we try to use these as buffer coords)
            return;
        }

        self.buffer[new_y as usize * self.width + new_x as usize] = color;
    }

    /// Draw the current buffer to the screen.
    /// Call this to make your changes actually do something.
    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }
}

/// The entrypoint class for the engine, encapsulates all entities and main classes needed to raycast a scene.
/// The internal canvas is where the actual pixels will reside after drawing the scene.
pub struct Scene {
    scene_data: SceneData,
    lights: Vec<Light>,
    origin: Vector3d,
    viewport: Viewport,
    pub canvas: Canvas,
}

impl Scene {
    pub fn new(width: usize, height: usize, scene_data: SceneData, lights: Vec<Light>) -> Scene {
        Scene {
            origin: Vector3d {
                x: 0.0,
                y: 0.0,
                z: -60.0,
            },
            viewport: Viewport::default(),
            canvas: Canvas::new(width, height),
            lights,
            scene_data,
        }
    }

    /// Render the provided vector of renderable items to its internal canvas,
    /// You still need to update the canvas for it to show the changes.
    pub fn draw_scene(&mut self) {
        for x in -(self.canvas.width as i32) / 2..(self.canvas.width as i32) / 2 {
            for y in -(self.canvas.height as i32) / 2..(self.canvas.height as i32) / 2 {
                let direction = self.canvas_to_viewport(x as f64, y as f64);
                let color = self.trace_ray_for_triangles(self.origin, direction);

                self.canvas.put_pixel(x, y, color.into());

                // println!("finished drawing ray at {x}, {y}");
            }
            self.canvas.update();
        }
    }

    fn viewport_x(&self, x: f64) -> f64 {
        x * (self.viewport.width / self.canvas.width as f64)
    }

    fn viewport_y(&self, y: f64) -> f64 {
        y * (self.viewport.height / self.canvas.height as f64)
    }

    fn canvas_to_viewport(&self, x: f64, y: f64) -> Vector3d {
        Vector3d {
            x: self.viewport_x(x),
            y: self.viewport_y(y),
            z: self.viewport.distance,
        }
    }

    fn trace_ray_for_triangles(&self, origin: Vector3d, direction: Vector3d) -> Color {
        let ray = Ray { origin, direction };

        let triangle_intersection = ray.intersect_with_octant(&self.scene_data.octree, 0);

        if let Some(intersection) = triangle_intersection {
            let p = origin + direction * intersection.t;

            let tex = &self.scene_data.textures[intersection.triangle.texture_index];

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

            let n = intersection.triangle.v2_normal_coords * intersection.u
                + intersection.triangle.v3_normal_coords * intersection.v
                + intersection.triangle.v1_normal_coords * w;

            return col
                * self.compute_lighting_intensity(
                    &p,
                    &n,
                    &-direction,
                    intersection.triangle.specular,
                );
        } else {
            return WHITE; // nothing, void
        }
    }

    /// Given all the lights in the scene, calculate a light intensity coefficient for the point P with the normal N.
    fn compute_lighting_intensity(
        &self,
        point: &Vector3d,
        normal: &Vector3d,
        v: &Vector3d,
        specular: f64,
    ) -> f64 {
        let mut i: f64 = 0.0;

        for light in &self.lights {
            match light {
                Light::Ambient { intensity } => {
                    i += *intensity;
                }
                Light::Directional {
                    intensity,
                    direction,
                } => {
                    let n_dot_l = normal.dot(direction);

                    i += self
                        .compute_diffuse_lighting_intensity(*intensity, n_dot_l, normal, direction);
                    i += self.compute_specular_lighting_intensity(
                        specular, *intensity, normal, v, direction,
                    );
                }
                Light::Point {
                    intensity,
                    position,
                } => {
                    let l = *position - *point;
                    let n_dot_l = normal.dot(&l);

                    i += self.compute_diffuse_lighting_intensity(*intensity, n_dot_l, normal, &l);
                    i += self
                        .compute_specular_lighting_intensity(specular, *intensity, normal, v, &l);
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
    ) -> f64 {
        if n_dot_l <= 0.0 {
            return 0.0;
        }

        intensity * n_dot_l / (normal.length() * l.length())
    }

    fn compute_specular_lighting_intensity(
        &self,
        s: f64,
        intensity: f64,
        normal: &Vector3d,
        v: &Vector3d,
        l: &Vector3d,
    ) -> f64 {
        if s != -1.0 {
            let r = (*normal * 2.0) * normal.dot(&l) - *l;
            let r_dot_v = r.dot(&v);

            if r_dot_v > 0.0 {
                return intensity * (r_dot_v / (r.length() * v.length())).powf(s);
            }
        }

        0.0
    }
}
