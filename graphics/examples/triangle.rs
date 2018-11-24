


use graphics::{color, events, model, screen};

use nalgebra::{Point2, Similarity2};

fn main() {
    let mut screen = match screen::Screen::create(800.0, 600.0, "Triangle") {
        Err(create_error) => panic!(create_error.to_string()),
        Ok(created_screen) => created_screen,
    };

    let translation: Similarity2<f32> = Similarity2::identity();

    let indices = [0, 1, 2];
    let verts = [
        Point2::new(-0.5, -0.5),
        Point2::new(0.5, -0.5),
        Point2::new(0.0, 0.5),
    ];
    let shape = screen.create_shape(&verts, &indices);

    let yellow = color::Color::new(1.0, 1.0, 0.0, 1.0);

    let model = model::Model::new(&shape, yellow, translation);

    let clear_color = color::Color::new(0.1, 0.2, 0.3, 1.0);

    let mut should_exit = false;
    while !should_exit {
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
