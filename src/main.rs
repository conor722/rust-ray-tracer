mod collision;
mod file_management;
mod scene;

use std::time::Instant;
use std::{fs, vec};

use minifb::Key;
use scene::engine::{Scene, Vector3d};
use scene::entities::Light;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() {
    let mut args = std::env::args();

    args.next();

    let file_name = args
        .next()
        .expect("First argument needs to be the name of a file with vertex and triangle data");

    println!("using model file: {file_name}");

    let file = fs::read_to_string(file_name).expect("Could not read file");

    let triangles = file_management::utils::parse_lines(file.lines());

    let lights = vec![
        Light::Ambient { intensity: 0.4 },
        Light::Point {
            intensity: 0.7,
            position: Vector3d {
                x: 2.0,
                y: 2.0,
                z: 0.0,
            },
        },
        Light::Directional {
            intensity: 0.5,
            direction: Vector3d {
                x: -5.0,
                y: 0.0,
                z: 2.0,
            },
        },
    ];

    let mut scene = Scene::new(WIDTH, HEIGHT, triangles, lights);

    // Limit to max ~60 fps update rate
    scene
        .canvas
        .window
        .limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let now = Instant::now();
    scene.draw_scene();
    let elapsed = now.elapsed();
    println!("It took: {:.2?} to draw the scene", elapsed);

    println!("draw finished");

    while scene.canvas.window.is_open() && !scene.canvas.window.is_key_down(Key::Escape) {
        scene.canvas.update();
    }
}
