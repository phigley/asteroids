use graphics::color::Color;
use graphics::errors::ScreenCreateError;
use graphics::events::{Event, Key};
use graphics::screen::Screen;
use graphics::shape::Shape as ScreenShape;
use graphics::FrameTimer;
use crate::na;

use crate::na::{Isometry2, Similarity2, Translation2, Vector2};

use specs::{Component, Join, ReadStorage, System, VecStorage, Write, WriteStorage};

use crate::input::Input;
use crate::physics::Physical;
use crate::shape::Shape;

#[derive(Component, Debug)]
#[storage(VecStorage)]
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

    max_x: f32,
    max_y: f32,

    frame_timer: FrameTimer,
}

impl Renderer {
    pub fn create(width: f64, height: f64) -> Result<Self, ScreenCreateError> {
        let screen = Screen::create(width, height, "Asteroids")?;
        let clear_color = Color::new(0.2, 0.2, 0.5, 1.0);

        let aspect_ratio = (width / height) as f32;
        let (max_x, max_y) = if aspect_ratio > 1.0 {
            (aspect_ratio, 1.0)
        } else {
            (1.0, 1.0 / aspect_ratio)
        };

        Ok(Renderer {
            screen,
            clear_color,

            max_x,
            max_y,

            frame_timer: FrameTimer::new(),
        })
    }

    pub fn get_max_coords(&self) -> (f32, f32) {
        (self.max_x, self.max_y)
    }
}

impl<'a> System<'a> for Renderer {
    type SystemData = (
        Write<'a, Input>,
        ReadStorage<'a, Shape>,
        WriteStorage<'a, Renderable>,
        ReadStorage<'a, Physical>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input, shapes, mut renderables, physicals) = data;

        self.screen.clear(self.clear_color);

        for (shape, renderable, physical) in (&shapes, &mut renderables, &physicals).join() {
            let render_transform = RenderTransform::new(
                physical.render_position(),
                self.max_x,
                self.max_y,
                shape.radius,
            );

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

        input.frame_time = self.frame_timer.update(10, 0.1);

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
    transforms: [Option<Similarity2<f32>>; 4],
}

impl RenderTransform {
    fn new(transform: Isometry2<f32>, max_x: f32, max_y: f32, radius: f32) -> Self {
        let mut transforms = [None; 4];

        transforms[0] = Some(na::convert(transform));

        let copy_x = if transform.translation.vector.x + radius > max_x {
            Some(transform.translation.vector.x - 2.0 * max_x)
        } else if transform.translation.vector.x - radius < -max_x {
            Some(transform.translation.vector.x + 2.0 * max_x)
        } else {
            None
        };

        let copy_y = if transform.translation.vector.y + radius > max_y {
            Some(transform.translation.vector.y - 2.0 * max_y)
        } else if transform.translation.vector.y - radius < -max_y {
            Some(transform.translation.vector.y + 2.0 * max_y)
        } else {
            None
        };

        if let Some(adjusted_x) = copy_x {
            transforms[1] = Some(Similarity2::from_parts(
                Translation2::from(Vector2::new(adjusted_x, transform.translation.vector.y)),
                transform.rotation,
                1.0,
            ));

            // when x and y need to be adjusted, we will need 4 copies.
            if let Some(adjusted_y) = copy_y {
                transforms[2] = Some(Similarity2::from_parts(
                    Translation2::from(Vector2::new(adjusted_x, adjusted_y)),
                    transform.rotation,
                    1.0,
                ));
            }
        }

        if let Some(adjusted_y) = copy_y {
            transforms[3] = Some(Similarity2::from_parts(
                Translation2::from(Vector2::new(transform.translation.vector.x, adjusted_y)),
                transform.rotation,
                1.0,
            ));
        }

        RenderTransform { transforms }
    }

    fn draw_shape(&self, screen: &mut Screen, color: Color, shape: &ScreenShape) {
        for optional_transform in &self.transforms {
            if let Some(ref transform) = *optional_transform {
                screen.draw_shape(transform, color, shape);
            }
        }
    }
}
