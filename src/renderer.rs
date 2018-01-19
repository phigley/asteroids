use std::cmp;

use graphics::screen::Screen;
use graphics::shape::Shape as ScreenShape;
use graphics::events::{Event, Key};
use graphics::color::Color;
use graphics::errors::ScreenCreateError;

use cgmath::{Basis2, Matrix2, Matrix4, Point2};

use specs::{FetchMut, Join, ReadStorage, System, VecStorage, WriteStorage};

use time::PreciseTime;

use physics::Physical;
use input::Input;
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
            let transform = create_transform(physical.pos, physical.orientation);

            match renderable.screen_shape {
                Some(ref s) => self.screen.draw_shape(&transform, renderable.color, s),
                ref mut s => {
                    // s is None, so create the shape and render it right away.
                    let new_s = self.screen.create_shape(&shape.verts, &shape.indices);
                    self.screen.draw_shape(&transform, renderable.color, &new_s);
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

fn create_transform(position: Point2<f32>, orientation: Basis2<f32>) -> Matrix4<f32> {
    let orientation_m2: Matrix2<f32> = orientation.into();

    #[cfg_attr(rustfmt, rustfmt_skip)]
    Matrix4::new( 
        orientation_m2.x.x, orientation_m2.x.y, 0.0, 0.0, 
        orientation_m2.y.x, orientation_m2.y.y, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        position.x, position.y, 0.0, 1.0
    )
}
