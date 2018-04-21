extern crate graphics;
extern crate nalgebra;

use graphics::{color, events, model, screen};

use nalgebra::Similarity2;

fn main() {
    let mut screen = match screen::Screen::create("Circle") {
        Err(create_error) => panic!(create_error.to_string()),
        Ok(created_screen) => created_screen,
    };

    let translation = Similarity2::identity();

    let num_vertices = 128;
    let shape = screen.create_circle(0.5, num_vertices);

    let yellow = color::Color::new(1.0, 1.0, 0.0, 1.0);

    let model = model::Model::new(&shape, yellow, translation);

    let clear_color = color::Color::new(0.1, 0.2, 0.3, 1.0);

    let mut should_exit = false;
    while !should_exit {
        screen.clear(clear_color);
        screen.draw_model(&model);
        screen.flush();

        screen.poll_events(|event| match event {
            events::Event::Exit => should_exit = true,
            _ => (),
        });
    }
}
