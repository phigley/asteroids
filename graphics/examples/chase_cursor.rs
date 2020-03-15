use graphics::{color, events, model, screen};

use anyhow::Result;
use nalgebra::{Point2, Similarity2, Vector2};
use std::time::Duration;
use std::vec::Vec;

struct App {
    chase_model: model::Model,
    follow_model: model::Model,
    fixed_model: model::Model,

    follow_scale: f32,
    chase_scale: f32,

    catchup_percent: f32,
    catchup_delay: f32,

    current_color: usize,
    colors: Vec<color::Color>,

    previous_mouse_pos: Point2<f32>,
}

impl App {
    fn new(screen: &mut screen::Screen) -> Self {
        let shape = screen.create_circle(0.02, 64, "Circle");

        let fixed_scale = 1.5;
        let chase_scale = 1.0;
        let follow_scale = 0.5;
        let chase_color = color::Color::new(1.0, 1.0, 1.0, 0.5);
        let chase_model = model::Model::new(
            shape.clone(),
            chase_color,
            Similarity2::from_scaling(chase_scale),
        );
        let follow_color = color::Color::new(1.0, 1.0, 0.0, 0.5);
        let follow_model = model::Model::new(
            shape.clone(),
            follow_color,
            Similarity2::from_scaling(follow_scale),
        );
        let colors = vec![
            color::Color::new(1.0, 0.0, 0.0, 1.0),
            color::Color::new(0.0, 1.0, 0.0, 1.0),
            color::Color::new(0.0, 0.0, 1.0, 1.0),
        ];

        let fixed_translation =
            Similarity2::new(Vector2::new(0.25f32, 0.25f32), 0.0f32, fixed_scale);
        let fixed_model = model::Model::new(shape, colors[0], fixed_translation);
        // catchup_percent of the distance is remaining after catchup_delay time has passed.
        let catchup_percent = 0.05f32;
        let catchup_delay = 1.0f32;

        let previous_mouse_pos = screen.get_mouse_pos();

        Self {
            chase_model,
            follow_model,
            fixed_model,

            follow_scale,
            chase_scale,

            catchup_percent,
            catchup_delay,

            current_color: 0,
            colors,

            previous_mouse_pos,
        }
    }
}

impl screen::ScreenCallbacks for App {
    fn handle_event(&mut self, _screen: &mut screen::Screen, event: events::Event) {
        match event {
            events::Event::Resize { mouse_pos } => {
                self.follow_model.transform =
                    Similarity2::new(mouse_pos.coords, 0.0f32, self.follow_scale);
            }
            events::Event::MouseMove { pos } => {
                self.follow_model.transform =
                    Similarity2::new(pos.coords, 0.0f32, self.follow_scale);
            }
            events::Event::MouseLMB { down } => {
                if !down {
                    self.current_color += 1;
                    if self.current_color >= self.colors.len() {
                        self.current_color = 0;
                    }

                    self.fixed_model.color = self.colors[self.current_color];
                }
            }
            _ => (),
        }
    }

    fn update(&mut self, screen: &mut screen::Screen, frame_delta: Duration) {
        let current_mouse_pos = screen.get_mouse_pos();

        let full_delta_pos = current_mouse_pos - self.previous_mouse_pos;

        let interp_percent = if full_delta_pos.norm() > 1e-8f32 {
            1.0 - self
                .catchup_percent
                .powf(frame_delta.as_secs_f32() / self.catchup_delay)
        } else {
            1.0f32
        };

        let new_chase_pos = self.previous_mouse_pos + full_delta_pos * interp_percent;

        self.chase_model.transform =
            Similarity2::new(new_chase_pos.coords, 0.0f32, self.chase_scale);

        self.previous_mouse_pos = new_chase_pos;
    }

    fn render(&self, mut screen_render: screen::ScreenRender) {
        screen_render.draw_model(&self.fixed_model);
        screen_render.draw_model(&self.chase_model);
        screen_render.draw_model(&self.follow_model);
    }
}

fn main() -> Result<()> {
    let clear_color = color::Color::new(0.1, 0.2, 0.3, 1.0);
    let mut runner = screen::ScreenRunner::create(800.0, 600.0, "Chase Cursor", clear_color)?;

    let app = App::new(&mut runner.screen);
    runner.run(app);
}
