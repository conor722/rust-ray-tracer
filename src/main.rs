mod file_management;
mod scene;

use std::vec;

use minifb::Key;
use scene::engine::{Scene, Vector3d};
use scene::entities::{Light, Sphere, Triangle};

use crate::scene::entities::Color;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() {
    let spheres = vec![
        Sphere {
            radius: 40.0,
            centre: Vector3d {
                x: 0.0,
                y: 0.0,
                z: 400.0,
            },
            color: Color { r: 0, g: 255, b: 0 },
            specular: 10.0,
        },
        Sphere {
            radius: 20.0,
            centre: Vector3d {
                x: 40.0,
                y: 0.0,
                z: 240.0,
            },
            color: Color {
                r: 0,
                g: 128,
                b: 255,
            },
            specular: 1000.0,
        },
        Sphere {
            radius: 30.0,
            centre: Vector3d {
                x: -60.0,
                y: 20.0,
                z: 240.0,
            },
            color: Color { r: 255, g: 0, b: 0 },
            specular: 2000.0,
        },
    ];

    let triangles = vec![
        Triangle {
            v1: Vector3d {
                x: -20.0,
                y: 0.0,
                z: 120.0,
            },
            v2: Vector3d {
                x: 20.0,
                y: 0.0,
                z: 120.0,
            },
            v3: Vector3d {
                x: 0.0,
                y: 20.0,
                z: 120.0,
            },
            color: Color { r: 255, g: 0, b: 0 },
            specular: 240.0,
        },
        Triangle {
            v1: Vector3d {
                x: 20.0,
                y: 0.0,
                z: 120.0,
            },
            v2: Vector3d {
                x: 50.0,
                y: 0.0,
                z: 140.0,
            },
            v3: Vector3d {
                x: 20.0,
                y: 20.0,
                z: 150.0,
            },
            color: Color { r: 0, g: 255, b: 0 },
            specular: 240.0,
        },
    ];

    let lights = vec![
        Light::Ambient { intensity: 0.2 },
        Light::Point {
            intensity: 0.6,
            position: Vector3d {
                x: 2.0,
                y: 2.0,
                z: 0.0,
            },
        },
        Light::Directional {
            intensity: 0.2,
            direction: Vector3d {
                x: -5.0,
                y: 0.0,
                z: 2.0,
            },
        },
    ];

    let mut scene = Scene::new(WIDTH, HEIGHT, spheres, triangles, lights);

    // Limit to max ~60 fps update rate
    scene
        .canvas
        .window
        .limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    scene.draw_scene();

    while scene.canvas.window.is_open() && !scene.canvas.window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        scene.canvas.update();
    }
}
