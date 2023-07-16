mod scene;

use minifb::Key;
use scene::engine::{Scene, Sphere, Vector3d};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() {
    let sp = Sphere {
        radius: 1.0,
        centre: Vector3d {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        },
        color: 0xFF00,
    };

    let mut scene = Scene::new(WIDTH, HEIGHT, vec![sp]);

    let v = Vector3d {
        x: 3.0,
        y: 2.0,
        z: 10.0,
    };

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
