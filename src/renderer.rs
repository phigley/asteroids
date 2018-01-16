use graphics::screen::Screen;
use graphics::events::{Event, Key};
use graphics::color::Color;
use graphics::errors::ScreenCreateError;

use cgmath::{Matrix4, Point2, Vector2};
use cgmath::prelude::*;

use specs::{FetchMut, System};

pub struct RendererControl {
    pub should_exit: bool,
}

impl RendererControl {
    pub fn new() -> Self {
        RendererControl { should_exit: false }
    }
}

pub struct Renderer {
    screen: Screen,
    clear_color: Color,
}

impl Renderer {
    pub fn create() -> Result<Self, ScreenCreateError> {
        let screen = Screen::create("Asteroids")?;
        let clear_color = Color::new(0.2, 0.2, 0.5, 1.0);

        Ok(Renderer {
            screen,
            clear_color,
        })
    }
}

impl<'a> System<'a> for Renderer {
    type SystemData = (FetchMut<'a, RendererControl>);

    fn run(&mut self, data: Self::SystemData) {
        let mut control = data;

        self.screen.clear(self.clear_color);

        // Render stuff here.

        self.screen.flush();

        self.screen.poll_events(|event| match event {
            Event::Exit => control.should_exit = true,
            _ => (),
        });
    }
}
