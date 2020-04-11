use crate::na;
use crate::na::{Isometry2, Similarity2, Translation2, Vector2};
use crate::physics::Physical;
use crate::shape::Shape;
use graphics::color::Color;
use graphics::screen::{Screen, ScreenRender};
use graphics::shape::Shape as ScreenShape;
use specs::{Component, Join, ReadStorage, VecStorage, WriteStorage};

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
    max_x: f32,
    max_y: f32,
}

impl Renderer {
    pub fn new(width: f64, height: f64) -> Self {
        let aspect_ratio = (width / height) as f32;
        let (max_x, max_y) = if aspect_ratio > 1.0 {
            (aspect_ratio, 1.0)
        } else {
            (1.0, 1.0 / aspect_ratio)
        };

        Self { max_x, max_y }
    }

    pub fn get_max_coords(&self) -> (f32, f32) {
        (self.max_x, self.max_y)
    }

    pub fn update(
        &self,
        screen: &mut Screen,
        data: (ReadStorage<Shape>, WriteStorage<Renderable>),
    ) {
        let (shapes, mut renderables) = data;

        for (shape, renderable) in (&shapes, &mut renderables).join() {
            match renderable.screen_shape {
                Some(_) => (),
                ref mut s => {
                    // s is None, so create the shape
                    let new_s =
                        screen.create_shape(&shape.verts, &shape.indices, "Renderable Shape");
                    *s = Some(new_s);
                }
            };
        }
    }

    pub fn render(
        &self,
        mut screen_render: ScreenRender,
        data: (
            ReadStorage<Shape>,
            WriteStorage<Renderable>,
            ReadStorage<Physical>,
        ),
    ) {
        let (shapes, mut renderables, physicals) = data;

        for (shape, renderable, physical) in (&shapes, &mut renderables, &physicals).join() {
            if let Some(ref s) = renderable.screen_shape {
                let render_transform = RenderTransform::new(
                    physical.render_position(),
                    self.max_x,
                    self.max_y,
                    shape.radius,
                );

                render_transform.draw_shape(&mut screen_render, renderable.color, s);
            }
        }
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

    fn draw_shape(&self, screen_render: &mut ScreenRender, color: Color, shape: &ScreenShape) {
        for optional_transform in &self.transforms {
            if let Some(ref transform) = *optional_transform {
                screen_render.draw_shape(transform, color, shape);
            }
        }
    }
}
