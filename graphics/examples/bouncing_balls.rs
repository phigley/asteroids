#[macro_use]
extern crate specs_derive;

use anyhow::Result;
use nalgebra::{Point2, Similarity2, Translation2, UnitComplex, Vector2};
use specs::{
    Builder, Component, Dispatcher, DispatcherBuilder, Join, Read, System, VecStorage, World,
    WorldExt, WriteStorage,
};
use std::f32;
use std::time::Duration;

use graphics::{
    color::Color,
    events::{Event, Key},
    screen::{Screen, ScreenCallbacks, ScreenRender, ScreenRunner},
    shape::Shape,
};

fn main() -> Result<()> {
    let clear_color = Color::new(0.2, 0.2, 0.5, 1.0);
    let mut runner = ScreenRunner::create(800.0, 600.0, "Bouncing Balls", clear_color)?;

    let app = App::new(&mut runner.screen);

    runner.run(app);
}

struct App<'a, 'b> {
    ball_shape: Shape,
    ball_color: Color,
    world: World,
    dispatcher: Box<Dispatcher<'a, 'b>>,
    pending_ball: Option<PendingBall>,

    current_pos: Point2<f32>,

    mouse_averge_factor: f32,
    mouse_average_delay: f32,
}

impl App<'_, '_> {
    fn new(screen: &mut Screen) -> Self {
        let dispatcher = Box::new(
            DispatcherBuilder::new()
                .with(ApplyPhysics::new(), "apply_physics", &[])
                .build(),
        );

        let ball_shape = screen.create_circle(0.02, 64, "Ball");
        let ball_color = Color::new(1.0, 1.0, 1.0, 1.0);
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<BallRenderable>();
        world.insert(FrameTime(0.0));

        Self {
            ball_shape,
            ball_color,

            world,

            dispatcher,
            pending_ball: None,
            current_pos: Point2::new(0.0, 0.0),

            mouse_averge_factor: 0.05f32,
            mouse_average_delay: 0.5f32,
        }
    }
}

impl ScreenCallbacks for App<'_, '_> {
    fn handle_event(&mut self, _screen: &mut Screen, event: Event) {
        match event {
            Event::KeyPress {
                key: Key::R,
                down: true,
            } => {
                self.world.delete_all();
            }
            Event::MouseLMB { down } => {
                if down {
                    self.pending_ball = Some(PendingBall::new(self.current_pos));
                } else {
                    if let Some(ref pending_ball) = self.pending_ball {
                        self.world
                            .create_entity()
                            .with(Position(pending_ball.pos))
                            .with(Velocity(pending_ball.vel))
                            .with(BallRenderable(self.ball_color))
                            .build();
                    }

                    self.pending_ball = None;
                }
            }

            Event::MouseMove { pos } => {
                if let Some(ref mut pending_ball) = self.pending_ball {
                    let system_data: Read<FrameTime> = self.world.system_data();
                    let frame_delta = system_data.0;
                    if frame_delta > 0.0 {
                        let delta_mouse_pos = pos - self.current_pos;
                        let mouse_vel = delta_mouse_pos / frame_delta;

                        let frame_decay = self
                            .mouse_averge_factor
                            .powf(frame_delta / self.mouse_average_delay);

                        pending_ball.vel *= frame_decay;
                        pending_ball.vel += mouse_vel * (1.0f32 - frame_decay);
                        pending_ball.pos = self.current_pos;
                    }
                }

                self.current_pos = pos;
            }
            _ => (),
        }
    }

    fn update(&mut self, _screen: &mut Screen, frame_delta: Duration) {
        *self.world.write_resource::<FrameTime>() = FrameTime(frame_delta.as_secs_f32());
        self.dispatcher.dispatch(&self.world);
    }

    fn render(&self, mut screen_render: ScreenRender) {
        {
            let ball_renderables = self.world.read_storage::<BallRenderable>();
            let positions = self.world.read_storage::<Position>();

            for (&BallRenderable(ref color), &Position(ref pos)) in
                (&ball_renderables, &positions).join()
            {
                let transform = Similarity2::from_parts(
                    Translation2::from(pos.coords),
                    UnitComplex::identity(),
                    1.0,
                );
                screen_render.draw_shape(&transform, *color, &self.ball_shape);
            }
        }

        if let Some(ref pending_ball) = self.pending_ball {
            let transform = Similarity2::from_parts(
                Translation2::from(pending_ball.pos.coords),
                UnitComplex::identity(),
                1.0,
            );
            screen_render.draw_shape(&transform, self.ball_color, &self.ball_shape);
        }
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
