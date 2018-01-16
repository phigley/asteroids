use specs::{Join, System, VecStorage, WriteStorage};
use cgmath::{Point2, Vector2};
use cgmath::prelude::*;

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Physical {
    pub pos: Point2<f32>,
    vel: Vector2<f32>,
}

impl Physical {
    pub fn new(pos: Point2<f32>) -> Self {
        Physical {
            pos,
            vel: Vector2::new(0.25, 0.1),
        }
    }
}

pub struct Physics {
    aspect_ratio: f32,
    max_speed: f32,
}

impl Physics {
    pub fn new() -> Self {
        let aspect_ratio = 8.0 / 6.0;
        let max_speed = 1.0;

        Physics {
            aspect_ratio,
            max_speed,
        }
    }
}

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Physical>);

    fn run(&mut self, data: Self::SystemData) {
        let mut physical = data;

        let frame_time = 0.0033;

        for ref mut physical in (&mut physical).join() {
            // Clamp velocity.
            let initial_speed = physical.vel.magnitude();

            if initial_speed > self.max_speed {
                physical.vel *= self.max_speed / initial_speed;
            }

            // Apply velocity.
            physical.pos += physical.vel * frame_time;

            // Perform wrap-around.
            let max_x = self.aspect_ratio;
            let max_y = 1.0;

            while physical.pos.x >= max_x {
                physical.pos.x -= 2.0 * max_x;
            }

            while physical.pos.x < -max_x {
                physical.pos.x += 2.0 * max_x;
            }

            while physical.pos.y >= max_y {
                physical.pos.y -= 2.0 * max_y;
            }

            while physical.pos.y < -max_y {
                physical.pos.y += 2.0 * max_y;
            }
        }
    }
}
