use minifb::{Window, WindowOptions, Scale, Key};

pub struct WindowHandler {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl WindowHandler {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                scale: Scale::X1,
                ..WindowOptions::default()
            },
        )
        .expect("Unable to open window");

        let buffer = vec![0; width * height];

        WindowHandler {
            window,
            buffer,
            width,
            height,
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn update(&mut self) {
        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
        self.buffer.fill(0); // Clear the buffer after each frame
    }

    pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: u32) {
        let mut x = radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            self.draw_pixel(center_x + x, center_y + y, color);
            self.draw_pixel(center_x + y, center_y + x, color);
            self.draw_pixel(center_x - y, center_y + x, color);
            self.draw_pixel(center_x - x, center_y + y, color);
            self.draw_pixel(center_x - x, center_y - y, color);
            self.draw_pixel(center_x - y, center_y - x, color);
            self.draw_pixel(center_x + y, center_y - x, color);
            self.draw_pixel(center_x + x, center_y - y, color);

            y += 1;
            err += 1 + 2 * y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }
    }

    fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.buffer[(y as usize) * self.width + (x as usize)] = color;
        }
    }

    pub fn get_mouse_pos(&self) -> Option<(f32, f32)> {
        self.window.get_mouse_pos(minifb::MouseMode::Discard)
    }

    pub fn get_mouse_down(&self, button: minifb::MouseButton) -> bool {
        self.window.get_mouse_down(button)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}