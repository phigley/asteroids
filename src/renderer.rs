use std::cmp;

use graphics::color::Color;
use graphics::errors::ScreenCreateError;
use graphics::events::{Event, Key};
use graphics::screen::Screen;
use graphics::shape::Shape as ScreenShape;

use cgmath::{Basis2, Matrix2, Matrix4, Point2};

use specs::{FetchMut, Join, ReadStorage, System, VecStorage, WriteStorage};

use time::PreciseTime;

use input::Input;
use physics::Physical;
use shape::Shape;

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Renderable {
    screen_shape: Option<ScreenShape>,
    color: Color,
}

impl Renderable {
    pub fn new(color: Color) -> Self {
        Renderable {
            screen_shape: None,
            color,
        }
    }
}

pub struct Renderer {
    screen: Screen,
    clear_color: Color,

    previous_time: PreciseTime,
}

impl Renderer {
    pub fn create() -> Result<Self, ScreenCreateError> {
        let screen = Screen::create("Asteroids")?;
        let clear_color = Color::new(0.2, 0.2, 0.5, 1.0);

        let previous_time = PreciseTime::now();

        Ok(Renderer {
            screen,
            clear_color,

            previous_time,
        })
    }
}

impl<'a> System<'a> for Renderer {
    type SystemData = (
        FetchMut<'a, Input>,
        ReadStorage<'a, Shape>,
        WriteStorage<'a, Renderable>,
        ReadStorage<'a, Physical>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input, shapes, mut renderables, physicals) = data;

        self.screen.clear(self.clear_color);

        for (shape, mut renderable, physical) in (&shapes, &mut renderables, &physicals).join() {
            let render_transform =
                RenderTransform::new(physical.pos, physical.orientation, shape.radius);

            match renderable.screen_shape {
                Some(ref s) => render_transform.draw_shape(&mut self.screen, renderable.color, s),
                ref mut s => {
                    // s is None, so create the shape and render it right away.
                    let new_s = self.screen.create_shape(&shape.verts, &shape.indices);
                    render_transform.draw_shape(&mut self.screen, renderable.color, &new_s);
                    *s = Some(new_s);
                }
            };
            ;
        }

        self.screen.flush();

        let current_time = PreciseTime::now();
        let frame_duration = self.previous_time.to(current_time);
        let frame_ms = cmp::min(frame_duration.num_milliseconds(), 100);
        input.frame_time = (frame_ms as f32) * 1e-3f32;
        self.previous_time = current_time;

        self.screen.poll_events(|event| match event {
            Event::Exit => input.should_exit = true,

            Event::KeyPress { key: Key::W, down } => input.actions.accel_forward = down,
            Event::KeyPress { key: Key::D, down } => input.actions.accel_right = down,
            Event::KeyPress { key: Key::A, down } => input.actions.accel_left = down,

            Event::KeyPress {
                key: Key::Right,
                down,
            } => input.actions.turn_right = down,
            Event::KeyPress {
                key: Key::Left,
                down,
            } => input.actions.turn_left = down,

            _ => (),
        });
    }
}

struct RenderTransform {
    transforms: [Option<Matrix4<f32>>; 4],
}

impl RenderTransform {
    fn new(position: Point2<f32>, orientation: Basis2<f32>, radius: f32) -> Self {
        let aspect_ratio = 8.0 / 6.0;

        let orientation_m2: Matrix2<f32> = orientation.into();

        let mut transforms = [None; 4];

        transforms[0] = Some(create_transform(&orientation_m2, position.x, position.y));

        let copy_x = if position.x + radius > aspect_ratio {
            Some(position.x - 2.0 * aspect_ratio)
        } else if position.x - radius < -aspect_ratio {
            Some(position.x + 2.0 * aspect_ratio)
        } else {
            None
        };

        let copy_y = if position.y + radius > 1.0 {
            Some(position.y - 2.0)
        } else if position.y - radius < -1.0 {
            Some(position.y + 2.0)
        } else {
            None
        };

        if let Some(adjusted_x) = copy_x {
            transforms[1] = Some(create_transform(&orientation_m2, adjusted_x, position.y));

            // when x and y need to be adjusted, we will need 4 copies.
            if let Some(adjusted_y) = copy_y {
                transforms[2] = Some(create_transform(&orientation_m2, adjusted_x, adjusted_y));
            }
        }

        if let Some(adjusted_y) = copy_y {
            transforms[3] = Some(create_transform(&orientation_m2, position.x, adjusted_y));
        }

        RenderTransform { transforms }
    }

    fn draw_shape(&self, screen: &mut Screen, color: Color, shape: &ScreenShape) {
        for optional_transform in &self.transforms {
            if let &Some(ref transform) = optional_transform {
                screen.draw_shape(transform, color, shape);
            }
        }
    }
}

fn create_transform(orientation: &Matrix2<f32>, x_position: f32, y_position: f32) -> Matrix4<f32> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    Matrix4::new( 
        orientation.x.x, orientation.x.y, 0.0, 0.0, 
        orientation.y.x, orientation.y.y, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        x_position, y_position, 0.0, 1.0
    )
}
