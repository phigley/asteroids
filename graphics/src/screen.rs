use nalgebra::{Matrix4, Point2, Similarity2};
use std::time::{Duration, Instant};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::color;
use crate::cursor::Cursor;
use crate::errors::ScreenCreateError;
use crate::events;
use crate::graphic_device::GraphicDevice;
use crate::model;
use crate::shape::Shape;
use crate::utils;
use crate::vertex::Vertex;

pub trait ScreenCallbacks {
    fn handle_event(&mut self, _screen: &mut Screen, _event: events::Event) {}
    fn update(&mut self, _screen: &mut Screen, _frame_delta: f32) {}
    fn render(&self, _screen_render: ScreenRender) {}
}

pub struct ScreenRunner {
    event_loop: EventLoop<()>,
    pub screen: Screen,
}

impl ScreenRunner {
    pub fn create(
        width: f64,
        height: f64,
        title: &str,
        clear_color: color::Color,
    ) -> Result<ScreenRunner, ScreenCreateError> {
        let event_loop = EventLoop::new();
        let screen = Screen::create(width, height, title, clear_color, &event_loop)?;

        Ok(ScreenRunner { event_loop, screen })
    }

    pub fn run<C: 'static + ScreenCallbacks>(self, mut callbacks: C) -> ! {
        let mut screen = self.screen;
        self.event_loop.run(move |winit_event, _, control_flow| {
            screen.handle_event(winit_event, control_flow, &mut callbacks)
        });
    }
}

pub struct Screen {
    window: Window,
    clear_color: wgpu::Color,

    device: GraphicDevice,
    cursor: Cursor,

    pending_resize: bool,
    logical_size: LogicalSize<f64>,
    dpi_factor: f64,

    last_frame_time: Instant,
    next_frame_time: Instant,
}

impl Screen {
    fn create(
        width: f64,
        height: f64,
        title: &str,
        clear_color: color::Color,
        event_loop: &EventLoop<()>,
    ) -> Result<Screen, ScreenCreateError> {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height))
            .build(event_loop)
            .map_err(ScreenCreateError::WindowCreateFailure)?;

        let (device, logical_size, dpi_factor) = GraphicDevice::create(&window)?;

        let wgpu_clear_color = wgpu::Color {
            r: clear_color.r as f64,
            g: clear_color.g as f64,
            b: clear_color.b as f64,
            a: clear_color.a as f64,
        };

        Ok(Screen {
            window,
            clear_color: wgpu_clear_color,
            device,
            cursor: Cursor::new(width, height),
            pending_resize: false,
            logical_size,
            dpi_factor,

            last_frame_time: Instant::now(),
            next_frame_time: Instant::now(),
        })
    }

    pub fn create_shape(
        &mut self,
        points: &[Point2<f32>],
        indices: &[u16],
        name: &'static str,
    ) -> Shape {
        let size_points = points.len();

        let mut vertex_data = Vec::with_capacity(size_points);

        for p in points.iter() {
            vertex_data.push(Vertex::new(p.x, p.y));
        }

        self.device
            .create_shape(vertex_data.as_slice(), &indices, name)
    }

    pub fn create_circle(&mut self, radius: f32, vertices: usize) -> Shape {
        let (vertex_data, indices) = utils::build_circle(radius, vertices);

        self.device
            .create_shape(vertex_data.as_slice(), &indices, "circle")
    }

    pub fn handle_event<C: 'static + ScreenCallbacks, T>(
        &mut self,
        winit_event: Event<'_, T>,
        control_flow: &mut ControlFlow,
        callbacks: &mut C,
    ) {
        match winit_event {
            Event::WindowEvent {
                event: ref window_event,
                window_id,
                ..
            } => {
                if window_id == self.window.id() {
                    *control_flow = self.handle_window_event(window_event, callbacks);
                }
            }
            Event::MainEventsCleared => {
                let current_time = Instant::now();
                let frame_delta = (current_time - self.last_frame_time).as_secs_f32();
                self.next_frame_time = current_time + Duration::from_millis(33);

                callbacks.update(self, frame_delta);

                self.trigger_possible_resize(callbacks);

                callbacks.render(ScreenRender {
                    device: &mut self.device,
                });

                self.device.render_frame(self.clear_color);

                *control_flow = ControlFlow::WaitUntil(self.next_frame_time)
            }

            Event::RedrawRequested(window_id) => {
                if window_id == self.window.id() {
                    self.trigger_possible_resize(callbacks);

                    callbacks.render(ScreenRender {
                        device: &mut self.device,
                    });

                    self.device.render_frame(self.clear_color);
                }
            }
            _ => *control_flow = ControlFlow::WaitUntil(self.next_frame_time),
        }
    }

    pub fn get_mouse_pos(&self) -> Point2<f32> {
        self.cursor.get_mouse_pos()
    }

    fn handle_window_event<C: ScreenCallbacks>(
        &mut self,
        window_event: &WindowEvent,
        callbacks: &mut C,
    ) -> ControlFlow {
        match window_event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            }
            | WindowEvent::CloseRequested => {
                callbacks.handle_event(self, events::Event::Exit);
                ControlFlow::Exit
            }

            WindowEvent::Resized(physical_size) => {
                self.pending_resize = true;
                self.logical_size = physical_size.to_logical(self.dpi_factor);
                ControlFlow::WaitUntil(self.next_frame_time)
            }

            WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {
                self.pending_resize = true;
                self.logical_size = new_inner_size.to_logical(*scale_factor);
                ControlFlow::WaitUntil(self.next_frame_time)
            }

            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(winit_key),
                        state,
                        ..
                    },
                ..
            } => {
                if let Some(key) = match_key(*winit_key) {
                    let down = match state {
                        ElementState::Pressed => true,
                        ElementState::Released => false,
                    };

                    callbacks.handle_event(self, events::Event::KeyPress { key, down });
                }
                ControlFlow::WaitUntil(self.next_frame_time)
            }

            WindowEvent::CursorMoved {
                position: logical_pos,
                ..
            } => {
                if let Some(pos) = self.cursor.mouse_moved(logical_pos.x, logical_pos.y) {
                    callbacks.handle_event(self, events::Event::MouseMove { pos });
                }
                ControlFlow::WaitUntil(self.next_frame_time)
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if self.cursor.on_screen() {
                    match button {
                        MouseButton::Left => callbacks.handle_event(
                            self,
                            events::Event::MouseLMB {
                                down: *state == ElementState::Pressed,
                            },
                        ),
                        MouseButton::Right => callbacks.handle_event(
                            self,
                            events::Event::MouseRMB {
                                down: *state == ElementState::Pressed,
                            },
                        ),
                        MouseButton::Middle => callbacks.handle_event(
                            self,
                            events::Event::MouseMMB {
                                down: *state == ElementState::Pressed,
                            },
                        ),
                        _ => (),
                    }
                }
                ControlFlow::WaitUntil(self.next_frame_time)
            }

            _ => ControlFlow::WaitUntil(self.next_frame_time),
        }
    }

    fn trigger_possible_resize<C: ScreenCallbacks>(&mut self, callbacks: &mut C) {
        if self.pending_resize {
            self.pending_resize = false;

            self.device
                .set_window_size(&self.logical_size, self.dpi_factor);
            self.cursor
                .set_window_size(self.logical_size.width, self.logical_size.height);

            let mouse_pos = self.cursor.get_mouse_pos();
            callbacks.handle_event(self, events::Event::Resize { mouse_pos });
        }
    }
}

pub struct ScreenRender<'a> {
    device: &'a mut GraphicDevice,
}

impl<'a> ScreenRender<'a> {
    /// Draws a model.
    pub fn draw_model(&mut self, model: &model::Model) {
        self.draw_shape(&model.transform, model.color, &model.shape);
    }

    // Draw a flat colored shape.
    pub fn draw_shape(&mut self, transform: &Similarity2<f32>, color: color::Color, shape: &Shape) {
        let vals = transform.to_homogeneous();
        #[rustfmt::skip]
        let draw_transform = Matrix4::new(
            vals[0], vals[3], 0.0, vals[6],
            vals[1], vals[4], 0.0, vals[7],
            0.0, 0.0, 0.0, 0.0,
            vals[2], vals[5], 0.0, vals[8]);

        self.device.draw_shape(draw_transform, color, shape);
    }
}

fn match_key(glutin_key: VirtualKeyCode) -> Option<events::Key> {
    match glutin_key {
        VirtualKeyCode::Key1 => Some(events::Key::Key1),
        VirtualKeyCode::Key2 => Some(events::Key::Key2),
        VirtualKeyCode::Key3 => Some(events::Key::Key3),
        VirtualKeyCode::Key4 => Some(events::Key::Key4),
        VirtualKeyCode::Key5 => Some(events::Key::Key5),
        VirtualKeyCode::Key6 => Some(events::Key::Key6),
        VirtualKeyCode::Key7 => Some(events::Key::Key7),
        VirtualKeyCode::Key8 => Some(events::Key::Key8),
        VirtualKeyCode::Key9 => Some(events::Key::Key9),
        VirtualKeyCode::Key0 => Some(events::Key::Key0),
        VirtualKeyCode::A => Some(events::Key::A),
        VirtualKeyCode::B => Some(events::Key::B),
        VirtualKeyCode::C => Some(events::Key::C),
        VirtualKeyCode::D => Some(events::Key::D),
        VirtualKeyCode::E => Some(events::Key::E),
        VirtualKeyCode::F => Some(events::Key::F),
        VirtualKeyCode::G => Some(events::Key::G),
        VirtualKeyCode::H => Some(events::Key::H),
        VirtualKeyCode::I => Some(events::Key::I),
        VirtualKeyCode::J => Some(events::Key::J),
        VirtualKeyCode::K => Some(events::Key::K),
        VirtualKeyCode::L => Some(events::Key::L),
        VirtualKeyCode::M => Some(events::Key::M),
        VirtualKeyCode::N => Some(events::Key::N),
        VirtualKeyCode::O => Some(events::Key::O),
        VirtualKeyCode::P => Some(events::Key::P),
        VirtualKeyCode::Q => Some(events::Key::Q),
        VirtualKeyCode::R => Some(events::Key::R),
        VirtualKeyCode::S => Some(events::Key::S),
        VirtualKeyCode::T => Some(events::Key::T),
        VirtualKeyCode::U => Some(events::Key::U),
        VirtualKeyCode::V => Some(events::Key::V),
        VirtualKeyCode::W => Some(events::Key::W),
        VirtualKeyCode::X => Some(events::Key::X),
        VirtualKeyCode::Y => Some(events::Key::Y),
        VirtualKeyCode::Z => Some(events::Key::Z),
        VirtualKeyCode::Insert => Some(events::Key::Insert),
        VirtualKeyCode::Home => Some(events::Key::Home),
        VirtualKeyCode::Delete => Some(events::Key::Delete),
        VirtualKeyCode::End => Some(events::Key::End),
        VirtualKeyCode::PageDown => Some(events::Key::PageDown),
        VirtualKeyCode::PageUp => Some(events::Key::PageUp),
        VirtualKeyCode::Left => Some(events::Key::Left),
        VirtualKeyCode::Up => Some(events::Key::Up),
        VirtualKeyCode::Right => Some(events::Key::Right),
        VirtualKeyCode::Down => Some(events::Key::Down),
        VirtualKeyCode::Back => Some(events::Key::Back),
        VirtualKeyCode::Return => Some(events::Key::Return),
        VirtualKeyCode::Space => Some(events::Key::Space),
        VirtualKeyCode::Numpad0 => Some(events::Key::Numpad0),
        VirtualKeyCode::Numpad1 => Some(events::Key::Numpad1),
        VirtualKeyCode::Numpad2 => Some(events::Key::Numpad2),
        VirtualKeyCode::Numpad3 => Some(events::Key::Numpad3),
        VirtualKeyCode::Numpad4 => Some(events::Key::Numpad4),
        VirtualKeyCode::Numpad5 => Some(events::Key::Numpad5),
        VirtualKeyCode::Numpad6 => Some(events::Key::Numpad6),
        VirtualKeyCode::Numpad7 => Some(events::Key::Numpad7),
        VirtualKeyCode::Numpad8 => Some(events::Key::Numpad8),
        VirtualKeyCode::Numpad9 => Some(events::Key::Numpad9),
        VirtualKeyCode::Add => Some(events::Key::Add),
        VirtualKeyCode::At => Some(events::Key::At),
        VirtualKeyCode::Backslash => Some(events::Key::Backslash),
        VirtualKeyCode::Colon => Some(events::Key::Colon),
        VirtualKeyCode::Comma => Some(events::Key::Comma),
        VirtualKeyCode::Decimal => Some(events::Key::Decimal),
        VirtualKeyCode::Divide => Some(events::Key::Divide),
        VirtualKeyCode::Equals => Some(events::Key::Equals),
        VirtualKeyCode::Grave => Some(events::Key::Grave),
        VirtualKeyCode::LAlt => Some(events::Key::LAlt),
        VirtualKeyCode::LBracket => Some(events::Key::LBracket),
        VirtualKeyCode::LControl => Some(events::Key::LControl),
        VirtualKeyCode::LShift => Some(events::Key::LShift),
        VirtualKeyCode::Minus => Some(events::Key::Minus),
        VirtualKeyCode::Multiply => Some(events::Key::Multiply),
        VirtualKeyCode::NumpadComma => Some(events::Key::NumpadComma),
        VirtualKeyCode::NumpadEnter => Some(events::Key::NumpadEnter),
        VirtualKeyCode::NumpadEquals => Some(events::Key::NumpadEquals),
        VirtualKeyCode::Period => Some(events::Key::Period),
        VirtualKeyCode::RAlt => Some(events::Key::RAlt),
        VirtualKeyCode::RBracket => Some(events::Key::RBracket),
        VirtualKeyCode::RControl => Some(events::Key::RControl),
        VirtualKeyCode::RShift => Some(events::Key::RShift),
        VirtualKeyCode::Semicolon => Some(events::Key::Semicolon),
        VirtualKeyCode::Slash => Some(events::Key::Slash),
        VirtualKeyCode::Subtract => Some(events::Key::Subtract),
        VirtualKeyCode::Tab => Some(events::Key::Tab),
        _ => None,
    }
}
