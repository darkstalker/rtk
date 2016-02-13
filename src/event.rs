use traits::*;

#[derive(Debug, Clone, Copy)]
pub enum Event<'a>
{
    // push events
    MouseButton(u8, bool),
    // pull events
    LabelChanged(&'a str),
    Resized(u32, u32),
}

impl<'a> From<&'a ExtEvent> for Event<'a>
{
    fn from(ev: &ExtEvent) -> Self
    {
        match *ev {
            ExtEvent::MouseButton(b, p) => Event::MouseButton(b, p),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExtEvent
{
    MouseButton(u8, bool),
}

pub fn push_event(obj: &Containable, ev: &ExtEvent) -> bool
{
    if obj.get_children().iter().map(|c| push_event(&**c, ev)).any(|a| a)
    {
        return true
    }

    obj.push_event(ev)
}

pub fn pull_events(obj: &mut Containable)
{
    obj.pull_events();
    for c in obj.get_children_mut()
    {
        pull_events(&mut **c);
    }
}
