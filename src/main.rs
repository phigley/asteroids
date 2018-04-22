extern crate graphics;
extern crate nalgebra;
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
use specs::{DispatcherBuilder, World};

use nalgebra::Point2;

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
    let renderer = Renderer::create()?;

    let mut rng = rand::thread_rng();

    let mut world = World::new();
    world.register::<Renderable>();
    world.register::<Physical>();
    world.register::<Player>();
    world.register::<Shape>();
    world.add_resource(Input::new());

    world
        .create_entity()
        .with(Player::new())
        .with(Shape::create_ship())
        .with(Physical::new(Point2::new(0.0, 0.0)))
        .with(Renderable::new(Color::new(1.0, 1.0, 1.0, 1.0)))
        .build();

    world
        .create_entity()
        .with(Shape::create_asteroid(&mut rng))
        .with(Physical::new(Point2::new(0.5, 0.5)))
        .with(Renderable::new(Color::new(1.0, 1.0, 1.0, 1.0)))
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .add(PlayerController, "player", &[])
        .add(Physics::new(), "physics", &["player"])
        .add_thread_local(renderer)
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
