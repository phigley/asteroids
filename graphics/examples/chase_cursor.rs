extern crate cgmath;
extern crate graphics;
extern crate time;

use graphics::{color, events, model, screen};

use cgmath::prelude::*;
use cgmath::{vec3, Matrix4};

use time::PreciseTime;

use std::f32;

fn main() {
    let mut screen = match screen::Screen::create("Chase Cursor") {
        Err(create_error) => panic!(create_error.to_string()),
        Ok(created_screen) => created_screen,
    };

    let shape = screen.create_circle(0.02, 64);

    let fixed_scale = Matrix4::from_scale(1.5);
    let chase_scale = Matrix4::from_scale(1.0);
    let follow_scale = Matrix4::from_scale(0.5);

    let chase_color = color::Color::new(1.0, 1.0, 1.0, 0.5);
    let mut chase_model = model::Model::new(&shape, chase_color, chase_scale);

    let follow_color = color::Color::new(1.0, 1.0, 0.0, 0.5);
    let mut follow_model = model::Model::new(&shape, follow_color, follow_scale);

    let colors = [
        color::Color::new(1.0, 0.0, 0.0, 1.0),
        color::Color::new(0.0, 1.0, 0.0, 1.0),
        color::Color::new(0.0, 0.0, 1.0, 1.0),
    ];

    let mut current_color = 0;

    let fixed_translation: Matrix4<f32> =
        Matrix4::from_translation(vec3(0.25, 0.25, 0.0)) * fixed_scale;
    let mut fixed_model = model::Model::new(&shape, colors[current_color], fixed_translation);

    let clear_color = color::Color::new(0.1, 0.2, 0.3, 1.0);

    // catchup_percent of the distance is remaining after catchup_delay time has passed.
    // time is in ms
    let catchup_percent = 0.05f32;
    let catchup_delay = 1000.0f32;

    let mut previous_mouse_pos = screen.get_mouse_pos();
    let mut previous_time = PreciseTime::now();

    let mut should_exit = false;
    while !should_exit {
        screen.clear(clear_color);
        screen.draw_model(&fixed_model);
        screen.draw_model(&chase_model);
        screen.draw_model(&follow_model);
        screen.flush();

        screen.poll_events(|event| match event {
            events::Event::Exit => should_exit = true,
            events::Event::Resize { mouse_pos } => {
                follow_model.transform =
                    Matrix4::from_translation(mouse_pos.to_vec().extend(0.0)) * follow_scale;
            }
            events::Event::MouseMove { pos } => {
                follow_model.transform =
                    Matrix4::from_translation(pos.to_vec().extend(0.0)) * follow_scale;
            }
            events::Event::MouseLMB { down } => {
                if down == false {
                    current_color += 1;
                    if current_color >= colors.len() {
                        current_color = 0;
                    }

                    fixed_model.color = colors[current_color];
                }
            }
            _ => (),
        });

        let current_time = PreciseTime::now();
        let current_mouse_pos = screen.get_mouse_pos();

        let full_delta_pos = current_mouse_pos - previous_mouse_pos;

        let interp_percent = if full_delta_pos.magnitude2() > 1e-8f32 {
            let time_delta = previous_time.to(current_time).num_milliseconds() as f32;
            1.0 - catchup_percent.powf(time_delta / catchup_delay)
        } else {
            1.0f32
        };

        let new_chase_pos = previous_mouse_pos + full_delta_pos * interp_percent;

        chase_model.transform =
            Matrix4::from_translation(new_chase_pos.to_vec().extend(0.0)) * chase_scale;

        previous_mouse_pos = new_chase_pos;
        previous_time = current_time;
    }
}
