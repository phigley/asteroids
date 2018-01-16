extern crate cgmath;
extern crate graphics;
extern crate specs;

#[macro_use]
extern crate specs_derive;

#[macro_use]
extern crate quick_error;

mod renderer;

use graphics::color::Color;
use graphics::errors::ScreenCreateError;
use specs::{DispatcherBuilder, World};

use renderer::{Renderable, Renderer, RendererControl, Shape};

fn main() {
    if let Err(error) = run() {
        println!("{}", error);
    }
}

fn run() -> Result<(), AppError> {
    let renderer = Renderer::create()?;

    let mut world = World::new();
    world.register::<Renderable>();
    world.add_resource(RendererControl::new());

    world
        .create_entity()
        .with(Renderable::new(Shape::Ship, Color::new(1.0, 1.0, 1.0, 1.0)))
        .build();

    let mut dispatcher = DispatcherBuilder::new().add_thread_local(renderer).build();

    while !world.read_resource::<RendererControl>().should_exit {
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
