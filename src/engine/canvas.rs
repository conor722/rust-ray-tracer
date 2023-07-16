use minifb::{Window, WindowOptions};

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
