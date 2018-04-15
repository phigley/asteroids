use glutin;

use cgmath::{Matrix4, Point2};

use super::color;
use super::cursor::Cursor;
use super::errors;
use super::events;
use super::graphic_device::GraphicDevice;
use super::model;
use super::shape;
use super::utils;

pub struct Screen {
    events_loop: glutin::EventsLoop,
    implementation: ScreenImplementation,
}

impl Screen {
    pub fn create(title: &str) -> Result<Screen, errors::ScreenCreateError> {
        let events_loop = glutin::EventsLoop::new();

        let initial_width = 800;
        let initial_height = 600;

        let device = try!(GraphicDevice::new(
            initial_width,
            initial_height,
            title,
            &events_loop,
        ));

        Ok(Screen {
            events_loop: events_loop,
            implementation: ScreenImplementation {
                device: device,
                cursor: Cursor::new(initial_width, initial_height),
            },
        })
    }

    pub fn create_shape(&mut self, points: &[Point2<f32>], indices: &[u16]) -> shape::Shape {
        let size_points = points.len();

        let mut vertex_data = Vec::with_capacity(size_points);

        for p in points.iter() {
            vertex_data.push(super::Vertex { pos: [p.x, p.y] });
        }

        self.implementation
            .device
            .create_shape(vertex_data.as_slice(), &indices)
    }

    pub fn create_circle(&mut self, radius: f32, vertices: usize) -> shape::Shape {
        let (vertex_data, indices) = utils::build_circle(radius, vertices);

        self.implementation
            .device
            .create_shape(vertex_data.as_slice(), &indices)
    }

    pub fn poll_events<F>(&mut self, mut callback: F)
    where
        F: FnMut(events::Event),
    {
        let ref mut implementation = self.implementation;

        self.events_loop
            .poll_events(|glutin_event| match glutin_event {
                glutin::Event::WindowEvent {
                    window_id: _,
                    event: window_event,
                } => implementation.handle_window_event(window_event, &mut callback),

                glutin::Event::DeviceEvent { .. } => (),

                glutin::Event::Awakened => (),

                glutin::Event::Suspended(_) => (),
            });
    }

    /// Clear the screen.
    /// This is intended to be called at the beginning of the draw code for the frame.
    pub fn clear(&mut self, clear_color: color::Color) {
        self.implementation.device.clear(clear_color);
    }

    /// Draws a model.
    pub fn draw_model(&mut self, model: &model::Model) {
        self.implementation
            .device
            .draw_shape(&model.transform, model.color, &model.shape);
    }

    // Draw a flat colored shape.
    pub fn draw_shape(
        &mut self,
        transform: &Matrix4<f32>,
        color: color::Color,
        shape: &shape::Shape,
    ) {
        self.implementation
            .device
            .draw_shape(transform, color, shape);
    }

    /// Issues the draw commands to the GPU.
    /// This must be called at the end of the draw frame.
    pub fn flush(&mut self) {
        self.implementation.device.flush();
    }

    pub fn get_mouse_pos(&self) -> Point2<f32> {
        self.implementation.cursor.get_mouse_pos()
    }
}

struct ScreenImplementation {
    device: GraphicDevice,
    cursor: Cursor,
}

impl ScreenImplementation {
    fn handle_window_event<F>(&mut self, window_event: glutin::WindowEvent, callback: &mut F)
    where
        F: FnMut(events::Event),
    {
        match window_event {
            glutin::WindowEvent::KeyboardInput {
                input:
                    glutin::KeyboardInput {
                        virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                        ..
                    },
                ..
            }
            | glutin::WindowEvent::Closed => callback(events::Event::Exit),

            glutin::WindowEvent::Resized(width, height) => {
                self.device.set_window_size(width, height);
                self.cursor.set_window_size(width, height);

                let mouse_pos = self.cursor.get_mouse_pos();
                callback(events::Event::Resize { mouse_pos });
            }

            glutin::WindowEvent::KeyboardInput {
                input:
                    glutin::KeyboardInput {
                        virtual_keycode: Some(glutin_key),
                        state,
                        ..
                    },
                ..
            } => {
                if let Some(key) = match_key(glutin_key) {
                    let down = match state {
                        glutin::ElementState::Pressed => true,
                        glutin::ElementState::Released => false,
                    };

                    callback(events::Event::KeyPress { key, down });
                }
            }

            glutin::WindowEvent::CursorMoved {
                position: (x_pixel, y_pixel),
                ..
            } => match self.cursor.mouse_moved(x_pixel, y_pixel) {
                Some(pos) => callback(events::Event::MouseMove { pos }),
                None => (),
            },

            glutin::WindowEvent::MouseInput { state, button, .. } => {
                if self.cursor.on_screen() {
                    match button {
                        glutin::MouseButton::Left => callback(events::Event::MouseLMB {
                            down: state == glutin::ElementState::Pressed,
                        }),
                        glutin::MouseButton::Right => callback(events::Event::MouseRMB {
                            down: state == glutin::ElementState::Pressed,
                        }),
                        glutin::MouseButton::Middle => callback(events::Event::MouseMMB {
                            down: state == glutin::ElementState::Pressed,
                        }),
                        _ => (),
                    }
                } else {
                    ()
                }
            }

            _ => (),
        }
    }
}

fn match_key(glutin_key: glutin::VirtualKeyCode) -> Option<events::Key> {
    match glutin_key {
        glutin::VirtualKeyCode::Key1 => Some(events::Key::Key1),
        glutin::VirtualKeyCode::Key2 => Some(events::Key::Key2),
        glutin::VirtualKeyCode::Key3 => Some(events::Key::Key3),
        glutin::VirtualKeyCode::Key4 => Some(events::Key::Key4),
        glutin::VirtualKeyCode::Key5 => Some(events::Key::Key5),
        glutin::VirtualKeyCode::Key6 => Some(events::Key::Key6),
        glutin::VirtualKeyCode::Key7 => Some(events::Key::Key7),
        glutin::VirtualKeyCode::Key8 => Some(events::Key::Key8),
        glutin::VirtualKeyCode::Key9 => Some(events::Key::Key9),
        glutin::VirtualKeyCode::Key0 => Some(events::Key::Key0),
        glutin::VirtualKeyCode::A => Some(events::Key::A),
        glutin::VirtualKeyCode::B => Some(events::Key::B),
        glutin::VirtualKeyCode::C => Some(events::Key::C),
        glutin::VirtualKeyCode::D => Some(events::Key::D),
        glutin::VirtualKeyCode::E => Some(events::Key::E),
        glutin::VirtualKeyCode::F => Some(events::Key::F),
        glutin::VirtualKeyCode::G => Some(events::Key::G),
        glutin::VirtualKeyCode::H => Some(events::Key::H),
        glutin::VirtualKeyCode::I => Some(events::Key::I),
        glutin::VirtualKeyCode::J => Some(events::Key::J),
        glutin::VirtualKeyCode::K => Some(events::Key::K),
        glutin::VirtualKeyCode::L => Some(events::Key::L),
        glutin::VirtualKeyCode::M => Some(events::Key::M),
        glutin::VirtualKeyCode::N => Some(events::Key::N),
        glutin::VirtualKeyCode::O => Some(events::Key::O),
        glutin::VirtualKeyCode::P => Some(events::Key::P),
        glutin::VirtualKeyCode::Q => Some(events::Key::Q),
        glutin::VirtualKeyCode::R => Some(events::Key::R),
        glutin::VirtualKeyCode::S => Some(events::Key::S),
        glutin::VirtualKeyCode::T => Some(events::Key::T),
        glutin::VirtualKeyCode::U => Some(events::Key::U),
        glutin::VirtualKeyCode::V => Some(events::Key::V),
        glutin::VirtualKeyCode::W => Some(events::Key::W),
        glutin::VirtualKeyCode::X => Some(events::Key::X),
        glutin::VirtualKeyCode::Y => Some(events::Key::Y),
        glutin::VirtualKeyCode::Z => Some(events::Key::Z),
        glutin::VirtualKeyCode::Insert => Some(events::Key::Insert),
        glutin::VirtualKeyCode::Home => Some(events::Key::Home),
        glutin::VirtualKeyCode::Delete => Some(events::Key::Delete),
        glutin::VirtualKeyCode::End => Some(events::Key::End),
        glutin::VirtualKeyCode::PageDown => Some(events::Key::PageDown),
        glutin::VirtualKeyCode::PageUp => Some(events::Key::PageUp),
        glutin::VirtualKeyCode::Left => Some(events::Key::Left),
        glutin::VirtualKeyCode::Up => Some(events::Key::Up),
        glutin::VirtualKeyCode::Right => Some(events::Key::Right),
        glutin::VirtualKeyCode::Down => Some(events::Key::Down),
        glutin::VirtualKeyCode::Back => Some(events::Key::Back),
        glutin::VirtualKeyCode::Return => Some(events::Key::Return),
        glutin::VirtualKeyCode::Space => Some(events::Key::Space),
        glutin::VirtualKeyCode::Numpad0 => Some(events::Key::Numpad0),
        glutin::VirtualKeyCode::Numpad1 => Some(events::Key::Numpad1),
        glutin::VirtualKeyCode::Numpad2 => Some(events::Key::Numpad2),
        glutin::VirtualKeyCode::Numpad3 => Some(events::Key::Numpad3),
        glutin::VirtualKeyCode::Numpad4 => Some(events::Key::Numpad4),
        glutin::VirtualKeyCode::Numpad5 => Some(events::Key::Numpad5),
        glutin::VirtualKeyCode::Numpad6 => Some(events::Key::Numpad6),
        glutin::VirtualKeyCode::Numpad7 => Some(events::Key::Numpad7),
        glutin::VirtualKeyCode::Numpad8 => Some(events::Key::Numpad8),
        glutin::VirtualKeyCode::Numpad9 => Some(events::Key::Numpad9),
        glutin::VirtualKeyCode::Add => Some(events::Key::Add),
        glutin::VirtualKeyCode::At => Some(events::Key::At),
        glutin::VirtualKeyCode::Backslash => Some(events::Key::Backslash),
        glutin::VirtualKeyCode::Colon => Some(events::Key::Colon),
        glutin::VirtualKeyCode::Comma => Some(events::Key::Comma),
        glutin::VirtualKeyCode::Decimal => Some(events::Key::Decimal),
        glutin::VirtualKeyCode::Divide => Some(events::Key::Divide),
        glutin::VirtualKeyCode::Equals => Some(events::Key::Equals),
        glutin::VirtualKeyCode::Grave => Some(events::Key::Grave),
        glutin::VirtualKeyCode::LAlt => Some(events::Key::LAlt),
        glutin::VirtualKeyCode::LBracket => Some(events::Key::LBracket),
        glutin::VirtualKeyCode::LControl => Some(events::Key::LControl),
        glutin::VirtualKeyCode::LShift => Some(events::Key::LShift),
        glutin::VirtualKeyCode::Minus => Some(events::Key::Minus),
        glutin::VirtualKeyCode::Multiply => Some(events::Key::Multiply),
        glutin::VirtualKeyCode::NumpadComma => Some(events::Key::NumpadComma),
        glutin::VirtualKeyCode::NumpadEnter => Some(events::Key::NumpadEnter),
        glutin::VirtualKeyCode::NumpadEquals => Some(events::Key::NumpadEquals),
        glutin::VirtualKeyCode::Period => Some(events::Key::Period),
        glutin::VirtualKeyCode::RAlt => Some(events::Key::RAlt),
        glutin::VirtualKeyCode::RBracket => Some(events::Key::RBracket),
        glutin::VirtualKeyCode::RControl => Some(events::Key::RControl),
        glutin::VirtualKeyCode::RShift => Some(events::Key::RShift),
        glutin::VirtualKeyCode::Semicolon => Some(events::Key::Semicolon),
        glutin::VirtualKeyCode::Slash => Some(events::Key::Slash),
        glutin::VirtualKeyCode::Subtract => Some(events::Key::Subtract),
        glutin::VirtualKeyCode::Tab => Some(events::Key::Tab),
        _ => None,
    }
}
