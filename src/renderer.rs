use std::cmp;

use graphics::screen::Screen;
use graphics::shape::Shape as ScreenShape;
use graphics::events::{Event, Key};
use graphics::color::Color;
use graphics::errors::ScreenCreateError;

use cgmath::{Matrix4, Point2};

use specs::{FetchMut, Join, ReadStorage, System, VecStorage};

use time::PreciseTime;

use physics::Physical;
use input::{Action, Input};

#[derive(Debug)]
pub enum Shape {
    Ship,
}

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Renderable {
    shape: Shape,
    scale: f32,
    color: Color,
}

impl Renderable {
    pub fn new(shape: Shape, scale: f32, color: Color) -> Self {
        Renderable {
            shape,
            scale,
            color,
        }
    }
}

pub struct Renderer {
    screen: Screen,
    clear_color: Color,

    ship_shape: ScreenShape,

    previous_time: PreciseTime,
}

impl Renderer {
    pub fn create() -> Result<Self, ScreenCreateError> {
        let mut screen = Screen::create("Asteroids")?;
        let clear_color = Color::new(0.2, 0.2, 0.5, 1.0);

        let ship_verts = [
            Point2::new(0.0, 1.0),
            Point2::new(1.0, -1.0),
            Point2::new(0.0, -0.5),
            Point2::new(-1.0, -1.0),
        ];
        let ship_indices = [0, 1, 2, 0, 2, 3];
        let ship_shape = screen.create_shape(&ship_verts, &ship_indices);

        let previous_time = PreciseTime::now();

        Ok(Renderer {
            screen,
            clear_color,
            ship_shape,

            previous_time,
        })
    }
}

impl<'a> System<'a> for Renderer {
    type SystemData = (
        FetchMut<'a, Input>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Physical>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input, renderables, physicals) = data;

        self.screen.clear(self.clear_color);

        for (renderable, physical) in (&renderables, &physicals).join() {
            let transform = create_transform(physical.pos, renderable.scale);

            match renderable.shape {
                Shape::Ship => {
                    self.screen
                        .draw_shape(&transform, renderable.color, &self.ship_shape)
                }
            }
        }

        self.screen.flush();

        let current_time = PreciseTime::now();
        let frame_duration = self.previous_time.to(current_time);
        let frame_ms = cmp::min(frame_duration.num_milliseconds(), 100);
        input.frame_time = (frame_ms as f32) * 1e-3f32;
        self.previous_time = current_time;

        input.actions.clear();

        self.screen.poll_events(|event| match event {
            Event::Exit => input.should_exit = true,

            Event::KeyPress {
                key: Key::W,
                down: true,
            } => input.actions.push(Action::Forward),

            Event::KeyPress {
                key: Key::A,
                down: true,
            } => input.actions.push(Action::Left),

            Event::KeyPress {
                key: Key::D,
                down: true,
            } => input.actions.push(Action::Right),

            _ => (),
        });
    }
}

fn create_transform(position: Point2<f32>, scale: f32) -> Matrix4<f32> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    Matrix4::new( 
        scale, 0.0, 0.0, 0.0, 
        0.0, scale, 0.0, 0.0,
        0.0, 0.0, scale, 0.0,
        position.x, position.y, 0.0, 1.0
        )
}
