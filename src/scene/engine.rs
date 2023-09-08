use super::entities::{Color, Light, Sphere, Triangle};
use minifb::{Window, WindowOptions};
use std::{
    f64::INFINITY,
    ops::{Add, Div, Mul, Neg, Sub},
};

static WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};

#[derive(Copy, Clone, Debug)]
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
        let new_y = y + (self.height as i32) / 2;

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
    spheres: Vec<Sphere>,
    triangles: Vec<Triangle>,
    lights: Vec<Light>,
    origin: Vector3d,
    viewport: Viewport,
    pub canvas: Canvas,
}

impl Scene {
    pub fn new(
        width: usize,
        height: usize,
        spheres: Vec<Sphere>,
        triangles: Vec<Triangle>,
        lights: Vec<Light>,
    ) -> Scene {
        Scene {
            origin: Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            viewport: Viewport::default(),
            canvas: Canvas::new(width, height),
            spheres,
            lights,
            triangles,
        }
    }

    /// Render the provided vector of renderable items to its internal canvas,
    /// You still need to update the canvas for it to show the changes.
    pub fn draw_scene(&mut self) {
        for x in -(self.canvas.width as i32) / 2..(self.canvas.width as i32) / 2 {
            for y in -(self.canvas.height as i32) / 2..(self.canvas.height as i32) / 2 {
                let D = self.canvas_to_viewport(x as f64, y as f64);
                let color = self.trace_ray_for_triangles(self.origin, D, 1.0, INFINITY);

                // println!("x={:?}, y={:?}, color={:?}", x, y, color);

                self.canvas.put_pixel(x, y, color.into());
            }
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

    fn trace_ray_for_spheres(&self, O: Vector3d, D: Vector3d, t_min: f64, t_max: f64) -> Color {
        let mut closest_t = INFINITY;
        let mut closest_sphere = Option::<&Sphere>::None;

        for sphere in self.spheres.iter() {
            let (t1, t2) = self.intersect_ray_with_sphere(O, D, sphere);

            // println!("t1={:?}, t2={:?}", t1, t2);

            if (t_min < t1) && (t1 < t_max) && (t1 < closest_t) {
                closest_t = t1;
                closest_sphere = Some(sphere);
            }

            if (t_min < t2) && (t2 < t_max) && (t2 < closest_t) {
                closest_t = t2;
                closest_sphere = Some(sphere);
            }
        }

        if let Some(sp) = closest_sphere {
            let P = O + D * closest_t;
            let mut N = P - sp.centre;
            N = N / N.length();

            return sp.color * self.compute_lighting_intensity(&P, &N, &-D, sp.specular);
        } else {
            return WHITE; // nothing, void
        }
    }

    fn trace_ray_for_triangles(&self, O: Vector3d, D: Vector3d, t_min: f64, t_max: f64) -> Color {
        let mut closest_t = INFINITY;
        let mut closest_triangle = Option::<&Triangle>::None;

        for triangle in self.triangles.iter() {
            let t = self.intersect_ray_with_triangle(O, D, triangle);

            println!("t={t}");

            if t < closest_t {
                closest_t = t;
                closest_triangle = Some(triangle);
            }
        }

        if let Some(tri) = closest_triangle {
            return tri.color;
        } else {
            return WHITE; // nothing, void
        }
    }
    fn intersect_ray_with_sphere(&self, O: Vector3d, D: Vector3d, sphere: &Sphere) -> (f64, f64) {
        let CO: Vector3d = O - sphere.centre;

        let a = D.dot(&D);
        let b = 2.0 * CO.dot(&D);
        let c = CO.dot(&CO) - sphere.radius * sphere.radius;

        let discriminant = b * b - 4.0 * a * c;

        /* println!(
            "CO={:?}, D = {:?}, a = {:?}, b = {:?}, c={:?}, discriminant={:?}",
            CO, D, a, b, c, discriminant
        ); */

        if discriminant < 0.0 {
            return (f64::INFINITY, f64::INFINITY);
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

        (t1, t2)
    }

    fn intersect_ray_with_triangle(&self, O: Vector3d, D: Vector3d, triangle: &Triangle) -> f64 {
        let EPSILON = 0.0000001;

        let edge1 = triangle.v2 - triangle.v1;
        let edge2 = triangle.v3 - triangle.v1;
        let h = D.cross(&edge2);

        println!("h={:?}", h);

        let a = edge1.dot(&h);

        if a > -EPSILON && a < EPSILON {
            return f64::INFINITY; // This ray is parallel to this triangle.
        }

        let f = 1.0 / a;
        let s = O - triangle.v1;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return f64::INFINITY;
        }

        let q = s.cross(&edge1);
        let v = f * D.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return f64::INFINITY;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(&q);

        if t > EPSILON {
            return t;
        }

        return f64::INFINITY;
    }

    /// Given all the lights in the scene, calculate a light intensity coefficient for the point P with the normal N.
    fn compute_lighting_intensity(
        &self,
        P: &Vector3d,
        N: &Vector3d,
        V: &Vector3d,
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
                    let n_dot_l = N.dot(direction);

                    i += self.compute_diffuse_lighting_intensity(*intensity, n_dot_l, N, direction);
                    i += self
                        .compute_specular_lighting_intensity(specular, *intensity, N, V, direction);
                }
                Light::Point {
                    intensity,
                    position,
                } => {
                    let L = *position - *P;
                    let n_dot_l = N.dot(&L);

                    i += self.compute_diffuse_lighting_intensity(*intensity, n_dot_l, N, &L);
                    i += self.compute_specular_lighting_intensity(specular, *intensity, N, V, &L);
                }
            }
        }

        return i;
    }

    fn compute_diffuse_lighting_intensity(
        &self,
        intensity: f64,
        n_dot_l: f64,
        N: &Vector3d,
        L: &Vector3d,
    ) -> f64 {
        if n_dot_l <= 0.0 {
            return 0.0;
        }

        intensity * n_dot_l / (N.length() * L.length())
    }

    fn compute_specular_lighting_intensity(
        &self,
        s: f64,
        intensity: f64,
        N: &Vector3d,
        V: &Vector3d,

        L: &Vector3d,
    ) -> f64 {
        if s != -1.0 {
            let R = (*N * 2.0) * N.dot(&L) - *L;
            let r_dot_v = R.dot(&V);

            if r_dot_v > 0.0 {
                return intensity * (r_dot_v / (R.length() * V.length())).powf(s);
            }
        }

        0.0
    }
}
