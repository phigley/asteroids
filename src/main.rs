#[macro_use]
extern crate specs_derive;

mod input;
mod physics;
mod player;
mod renderer;
mod shape;

use crate::input::Input;
use crate::physics::{AddCollision, CollisionCreator, Physics};
use crate::player::{Player, PlayerController};
use crate::renderer::{Renderable, Renderer};
use crate::shape::Shape;
use anyhow::Result;
use graphics::{
    color::Color,
    events::{Event, Key},
    screen::{Screen, ScreenCallbacks, ScreenRender, ScreenRunner},
};
use nalgebra as na;
use nalgebra::{Isometry2, Vector2};
use specs::{Builder, Dispatcher, DispatcherBuilder, World, WorldExt};
use std::time::Duration;

fn main() -> Result<()> {
    let width = 800.0;
    let height = 600.0;
    let clear_color = Color::new(0.2, 0.2, 0.5, 1.0);
    let runner = ScreenRunner::create(width, height, "Bouncing Balls", clear_color)?;

    let app = App::new(width, height);

    runner.run(app);
}

struct App<'a, 'b> {
    world: World,
    dispatcher: Box<Dispatcher<'a, 'b>>,
    renderer: Renderer,
}

impl App<'_, '_> {
    fn new(width: f64, height: f64) -> Self {
        let renderer = Renderer::new(width, height);

        let mut rng = rand::thread_rng();

        let mut world = World::new();
        // Renderable is not used in a dispatched system, so
        // we must explicitly register it.
        world.register::<Renderable>();
        let mut dispatcher = Box::new(
            DispatcherBuilder::new()
                .with(PlayerController, "player", &[])
                .with(CollisionCreator, "collision_creator", &[])
                .with(
                    Physics::new(renderer.get_max_coords()),
                    "physics",
                    &["player", "collision_creator"],
                )
                .build(),
        );
        dispatcher.setup(&mut world);

        let player_pos = Isometry2::new(Vector2::new(0.0, 0.0), na::zero());
        let player_shape = Shape::create_ship();
        let player_physical = AddCollision::new(player_pos, Vector2::new(0.25, 0.5));
        world
            .create_entity()
            .with(Player::new())
            .with(player_shape)
            .with(player_physical)
            .with(Renderable::new(Color::new(1.0, 1.0, 1.0, 1.0)))
            .build();

        let astroid_shape = Shape::create_asteroid(&mut rng);
        let asteroid_pos = Isometry2::new(Vector2::new(0.5, 0.5), na::zero());
        let astroid_physical = AddCollision::new(asteroid_pos, Vector2::new(0.25, 0.5));
        world
            .create_entity()
            .with(astroid_shape)
            .with(astroid_physical)
            .with(Renderable::new(Color::new(1.0, 1.0, 1.0, 1.0)))
            .build();

        let astroid_shape2 = Shape::create_asteroid(&mut rng);
        let asteroid_pos2 = Isometry2::new(Vector2::new(-0.5, 0.5), na::zero());
        let astroid_physical2 = AddCollision::new(asteroid_pos2, Vector2::new(-0.25, 0.5));
        world
            .create_entity()
            .with(astroid_shape2)
            .with(astroid_physical2)
            .with(Renderable::new(Color::new(1.0, 1.0, 1.0, 1.0)))
            .build();

        Self {
            world,
            dispatcher,
            renderer,
        }
    }
}

impl ScreenCallbacks for App<'_, '_> {
    fn handle_event(&mut self, _screen: &mut Screen, event: Event) {
        let mut input = self.world.write_resource::<Input>();
        match event {
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
        }
    }
    fn update(&mut self, screen: &mut Screen, frame_delta: Duration) {
        {
            let mut input = self.world.write_resource::<Input>();
            input.frame_time = frame_delta.as_secs_f32();
        }
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();

        self.renderer.update(screen, self.world.system_data());
    }

    fn render(&self, screen_render: ScreenRender) {
        self.renderer
            .render(screen_render, self.world.system_data())
    }
}
