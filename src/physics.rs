use na;
use na::{Isometry2, Point2, Vector2};
use ncollide2d::events::ContactEvent;
use ncollide2d::shape::{Polyline, ShapeHandle};
use ncollide2d::world::{
    CollisionGroups, CollisionObjectHandle, CollisionWorld, GeometricQueryType,
};
use specs::{Entities, Entity, Join, Read, System, VecStorage, WriteStorage};

use input::Input;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Physical {
    pub pos: Isometry2<f32>,
    pub vel: Vector2<f32>,

    shape_handle: ShapeHandle<f32>,

    collision_handle: Option<CollisionObjectHandle>,
}

impl Physical {
    pub fn new(pos: Isometry2<f32>, vertices: Vec<Point2<f32>>) -> Self {
        Physical {
            pos,
            vel: Vector2::new(0.25, 0.1),

            shape_handle: ShapeHandle::new(Polyline::new(vertices)),

            collision_handle: None,
        }
    }

    fn apply_dynamics(&mut self, frame_time: f32, max_speed: f32) {
        // Clamp velocity.
        let initial_speed = self.vel.norm();

        if initial_speed > max_speed {
            self.vel *= max_speed / initial_speed;
        }

        // Apply velocity.
        self.pos.translation.vector += self.vel * frame_time;
    }

    fn apply_wraparound(&mut self, max_x: f32, max_y: f32) {
        while self.pos.translation.vector.x >= max_x {
            self.pos.translation.vector.x -= 2.0 * max_x;
        }

        while self.pos.translation.vector.x < -max_x {
            self.pos.translation.vector.x += 2.0 * max_x;
        }

        while self.pos.translation.vector.y >= max_y {
            self.pos.translation.vector.y -= 2.0 * max_y;
        }

        while self.pos.translation.vector.y < -max_y {
            self.pos.translation.vector.y += 2.0 * max_y;
        }
    }
}

pub struct Physics {
    aspect_ratio: f32,
    max_speed: f32,

    collision_world: CollisionWorld<f32, Entity>,
}

impl Physics {
    pub fn new() -> Self {
        let aspect_ratio = 8.0 / 6.0;
        let max_speed = 1.0;

        let collision_world = CollisionWorld::new(0.02);
        Physics {
            aspect_ratio,
            max_speed,
            collision_world,
        }
    }
}

impl<'a> System<'a> for Physics {
    type SystemData = (Read<'a, Input>, Entities<'a>, WriteStorage<'a, Physical>);

    fn run(&mut self, data: Self::SystemData) {
        let (input, entities, mut physical) = data;

        let frame_time = input.frame_time;

        let max_x = self.aspect_ratio;
        let max_y = 1.0;

        // Add new physical components.
        // Is there a better way to do this?
        for (e, physical) in (&entities, &mut physical).join() {
            if None == physical.collision_handle {
                physical.collision_handle = Some(self.collision_world.add(
                    physical.pos,
                    physical.shape_handle.clone(),
                    CollisionGroups::new(),
                    GeometricQueryType::Contacts(0.0, 0.09),
                    e,
                ));
            }
        }

        self.collision_world.update();

        for event in self.collision_world.contact_events() {
            if let ContactEvent::Started(c0, c1) = *event {
                if let Some(pair) = self.collision_world.contact_pair(c0, c1) {
                    let mut collector = Vec::new();
                    pair.contacts(&mut collector);

                    let normal = collector[0].deepest_contact().unwrap().contact.normal;

                    if let Some(co0) = self.collision_world.collision_object(c0) {
                        if let Some(ref mut physical0) = physical.get_mut(*co0.data()) {
                            physical0.vel -= 2.0 * na::dot(&physical0.vel, &normal) * *normal;
                        }
                    }

                    if let Some(co1) = self.collision_world.collision_object(c1) {
                        if let Some(ref mut physical1) = physical.get_mut(*co1.data()) {
                            physical1.vel -= 2.0 * na::dot(&physical1.vel, &normal) * *normal;
                        }
                    }
                }
            }
        }

        for physical in (&mut physical).join() {
            physical.apply_dynamics(frame_time, self.max_speed);
            physical.apply_wraparound(max_x, max_y);

            if let Some(collision_handle) = physical.collision_handle {
                self.collision_world
                    .set_position(collision_handle, physical.pos);
            }
        }
    }
}
