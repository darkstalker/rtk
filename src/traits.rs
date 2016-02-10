use event::{Event, ExtEvent};

pub trait HasLabel
{
    fn get_label(&self) -> &str;
    fn set_label(&mut self, label: &str);
}

pub trait HasSize
{
    fn get_size(&self) -> (u32, u32);
    fn set_size(&mut self, width: u32, height: u32);
}

pub trait HasEvents
{
    fn on_event<F>(&mut self, handler: F)
        where F: Fn(&Self, Event) -> bool + 'static;
}

pub trait DrawContext
{
    fn draw_text(&mut self, text: &str);
}

pub trait CanDraw
{
    fn draw(&self, ctx: &mut DrawContext);
}

pub trait Containable: PushEvents + PullEvents + Container + CanDraw {}

pub trait Container
{
    fn get_children(&self) -> &[Box<Containable>];
    fn get_children_mut(&mut self) -> &mut [Box<Containable>];
    fn add<T>(&mut self, obj: T)
        where T: Containable + 'static, Self: Sized;
}

pub trait PushEvents
{
    fn push_event(&self, event: ExtEvent) -> bool;
}

pub trait PullEvents
{
    fn pull_events(&mut self);
}
