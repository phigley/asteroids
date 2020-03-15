use graphics::{color, model, screen};

use nalgebra::{Point2, Similarity2};

struct App {
    model: model::Model,
}

impl App {
    fn new(runner: &mut screen::ScreenRunner) -> Self {
        let translation: Similarity2<f32> = Similarity2::identity();

        let indices = [0, 1, 2];
        let verts = [
            Point2::new(-0.5, 0.5),
            Point2::new(0.5, 0.5),
            Point2::new(0.0, -0.5),
        ];
        let shape = runner.screen.create_shape(&verts, &indices, "triangle");
        let yellow = color::Color::new(1.0, 1.0, 0.0, 1.0);
        Self {
            model: model::Model::new(shape, yellow, translation),
        }
    }
}

impl screen::ScreenCallbacks for App {
    fn render(&self, mut screen_render: screen::ScreenRender) {
        screen_render.draw_model(&self.model);
    }
}

fn main() {
    let clear_color = color::Color::new(0.1, 0.2, 0.3, 1.0);

    let mut runner = match screen::ScreenRunner::create(800.0, 600.0, "Triangle", clear_color) {
        Err(create_error) => panic!(create_error.to_string()),
        Ok(created_screen) => created_screen,
    };

    let app = App::new(&mut runner);
    runner.run(app);
}
