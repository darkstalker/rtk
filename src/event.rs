use traits::*;

pub use glium::glutin::Event as ExtEvent;
pub use glium::glutin::{ElementState, MouseButton};

#[derive(Debug, Clone, Copy)]
pub enum Event<'a>
{
    // push events
    MouseInput(ElementState, MouseButton),
    // pull events
    LabelChanged(&'a str),
    Resized(u32, u32),
}

impl<'a> From<&'a ExtEvent> for Event<'a>
{
    fn from(ev: &ExtEvent) -> Self
    {
        match *ev {
            ExtEvent::MouseInput(st, b) => Event::MouseInput(st, b),
            _ => unimplemented!()
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
