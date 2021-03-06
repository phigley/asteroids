use nalgebra::Point2;
use winit::dpi::PhysicalSize;

pub struct Cursor {
    on_screen: bool,
    pos: Point2<f32>,
    screen_dims: (f32, f32),
}

impl Cursor {
    pub fn new(physical_size: PhysicalSize<u32>) -> Cursor {
        Cursor {
            on_screen: false,
            pos: Point2::new(0.0, 0.0),
            screen_dims: (physical_size.width as f32, physical_size.height as f32),
        }
    }

    pub fn on_screen(&self) -> bool {
        self.on_screen
    }

    pub fn get_mouse_pos(&self) -> Point2<f32> {
        self.pos
    }

    pub fn set_window_size(&mut self, physical_size: PhysicalSize<u32>) {
        // Determine our original position as percent of screen.
        let (x_pixel, y_pixel) = pos_to_pixel(self.pos, self.screen_dims.0, self.screen_dims.1);

        let x_pixel_percent = x_pixel / (self.screen_dims.0 as f32);
        let y_pixel_percent = y_pixel / (self.screen_dims.1 as f32);

        // Update our screen dims.
        self.screen_dims = (physical_size.width as f32, physical_size.height as f32);

        // Set our position to match the percent of screen.
        self.pos = pixel_to_pos(
            x_pixel_percent * self.screen_dims.0,
            y_pixel_percent * self.screen_dims.1,
            self.screen_dims.0,
            self.screen_dims.1,
        );
    }

    pub fn mouse_moved(&mut self, x_pixel: f64, y_pixel: f64) -> Option<Point2<f32>> {
        if x_pixel < 0.0
            || y_pixel < 0.0
            || x_pixel as f32 > self.screen_dims.0
            || y_pixel as f32 > self.screen_dims.1
        {
            self.on_screen = false;
            None
        } else {
            self.on_screen = true;
            self.pos = pixel_to_pos(
                x_pixel as f32,
                y_pixel as f32,
                self.screen_dims.0,
                self.screen_dims.1,
            );
            Some(self.pos)
        }
    }
}

fn pixel_to_pos(x_pixel: f32, y_pixel: f32, width: f32, height: f32) -> Point2<f32> {
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    let divisor = f32::min(half_width, half_height);

    let x = (x_pixel - half_width) / divisor;
    let y = (half_height - y_pixel) / divisor;

    Point2::new(x, y)
}

fn pos_to_pixel(pos: Point2<f32>, width: f32, height: f32) -> (f32, f32) {
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    let divisor = f32::min(half_width, half_height);

    let x = pos.x * divisor + half_width;
    let y = half_height - pos.y * divisor;

    (x, y)
}
