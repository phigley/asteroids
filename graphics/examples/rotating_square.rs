use graphics::{color, model, screen};

use anyhow::Result;
use nalgebra::{Point2, Similarity2, Vector2};
use std::time::Duration;

struct App {
    model: model::Model,
    time: Duration,
}

impl App {
    fn new(model: model::Model) -> Self {
        Self {
            model,
            time: Duration::new(0, 0),
        }
    }
}

impl screen::ScreenCallbacks for App {
    fn update(&mut self, _screen: &mut screen::Screen, frame_delta: Duration) {
        let period = Duration::from_secs(5);

        self.time += frame_delta;

        while self.time >= period {
            self.time -= period;
        }

        let delta_fraction = self.time.div_f32(period.as_secs_f32()).as_secs_f32();
        let current_angle = -std::f32::consts::PI * 2.0 * delta_fraction;

        self.model.transform =
            Similarity2::new(Vector2::new(0.0f32, 0.0f32), current_angle, 1.0f32);
    }

    fn render(&self, mut screen_render: screen::ScreenRender) {
        screen_render.draw_model(&self.model);
    }
}

fn main() -> Result<()> {
    let clear_color = color::Color::new(0.1, 0.2, 0.3, 1.0);

    let mut runner = screen::ScreenRunner::create(800.0, 600.0, "Rotating Square", clear_color)?;

    let indices = [0, 1, 2, 0, 2, 3];

    let verts = [
        Point2::new(-0.5, -0.5),
        Point2::new(0.5, -0.5),
        Point2::new(0.5, 0.5),
        Point2::new(-0.5, 0.5),
    ];

    let shape = runner.screen.create_shape(&verts, &indices, "square");
    let yellow = color::Color::new(1.0, 1.0, 0.0, 1.0);
    let model = model::Model::new(shape, yellow, Similarity2::identity());

    runner.run(App::new(model));
}
