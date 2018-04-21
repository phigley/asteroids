extern crate graphics;
extern crate nalgebra;
extern crate time;

use graphics::{color, events, model, screen};

use nalgebra::{Point2, Similarity2, Vector2};

use time::PreciseTime;

fn main() {
    let mut screen = match screen::Screen::create("Rotating Square") {
        Err(create_error) => panic!(create_error.to_string()),
        Ok(created_screen) => created_screen,
    };

    let indices = [0, 1, 2, 0, 2, 3];

    let verts = [
        Point2::new(-0.5, 0.5),
        Point2::new(0.5, 0.5),
        Point2::new(0.5, -0.5),
        Point2::new(-0.5, -0.5),
    ];

    let shape = screen.create_shape(&verts, &indices);

    let yellow = color::Color::new(1.0, 1.0, 0.0, 1.0);

    let mut model = model::Model::new(&shape, yellow, Similarity2::identity());

    let clear_color = color::Color::new(0.1, 0.2, 0.3, 1.0);

    let start_time = PreciseTime::now();
    let period_ms = 5000;

    let mut should_exit = false;
    while !should_exit {
        let current_time = PreciseTime::now();
        let delta_time_ms = start_time.to(current_time).num_milliseconds();

        let delta_period = delta_time_ms % period_ms;
        let delta_fraction = (delta_period as f32) / (period_ms as f32);
        let current_angle = -std::f32::consts::PI * 2.0 * delta_fraction;

        model.transform = Similarity2::new(Vector2::new(0.0f32, 0.0f32), current_angle, 1.0f32);

        screen.clear(clear_color);
        screen.draw_model(&model);
        screen.flush();

        screen.poll_events(|event| match event {
            events::Event::Exit => should_exit = true,
            _ => (),
        });
    }
}
