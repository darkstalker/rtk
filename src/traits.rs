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
    fn push_event(&self, event: &ExtEvent) -> bool;
    fn pull_events(&mut self);

    fn on_event<F>(&mut self, handler: F)
        where F: Fn(&Self, Event) -> bool + 'static, Self: Sized;
}

pub trait DrawContext
{
    fn draw_text(&mut self, text: &str);
}

pub trait CanDraw
{
    fn draw(&self, ctx: &mut DrawContext);
}

pub trait Container
{
    fn get_children(&self) -> &[Box<Widget>];
    fn get_children_mut(&mut self) -> &mut [Box<Widget>];

    fn add<T>(&mut self, obj: T)
        where T: Widget + 'static, Self: Sized;
}

pub trait Widget: HasEvents + Container + CanDraw {}
impl<T> Widget for T where T: HasEvents + Container + CanDraw {}
