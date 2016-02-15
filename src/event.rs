use std::path::PathBuf;
use traits::*;

pub use glium::glutin::Event as ExtEvent;
pub use glium::glutin::{ElementState, MouseButton, VirtualKeyCode, MouseScrollDelta, Touch, TouchPhase};

#[derive(Debug, Clone, Copy)]
pub enum Event<'a>
{
    // shouldn't propagate: Resized, Moved, Awakened, Refresh
    Unused(&'a ExtEvent),
    // push events
    WindowClosed,
    DroppedFile(&'a PathBuf),
    ReceivedCharacter(char),
    WindowFocused(bool),
    KeyboardInput(ElementState, u8, Option<VirtualKeyCode>),
    MouseMoved(i32, i32),
    MouseWheel(MouseScrollDelta),
    MouseInput(ElementState, MouseButton),
    Suspended(bool),
    Touch(Touch),
    // pull events
    LabelChanged(&'a str),
    Resized(u32, u32),
    Moved(i32, i32),
}

impl<'a> From<&'a ExtEvent> for Event<'a>
{
    fn from(ev: &'a ExtEvent) -> Self
    {
        match *ev {
            ExtEvent::Closed => Event::WindowClosed,
            ExtEvent::DroppedFile(ref p) => Event::DroppedFile(p),
            ExtEvent::ReceivedCharacter(c) => Event::ReceivedCharacter(c),
            ExtEvent::Focused(f) => Event::WindowFocused(f),
            ExtEvent::KeyboardInput(st, sc, kc) => Event::KeyboardInput(st, sc, kc),
            ExtEvent::MouseMoved((x, y)) => Event::MouseMoved(x, y),
            ExtEvent::MouseWheel(sd) => Event::MouseWheel(sd),
            ExtEvent::MouseInput(st, b) => Event::MouseInput(st, b),
            ExtEvent::Suspended(s) => Event::Suspended(s),
            ExtEvent::Touch(t) => Event::Touch(t),
            _ => Event::Unused(ev),
        }
    }
}

pub fn push_event(obj: &Widget, ev: &ExtEvent) -> bool
{
    if obj.get_children().iter().map(|c| push_event(&**c, ev)).any(|a| a)
    {
        return true
    }

    obj.push_event(ev)
}

pub fn pull_events(obj: &mut Widget)
{
    obj.pull_events();
    for c in obj.get_children_mut()
    {
        pull_events(&mut **c);
    }
}
