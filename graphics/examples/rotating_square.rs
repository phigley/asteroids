use graphics::{color, events, model, screen, FrameTimer};

use nalgebra::{Point2, Similarity2, Vector2};

fn main() {
    let mut screen = match screen::Screen::create(800.0, 600.0, "Rotating Square") {
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

    let mut frame_timer = FrameTimer::new();
    let period_ms = 5000;

    let mut should_exit = false;
    while !should_exit {
        frame_timer.update(10, 0.1);
        let delta_time_ms = frame_timer.elapsed().num_milliseconds();

        let delta_period = delta_time_ms % period_ms;
        let delta_fraction = (delta_period as f32) / (period_ms as f32);
        let current_angle = -std::f32::consts::PI * 2.0 * delta_fraction;

        model.transform = Similarity2::new(Vector2::new(0.0f32, 0.0f32), current_angle, 1.0f32);

        screen.clear(clear_color);
        screen.draw_model(&model);
        screen.flush();

        screen.poll_events(|event| {
            if let events::Event::Exit = event {
                should_exit = true;
            }
        });
    }
}
