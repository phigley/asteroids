use graphics::screen::Screen;
use graphics::shape::Shape as ScreenShape;
use graphics::events::{Event, Key};
use graphics::color::Color;
use graphics::errors::ScreenCreateError;

use cgmath::{Matrix4, Point2};
use cgmath::prelude::*;

use specs::{FetchMut, Join, ReadStorage, System, VecStorage};

use physics::Physical;

pub struct RendererControl {
    pub should_exit: bool,
}

impl RendererControl {
    pub fn new() -> Self {
        RendererControl { should_exit: false }
    }
}

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

        Ok(Renderer {
            screen,
            clear_color,
            ship_shape,
        })
    }
}

impl<'a> System<'a> for Renderer {
    type SystemData = (
        FetchMut<'a, RendererControl>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Physical>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut control, renderables, physicals) = data;

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

        self.screen.poll_events(|event| match event {
            Event::Exit => control.should_exit = true,
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
