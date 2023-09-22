use super::{entities::Color, raytracer::RayTracer};
use minifb::{Window, WindowOptions};
use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    sync::{mpsc, Arc},
    thread::available_parallelism,
};
use threadpool::ThreadPool;

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
    viewport: Viewport,
    pub canvas: Canvas,
}

impl Scene {
    pub fn new(width: usize, height: usize) -> Scene {
        Scene {
            viewport: Viewport::default(),
            canvas: Canvas::new(width, height),
        }
    }

    /// Render the provided vector of renderable items to its internal canvas,
    /// You still need to update the canvas for it to show the changes.
    pub fn draw_scene(&mut self, rt: RayTracer) {
        let rt_arc = Arc::new(rt);
        let ap = usize::from(available_parallelism().unwrap());

        println!("Going to trace scene with {ap} threads");

        let tp = ThreadPool::new(ap);

        let x_scale = self.viewport.width / self.canvas.width as f64;
        let y_scale = self.viewport.height / self.canvas.height as f64;
        let z_value = self.viewport.distance;
        let height = self.canvas.height as i32;
        let width = self.canvas.width as i32;

        let (tx, rx) = mpsc::channel();

        for x in -(width as i32) / 2..(width as i32) / 2 {
            for y in -(height as i32) / 2..(height as i32) / 2 {
                let rt_arc_c = rt_arc.clone();
                let tx_clone = tx.clone();

                // Put the trace function for the ray at this point into a threadpool and go
                tp.execute(move || {
                    let direction1 = Vector3d {
                        x: x as f64 * x_scale,
                        y: y as f64 * y_scale,
                        z: z_value,
                    };

                    // We are going to split the (x, y) pair into corners and render a ray for each corner,
                    // this makes the end render result look less jagged (a form of anti aliasing)
                    let color1 = rt_arc_c.get_ray_colour(rt_arc_c.origin, direction1);

                    let direction2 = Vector3d {
                        x: (x as f64 + 0.5) * x_scale,
                        y: y as f64 * y_scale,
                        z: z_value,
                    };
                    let color2 = rt_arc_c.get_ray_colour(rt_arc_c.origin, direction2);

                    let direction3 = Vector3d {
                        x: x as f64 * x_scale,
                        y: (y as f64 + 0.5) * y_scale,
                        z: z_value,
                    };
                    let color3 = rt_arc_c.get_ray_colour(rt_arc_c.origin, direction3);

                    let direction4 = Vector3d {
                        x: (x as f64 + 0.5) * x_scale,
                        y: (y as f64 + 0.5) * y_scale,
                        z: z_value,
                    };
                    let color4 = rt_arc_c.get_ray_colour(rt_arc_c.origin, direction4);

                    let final_color = Color::mix(&vec![color1, color2, color3, color4]);

                    tx_clone.send((y, x, final_color)).unwrap();
                })
            }
        }

        // Need to drop so the receiver will eventually terminate.
        drop(tx);

        let mut ctr = 0;

        for received in rx {
            ctr += 1;
            let (y, x, col) = received;
            self.canvas.put_pixel(x, y, col.into());

            // It looks cooler if we update the canvas during rendering, but
            // it slows down the rendering a lot, so do it per 8000 pixels.
            if ctr % 8000 == 0 {
                self.canvas.update();
            }
        }

        self.canvas.update()
    }
}
