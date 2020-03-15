use graphics::{color, model, screen};

use anyhow::Result;
use nalgebra::Similarity2;

struct App {
    model: model::Model,
}

impl screen::ScreenCallbacks for App {
    fn render(&self, mut screen_render: screen::ScreenRender) {
        screen_render.draw_model(&self.model);
    }
}

fn main() -> Result<()> {
    let clear_color = color::Color::new(0.1, 0.2, 0.3, 1.0);
    let mut runner = screen::ScreenRunner::create(800.0, 600.0, "Circle", clear_color)?;

    let translation = Similarity2::identity();

    let num_vertices = 128;
    let shape = runner.screen.create_circle(0.5, num_vertices, "Circle");
    let yellow = color::Color::new(1.0, 1.0, 0.0, 1.0);
    let model = model::Model::new(shape, yellow, translation);

    runner.run(App { model });
}
