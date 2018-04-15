extern crate cgmath;
extern crate graphics;

use graphics::{color, events, model, screen};

use cgmath::prelude::*;
use cgmath::{Matrix4, Point2};

fn main() {
    let mut screen = match screen::Screen::create("Triangle") {
        Err(create_error) => panic!(create_error.to_string()),
        Ok(created_screen) => created_screen,
    };

    let translation: Matrix4<f32> = Matrix4::identity();

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

        screen.poll_events(|event| match event {
            events::Event::Exit => {
                should_exit = true;
            }
            _ => (),
        });
    }
}
