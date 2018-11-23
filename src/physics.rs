use crate::na::{Isometry2, Point2, UnitComplex, Vector2};
use crate::ncollide2d::shape::{Polyline, ShapeHandle};
use crate::nphysics2d::{
    algebra::Inertia2,
    object::{BodyHandle, Material},
    world::World,
};
use specs::{
    Component, Entities, HashMapStorage, Join, LazyUpdate, Read, ReadStorage, System, VecStorage,
    Write, WriteStorage,
};

use input::Input;
use shape::Shape;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct AddCollision {
    pos: Isometry2<f32>,
    vel: Vector2<f32>,
}

impl AddCollision {
    pub fn new(pos: Isometry2<f32>, vel: Vector2<f32>) -> Self {
        AddCollision { pos, vel }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Physical {
    pos: Isometry2<f32>,
    vel: Vector2<f32>,

    render_pos: Isometry2<f32>,

    pulse_accel: Vector2<f32>,
    pulse_rot: f32,

    body_handle: BodyHandle,
}

impl Physical {
    fn new(pos: Isometry2<f32>, vel: Vector2<f32>, body_handle: BodyHandle) -> Self {
        Physical {
            pos,
            vel,

            render_pos: pos,

            pulse_accel: Vector2::new(0.0, 0.0),
            pulse_rot: 0.0,

            body_handle,
        }
    }

    pub fn render_position(&self) -> Isometry2<f32> {
        self.render_pos
    }

    pub fn add_relative_pulse(&mut self, accel: Vector2<f32>) {
        self.pulse_accel += self.pos.rotation * accel;
    }

    pub fn add_angular_pulse(&mut self, angle: f32) {
        self.pulse_rot += angle;
    }

    fn apply_dynamics(&mut self, world: &mut World<f32>, frame_time: f32, max_speed: f32) {
        // Clamp velocity.
        let accel = self.pulse_accel.norm();
        if accel > 0.0 {
            self.vel += self.pulse_accel * frame_time;
            let initial_speed = self.vel.norm();

            if initial_speed > max_speed {
                self.vel *= max_speed / initial_speed;
            }

            // Apply velocity.
            if let Some(ref mut rigid_body) = world.rigid_body_mut(self.body_handle) {
                rigid_body.set_linear_velocity(self.vel);
            }
        }

        if self.pulse_rot != 0.0 {
            if let Some(ref mut rigid_body) = world.rigid_body_mut(self.body_handle) {
                let mut position = rigid_body.position();

                let delta_angle = UnitComplex::new(-self.pulse_rot * frame_time);
                position.rotation *= delta_angle;

                rigid_body.set_position(position);
            }
        }
    }

    fn apply_step(&mut self, world: &World<f32>, extra_frame_time: f32) {
        if let Some(ref rigid_body) = world.rigid_body(self.body_handle) {
            self.pos = rigid_body.position();
            self.vel = rigid_body.velocity().linear;

            self.render_pos = self.pos;
            self.render_pos.translation.vector += self.vel * extra_frame_time;
        }
        self.pulse_accel = Vector2::new(0.0, 0.0);
        self.pulse_rot = 0.0;
    }

    fn apply_wraparound(&mut self, world: &mut World<f32>, max_x: f32, max_y: f32) {
        let mut modified = false;

        while self.pos.translation.vector.x >= max_x {
            self.pos.translation.vector.x -= 2.0 * max_x;
            modified = true;
        }

        while self.pos.translation.vector.x < -max_x {
            self.pos.translation.vector.x += 2.0 * max_x;
            modified = true;
        }

        while self.pos.translation.vector.y >= max_y {
            self.pos.translation.vector.y -= 2.0 * max_y;
            modified = true;
        }

        while self.pos.translation.vector.y < -max_y {
            self.pos.translation.vector.y += 2.0 * max_y;
            modified = true;
        }

        if modified {
            if let Some(ref mut rigid_body) = world.rigid_body_mut(self.body_handle) {
                rigid_body.set_position(self.pos);
            }
        }
    }
}

pub struct PhysicsWorld {
    world: World<f32>,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        PhysicsWorld {
            world: World::new(),
        }
    }
}

pub struct CollisionCreator;

impl<'a> System<'a> for CollisionCreator {
    //#[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'a, LazyUpdate>,
        Write<'a, PhysicsWorld>,
        Entities<'a>,
        ReadStorage<'a, AddCollision>,
        ReadStorage<'a, Shape>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (lazy, mut physics_world, entities, add_collisions, shapes) = data;

        for (e, add_collision, shape) in (&entities, &add_collisions, &shapes).join() {
            let center_of_mass: Point2<f32> = {
                let mut result: Vector2<f32> =
                    shape
                        .verts
                        .iter()
                        .fold(Vector2::new(0.0, 0.0), |mut acc, v| {
                            acc.x += v.x;
                            acc.y += v.y;
                            acc
                        });
                result /= shape.verts.len() as f32;
                Point2::from(result)
            };

            let inertia = Inertia2::new(1.0, 1.0);

            let shape_handle = ShapeHandle::new(Polyline::new(shape.verts.clone()));

            let body_handle =
                physics_world
                    .world
                    .add_rigid_body(add_collision.pos, inertia, center_of_mass);

            physics_world.world.add_collider(
                0.002,
                shape_handle,
                body_handle,
                Isometry2::identity(),
                Material::default(),
            );

            if let Some(ref mut rigid_body) = physics_world.world.rigid_body_mut(body_handle) {
                rigid_body.set_linear_velocity(add_collision.vel);
            }

            lazy.remove::<AddCollision>(e);
            lazy.insert(
                e,
                Physical::new(add_collision.pos, add_collision.vel, body_handle),
            );
        }
    }
}

pub struct Physics {
    max_x: f32,
    max_y: f32,
    max_speed: f32,
    extra_frame_time: f32,
}

impl Physics {
    pub fn new((max_x, max_y): (f32, f32)) -> Self {
        let max_speed = 1.0;

        Physics {
            max_x,
            max_y,
            max_speed,
            extra_frame_time: 0.0,
        }
    }

    fn calc_step_time(&mut self, frame_time: f32) -> (i32, f32) {
        let step_time = frame_time + self.extra_frame_time;

        let step_frames = step_time * 60.0;

        // If we are going to execute too many frames, just clamp it.
        if step_frames > 6.0 {
            self.extra_frame_time = 0.0;
            (6, 0.1)
        } else {
            let full_frames = f32::floor(step_frames);
            self.extra_frame_time = (step_frames - full_frames) / 60.0;
            (full_frames as i32, full_frames / 60.0)
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

        let (physics_steps, physics_frame_time) = self.calc_step_time(input.frame_time);

        for physical in (&mut physical).join() {
            physical.apply_dynamics(&mut physics_world.world, physics_frame_time, self.max_speed);
            physical.apply_wraparound(&mut physics_world.world, self.max_x, self.max_y);
        }

        for _ in 0..physics_steps {
            physics_world.world.step();
        }

        for physical in (&mut physical).join() {
            physical.apply_step(&physics_world.world, self.extra_frame_time);
            physical.apply_wraparound(&mut physics_world.world, self.max_x, self.max_y);
        }
    }
}
