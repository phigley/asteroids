use na;
use na::{Isometry2, Unit, Vector2};
use ncollide2d::events::ContactEvent;
use ncollide2d::shape::{Polyline, ShapeHandle};
use ncollide2d::world::{
    CollisionGroups, CollisionObjectHandle, CollisionWorld, GeometricQueryType,
};
use specs::{
    Component, Entities, Entity, Join, LazyUpdate, NullStorage, Read, ReadStorage, System,
    VecStorage, Write, WriteStorage,
};

use input::Input;
use shape::Shape;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct AddCollision;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Physical {
    pub pos: Isometry2<f32>,
    pub vel: Vector2<f32>,

    collision_handle: Option<CollisionObjectHandle>,
}

impl Physical {
    pub fn new(pos: Isometry2<f32>, vel: Vector2<f32>) -> Self {
        Physical {
            pos,
            vel,
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

pub struct PhysicsWorld {
    collision_world: CollisionWorld<f32, Entity>,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        PhysicsWorld {
            collision_world: CollisionWorld::new(0.02),
        }
    }
}

pub struct CollisionCreator;

impl<'a> System<'a> for CollisionCreator {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'a, LazyUpdate>,
        Write<'a, PhysicsWorld>,
        Entities<'a>,
        ReadStorage<'a, AddCollision>,
        ReadStorage<'a, Shape>,
        WriteStorage<'a, Physical>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (lazy, mut physics_world, entities, add_collisions, shapes, mut physical) = data;

        for (e, _, shape, physical) in (&entities, &add_collisions, &shapes, &mut physical).join() {
            let shape_handle = ShapeHandle::new(Polyline::new(shape.verts.clone()));

            let collision_handle = physics_world.collision_world.add(
                physical.pos,
                shape_handle,
                CollisionGroups::new(),
                GeometricQueryType::Contacts(0.0, 0.09),
                e,
            );

            physical.collision_handle = Some(collision_handle);

            lazy.remove::<AddCollision>(e);
        }
    }
}

pub struct Physics {
    max_x: f32,
    max_y: f32,
    max_speed: f32,
}

impl Physics {
    pub fn new((max_x, max_y): (f32, f32)) -> Self {
        let max_speed = 1.0;

        Physics {
            max_x,
            max_y,
            max_speed,
        }
    }
}

impl<'a> System<'a> for Physics {
    type SystemData = (
        Read<'a, Input>,
        Write<'a, PhysicsWorld>,
        WriteStorage<'a, Physical>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, mut physics_world, mut physical) = data;

        let frame_time = input.frame_time;

        physics_world.collision_world.update();

        for event in physics_world.collision_world.contact_events() {
            if let ContactEvent::Started(c0, c1) = *event {
                if let Some(pair) = physics_world.collision_world.contact_pair(c0, c1) {
                    let mut collector = Vec::new();
                    pair.contacts(&mut collector);

                    let mut accumulator = na::zero();
                    for contact in collector {
                        if let Some(deepest) = contact.deepest_contact() {
                            let contact = &deepest.contact;
                            accumulator += *contact.normal * contact.depth;
                        }
                    }

                    if let Some(normal) = Unit::try_new(accumulator, 1e-6) {
                        if let Some(co0) = physics_world.collision_world.collision_object(c0) {
                            if let Some(ref mut physical0) = physical.get_mut(*co0.data()) {
                                physical0.vel -= 2.0 * na::dot(&physical0.vel, &normal) * *normal;
                            }
                        }

                        if let Some(co1) = physics_world.collision_world.collision_object(c1) {
                            let flipped_normal = -normal;

                            if let Some(ref mut physical1) = physical.get_mut(*co1.data()) {
                                physical1.vel -= 2.0
                                    * na::dot(&physical1.vel, &flipped_normal)
                                    * *flipped_normal;
                            }
                        }
                    }

                    // let normal = collector[0].deepest_contact().unwrap().contact.normal;
                }
            }
        }

        for physical in (&mut physical).join() {
            physical.apply_dynamics(frame_time, self.max_speed);
            physical.apply_wraparound(self.max_x, self.max_y);

            if let Some(collision_handle) = physical.collision_handle {
                physics_world
                    .collision_world
                    .set_position(collision_handle, physical.pos);
            }
        }
    }
}
