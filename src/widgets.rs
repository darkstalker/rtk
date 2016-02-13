use ref_slice;
use std::fmt;
use traits::*;
use data::{Property, EventCallback};
use event::{Event, ExtEvent};

pub struct Window
{
    label: Property<String>,
    size: Property<(u32, u32)>,
    ev_handler: EventCallback<Window>,
    child: Option<Box<Containable>>,
}

impl Window
{
    pub fn new(text: &str) -> Window
    {
        Window{
            label: Property::new(text.to_owned()),
            size: Default::default(),
            ev_handler: Default::default(),
            child: None,
        }
    }
}

impl fmt::Debug for Window
{
    debug_fmt!(Window, label, size);
}

impl Container for Window
{
    fn get_children(&self) -> &[Box<Containable>]
    {
        ref_slice::opt_slice(&self.child)
    }

    fn get_children_mut(&mut self) -> &mut [Box<Containable>]
    {
        ref_slice::mut_opt_slice(&mut self.child)
    }

    fn add<T>(&mut self, obj: T)
        where T: Containable + 'static
    {
        self.child = Some(Box::new(obj))
    }
}

impl PushEvents for Window
{
    fn push_event(&self, event: &ExtEvent) -> bool
    {
        (self.ev_handler)(self, event.into())
    }
}

impl PullEvents for Window
{
    fn pull_events(&mut self)
    {
        if self.label.consume_event()
        {
            (self.ev_handler)(self, Event::LabelChanged(self.label.get()));
        }
        if self.size.consume_event()
        {
            let (w, h) = *self.size.get();
            (self.ev_handler)(self, Event::Resized(w, h));
        }
    }
}

impl HasLabel for Window
{
    fn get_label(&self) -> &str
    {
        &self.label.get()
    }

    fn set_label(&mut self, text: &str)
    {
        self.label.set(text.to_owned())
    }
}

impl HasSize for Window
{
    fn get_size(&self) -> (u32, u32)
    {
        *self.size.get()
    }

    fn set_size(&mut self, width: u32, height: u32)
    {
        self.size.set((width, height))
    }
}

impl HasEvents for Window
{
    fn on_event<F>(&mut self, handler: F)
        where F: Fn(&Self, Event) -> bool + 'static
    {
        self.ev_handler = EventCallback::new(handler);
    }
}

impl CanDraw for Window
{
    fn draw(&self, ctx: &mut DrawContext)
    {
        ctx.draw_text(&self.label);

        if let Some(ref child) = self.child
        {
            child.draw(ctx)
        }
    }
}
