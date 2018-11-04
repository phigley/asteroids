extern crate graphics;
extern crate nalgebra;
extern crate specs;

#[macro_use]
extern crate specs_derive;

use graphics::color::Color;
use graphics::events::{Event, Key};
use graphics::screen::Screen;
use graphics::FrameTimer;

use nalgebra::{Point2, Similarity2, Translation2, UnitComplex, Vector2};

use specs::{
    Builder, Component, DispatcherBuilder, Join, Read, System, VecStorage, World, WriteStorage,
};

use std::f32;

fn main() {
    let mut screen = match Screen::create(800.0, 600.0, "Bouncing Balls") {
        Err(create_error) => panic!("{:?}", create_error),
        Ok(created_screen) => created_screen,
    };

    let ball_shape = screen.create_circle(0.02, 64);
    let ball_color = Color::new(1.0, 1.0, 1.0, 1.0);

    let clear_color = Color::new(0.2, 0.2, 0.5, 1.0);

    let mouse_averge_factor = 0.05f32;
    let mouse_average_delay = 0.5f32;

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<BallRenderable>();

    world.add_resource(FrameTime(0.0));

    let mut frame_timer = FrameTimer::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(ApplyPhysics::new(), "apply_physics", &[])
        .build();

    let mut pending_ball: Option<PendingBall> = None;

    let mut current_pos = Point2::new(0.0, 0.0);

    let mut should_exit = false;
    while !should_exit {
        screen.clear(clear_color);

        let frame_delta = frame_timer.update(10, 0.1);

        *world.write_resource::<FrameTime>() = FrameTime(frame_delta);

        screen.poll_events(|event| match event {
            Event::Exit => should_exit = true,

            Event::KeyPress {
                key: Key::R,
                down: true,
            } => {
                world.delete_all();
            }

            Event::MouseLMB { down } => {
                if down {
                    pending_ball = Some(PendingBall::new(current_pos));
                } else {
                    if let Some(ref pending_ball) = pending_ball {
                        world
                            .create_entity()
                            .with(Position(pending_ball.pos))
                            .with(Velocity(pending_ball.vel))
                            .with(BallRenderable(ball_color))
                            .build();
                    }

                    pending_ball = None;
                }
            }

            Event::MouseMove { pos } => {
                if let Some(ref mut pending_ball) = pending_ball {
                    if frame_delta > 0.0 {
                        let delta_mouse_pos = pos - current_pos;
                        let mouse_vel = delta_mouse_pos / frame_delta;

                        let frame_decay =
                            mouse_averge_factor.powf(frame_delta / mouse_average_delay);

                        pending_ball.vel *= frame_decay;
                        pending_ball.vel += mouse_vel * (1.0f32 - frame_decay);

                        pending_ball.pos = current_pos;
                    }
                }

                current_pos = pos;
            }
            _ => (),
        });

        dispatcher.dispatch(&world.res);

        {
            let ball_renderables = world.read_storage::<BallRenderable>();
            let positions = world.read_storage::<Position>();

            for (&BallRenderable(ref color), &Position(ref pos)) in
                (&ball_renderables, &positions).join()
            {
                let transform = Similarity2::from_parts(
                    Translation2::from(pos.coords),
                    UnitComplex::identity(),
                    1.0,
                );
                screen.draw_shape(&transform, *color, &ball_shape);
            }
        }

        if let Some(ref pending_ball) = pending_ball {
            let transform = Similarity2::from_parts(
                Translation2::from(pending_ball.pos.coords),
                UnitComplex::identity(),
                1.0,
            );
            screen.draw_shape(&transform, ball_color, &ball_shape);
        }

        screen.flush();
    }
}

#[derive(Debug, Default)]
struct FrameTime(f32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position(Point2<f32>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity(Vector2<f32>);

#[derive(Debug)]
struct ApplyPhysics {
    acceleration: Vector2<f32>,
    restitution: f32,
    max_velocity: f32,
}

impl ApplyPhysics {
    fn new() -> ApplyPhysics {
        let acceleration = Vector2::new(0.0, -0.98);
        let restitution = 0.99f32;

        let max_velocity = 3.0f32;

        ApplyPhysics {
            acceleration,
            restitution,
            max_velocity,
        }
    }
}

impl<'a> System<'a> for ApplyPhysics {
    type SystemData = (
        Read<'a, FrameTime>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (frame_time, mut pos, mut vel) = data;

        let frame_delta = frame_time.0;

        for (&mut Position(ref mut pos), &mut Velocity(ref mut vel)) in (&mut pos, &mut vel).join()
        {
            *vel += self.acceleration * frame_delta;

            let speed: f32 = vel.norm();
            if speed > self.max_velocity {
                *vel *= self.max_velocity / speed;
            }

            *pos += *vel * frame_delta;

            if pos.y < -1.0f32 {
                pos.y = -1.0 - (pos.y % 1.0);
                vel.y = -vel.y;

                *vel *= self.restitution;
            } else if pos.y > 1.0f32 {
                pos.y = 1.0 - (pos.y % 1.0);
                vel.y = -vel.y;

                *vel *= self.restitution;
            }

            if pos.x < -1.0f32 {
                pos.x = -1.0 - (pos.x % 1.0);
                vel.x = -vel.x;

                *vel *= self.restitution;
            } else if pos.x > 1.0f32 {
                pos.x = 1.0 - (pos.x % 1.0);
                vel.x = -vel.x;

                *vel *= self.restitution;
            }
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct BallRenderable(Color);

#[derive(Clone)]
struct PendingBall {
    pos: Point2<f32>,
    vel: Vector2<f32>,
}

impl PendingBall {
    fn new(pos: Point2<f32>) -> PendingBall {
        PendingBall {
            pos,
            vel: Vector2::new(0.0, 0.0),
        }
    }
}
