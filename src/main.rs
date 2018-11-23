extern crate graphics;
extern crate nalgebra as na;
extern crate ncollide2d;
extern crate nphysics2d;
extern crate rand;
extern crate specs;

#[macro_use]
extern crate specs_derive;

#[macro_use]
extern crate quick_error;

mod input;
mod physics;
mod player;
mod renderer;
mod shape;

use graphics::color::Color;
use graphics::errors::ScreenCreateError;
use specs::{Builder, DispatcherBuilder, World};

use na::{Isometry2, Vector2};

use input::Input;
use physics::{AddCollision, CollisionCreator, Physics};
use player::{Player, PlayerController};
use renderer::{Renderable, Renderer};
use shape::Shape;

fn main() {
    if let Err(error) = run() {
        println!("{}", error);
    }
}

fn run() -> Result<(), AppError> {
    let width = 800.0;
    let height = 600.0;

    let renderer = Renderer::create(width, height)?;

    let mut rng = rand::thread_rng();

    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerController, "player", &[])
        .with(CollisionCreator, "collision_creator", &[])
        .with(
            Physics::new(renderer.get_max_coords()),
            "physics",
            &["player", "collision_creator"],
        ).with_thread_local(renderer)
        .build();

    dispatcher.setup(&mut world.res);

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

    while !world.read_resource::<Input>().should_exit {
        dispatcher.dispatch(&world.res);
        world.maintain();
    }

    Ok(())
}

quick_error! {
    #[derive(Debug)]
    enum AppError {
        GraphicsError(err: ScreenCreateError) {
            from(err: ScreenCreateError) -> (err)
            display("Could not create graphics screen: {}", err)
        }
    }
}
