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

pub trait HasPosition
{
    fn get_position(&self) -> (i32, i32);
    fn set_position(&mut self, x: i32, y: i32);
}

pub trait HasVisibility
{
    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, vis: bool);

    fn show(&mut self) { self.set_visible(true) }
    fn hide(&mut self) { self.set_visible(false) }
}

pub trait HasEvents
{
    fn push_event(&self, ev: Event) -> bool;

    fn on_event<F>(&mut self, handler: F)
        where F: Fn(&Self, Event) -> bool, Self: Sized;
}

pub trait DrawContext
{
    fn clear(&mut self);
}

pub trait CanDraw
{
    fn draw(&self, ctx: &mut DrawContext);
}

pub trait TopLevel
{
    fn push_ext_event(&self, ext_ev: &ExtEvent) -> bool;
}

pub trait Containable: HasEvents + CanDraw {}
