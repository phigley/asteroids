extern crate cgmath;
extern crate graphics;
extern crate specs;

//#[macro_use]
//extern crate specs_derive;

#[macro_use]
extern crate quick_error;

mod renderer;

use graphics::errors::ScreenCreateError;
use specs::{DispatcherBuilder, World};

use renderer::{Renderer, RendererControl};

fn main() {
    if let Err(error) = run() {
        println!("{}", error);
    }
}

fn run() -> Result<(), AppError> {
    let renderer = Renderer::create()?;

    let mut world = World::new();
    world.add_resource(RendererControl::new());

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
