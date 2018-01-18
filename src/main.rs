extern crate cgmath;
extern crate graphics;
extern crate specs;
extern crate time;

#[macro_use]
extern crate specs_derive;

#[macro_use]
extern crate quick_error;

mod renderer;
mod physics;
mod player;
mod input;

use graphics::color::Color;
use graphics::errors::ScreenCreateError;
use specs::{DispatcherBuilder, World};

use cgmath::Point2;

use renderer::{Renderable, Renderer, Shape};
use physics::{Physical, Physics};
use player::{Player, PlayerController};
use input::Input;

fn main() {
    if let Err(error) = run() {
        println!("{}", error);
    }
}

fn run() -> Result<(), AppError> {
    let renderer = Renderer::create()?;

    let mut world = World::new();
    world.register::<Renderable>();
    world.register::<Physical>();
    world.register::<Player>();
    world.add_resource(Input::new());

    world
        .create_entity()
        .with(Player::new())
        .with(Physical::new(Point2::new(0.0, 0.0)))
        .with(Renderable::new(
            Shape::Ship,
            0.025,
            Color::new(1.0, 1.0, 1.0, 1.0),
        ))
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .add(PlayerController, "player", &[])
        .add(Physics::new(), "physics", &["player"])
        .add_thread_local(renderer)
        .build();

    while !world.read_resource::<Input>().should_exit {
        dispatcher.dispatch(&mut world.res);
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
