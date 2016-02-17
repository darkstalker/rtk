use std::path::PathBuf;

pub use glium::glutin::Event as ExtEvent;
pub use glium::glutin::{ElementState, MouseButton, VirtualKeyCode, MouseScrollDelta, Touch, TouchPhase};

#[derive(Debug, Clone, Copy)]
pub enum Event<'a>
{
    // push events (external)
    MouseMoved(i32, i32),
    MouseWheel(MouseScrollDelta),
    MouseInput(ElementState, MouseButton),
    ReceivedCharacter(char),
    KeyboardInput(ElementState, u8, Option<VirtualKeyCode>),
    Touch(Touch),
    DroppedFile(&'a PathBuf),
    WindowFocused(bool),
    WindowClosing,
    Suspended(bool),
    // pull events (from property changes)
    LabelChanged(&'a str),
    Resized(u32, u32),
    Moved(i32, i32),
}

pub fn cast<'a>(from: &'a ExtEvent) -> Option<Event<'a>>
{
    match *from {
        ExtEvent::DroppedFile(ref p) => Some(Event::DroppedFile(p)),
        ExtEvent::ReceivedCharacter(c) => Some(Event::ReceivedCharacter(c)),
        ExtEvent::KeyboardInput(st, sc, kc) => Some(Event::KeyboardInput(st, sc, kc)),
        ExtEvent::MouseMoved((x, y)) => Some(Event::MouseMoved(x, y)),
        ExtEvent::MouseWheel(sd) => Some(Event::MouseWheel(sd)),
        ExtEvent::MouseInput(st, b) => Some(Event::MouseInput(st, b)),
        ExtEvent::Touch(t) => Some(Event::Touch(t)),
        // shouldn't propagate: Resized, Moved, Closed, Focused, Awakened, Refresh, Suspended
        _ => None
    }
}
