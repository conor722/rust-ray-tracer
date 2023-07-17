mod scene;

use minifb::Key;
use scene::engine::{Scene, Sphere, Vector3d};
use scene::entities::Light;

use crate::scene::entities::Color;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() {
    let sp = Sphere {
        radius: 40.0,
        centre: Vector3d {
            x: 0.0,
            y: 0.0,
            z: 400.0,
        },
        color: Color { r: 0, g: 255, b: 0 },
    };

    let lights = vec![
        Light::Ambient { intensity: 0.2 },
        Light::Point {
            intensity: 0.4,
            position: Vector3d {
                x: 2.0,
                y: 1.0,
                z: 0.0,
            },
        },
        Light::Directional {
            intensity: 0.2,
            direction: Vector3d {
                x: 1.0,
                y: 4.0,
                z: 4.0,
            },
        },
    ];

    let m: u32 = 34534654;
    let d = m as f64;
    println!("{:?}", d * 0.5);

    let mut scene = Scene::new(WIDTH, HEIGHT, vec![sp], lights);

    let c = Color { r: 255, b: 0, g: 0 };
    let n: u32 = c.into();

    println!("converted={:x}", n);

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
