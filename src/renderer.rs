use graphics::color::Color;
use graphics::errors::ScreenCreateError;
use graphics::events::{Event, Key};
use graphics::screen::Screen;
use graphics::shape::Shape as ScreenShape;
use graphics::FrameTimer;

use nalgebra::{Point2, Similarity2, Translation2, UnitComplex};

use specs::{Join, ReadStorage, System, VecStorage, Write, WriteStorage};

use input::Input;
use physics::Physical;
use shape::Shape;

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

    frame_timer: FrameTimer,
}

impl Renderer {
    pub fn create() -> Result<Self, ScreenCreateError> {
        let screen = Screen::create("Asteroids")?;
        let clear_color = Color::new(0.2, 0.2, 0.5, 1.0);

        Ok(Renderer {
            screen,
            clear_color,

            frame_timer: FrameTimer::new(),
        })
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
    fn new(position: Point2<f32>, orientation: UnitComplex<f32>, radius: f32) -> Self {
        let aspect_ratio = 8.0 / 6.0;

        let mut transforms = [None; 4];

        transforms[0] = Some(create_transform(position, orientation));

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
            transforms[1] = Some(create_transform(
                Point2::new(adjusted_x, position.y),
                orientation,
            ));

            // when x and y need to be adjusted, we will need 4 copies.
            if let Some(adjusted_y) = copy_y {
                transforms[2] = Some(create_transform(
                    Point2::new(adjusted_x, adjusted_y),
                    orientation,
                ));
            }
        }

        if let Some(adjusted_y) = copy_y {
            transforms[3] = Some(create_transform(
                Point2::new(position.x, adjusted_y),
                orientation,
            ));
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

fn create_transform(position: Point2<f32>, orientation: UnitComplex<f32>) -> Similarity2<f32> {
    Similarity2::from_parts(Translation2::new(position.x, position.y), orientation, 1.0)
}
