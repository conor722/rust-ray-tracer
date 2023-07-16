mod engine;

use engine::canvas::Canvas;
use minifb::Key;

const WIDTH: usize = 1280;
const HEIGHT: usize = 800;

fn main() {
    let mut canvas: Canvas = Canvas::new(WIDTH, HEIGHT);
    let mut counter: u32 = 0;

    // Limit to max ~60 fps update rate
    canvas
        .window
        .limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while canvas.window.is_open() && !canvas.window.is_key_down(Key::Escape) {
        for x in -640..640 {
            for y in -400..400 {
                canvas.put_pixel(x, y, counter);
                counter += 1;
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        canvas.update();
    }
}
