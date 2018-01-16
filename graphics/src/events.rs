use cgmath::Point2;

pub enum Key {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    Add,
    At,
    Backslash,
    Colon,
    Comma,
    Decimal,
    Divide,
    Equals,
    Grave,
    LAlt,
    LBracket,
    LControl,
    LShift,
    Minus,
    Multiply,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    Period,
    RAlt,
    RBracket,
    RControl,
    RShift,
    Semicolon,
    Slash,
    Subtract,
    Tab,
}

/// Possible event types that can occur
pub enum Event {
    /// Exit has been requested, ie. window was closed.
    Exit,

    /// A screen resize has occured.
    /// mouse_pos is the new mouse position, assuming that it stayed at the same pixel location.
    Resize { mouse_pos: Point2<f32> },

    /// Key was pressed or released
    KeyPress { key: Key, down: bool },

    /// Mouse has moved. Position is relative to center of screen
    /// and goes from -1.0 to 1.0 for the shortest dimension.
    /// Positive x axis points to right, positive y axis points to top.
    MouseMove { pos: Point2<f32> },

    /// Left mouse button.
    MouseLMB { down: bool },

    /// Right mouse button.
    MouseRMB { down: bool },

    /// Middle mouse button.
    MouseMMB { down: bool },
}
