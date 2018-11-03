extern crate graphics;
extern crate nalgebra as na;
extern crate ncollide2d;
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
use physics::{Physical, Physics};
use player::{Player, PlayerController};
use renderer::{Renderable, Renderer};
use shape::Shape;

fn main() {
    if let Err(error) = run() {
        println!("{}", error);
    }
}

fn run() -> Result<(), AppError> {
    let renderer = Renderer::create(800.0, 600.0)?;

    let mut rng = rand::thread_rng();

    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PlayerController, "player", &[])
        .with(Physics::new(), "physics", &["player"])
        .with_thread_local(renderer)
        .build();

    dispatcher.setup(&mut world.res);

    let player_pos = Isometry2::new(Vector2::new(0.0, 0.0), na::zero());

    let player_shape = Shape::create_ship();
    let player_physical = Physical::new(player_pos, player_shape.verts.clone());

    world
        .create_entity()
        .with(Player::new())
        .with(player_shape)
        .with(player_physical)
        .with(Renderable::new(Color::new(1.0, 1.0, 1.0, 1.0)))
        .build();

    let asteroid_pos = Isometry2::new(Vector2::new(0.5, 0.5), na::zero());
    let astroid_shape = Shape::create_asteroid(&mut rng);
    let astroid_physical = Physical::new(asteroid_pos, astroid_shape.verts.clone());

    world
        .create_entity()
        .with(astroid_shape)
        .with(astroid_physical)
        .with(Renderable::new(Color::new(1.0, 1.0, 1.0, 1.0)))
        .build();

    while !world.read_resource::<Input>().should_exit {
        dispatcher.dispatch(&world.res);
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
