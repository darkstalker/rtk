use std::fmt;
use traits::*;
use data::{Event, Property, EventCallback};
use utils;

pub struct Label
{
    label: Property<String>,
    size: Property<(u32, u32)>,
    ev_handler: EventCallback<Label>,
    child: Option<Box<Containable>>,
}

impl Label
{
    pub fn new(text: &str) -> Label
    {
        Label{
            label: Property::new(text.to_owned()),
            size: Default::default(),
            ev_handler: Default::default(),
            child: None,
        }
    }
}

impl fmt::Debug for Label
{
    debug_fmt!(Label, label, size);
}

impl Containable for Label {}

impl Container for Label
{
    fn get_children(&self) -> &[Box<Containable>]
    {
        utils::option_as_slice(&self.child)
    }

    fn get_children_mut(&mut self) -> &mut [Box<Containable>]
    {
        utils::option_as_slice_mut(&mut self.child)
    }

    fn add<T>(&mut self, obj: T)
        where T: Containable + 'static
    {
        self.child = Some(Box::new(obj))
    }
}

impl PushEvents for Label
{
    fn push_event_local(&self, event: &Event) -> bool
    {
        (self.ev_handler)(self, event)
    }
}

impl PullEvents for Label
{
    fn pull_events_local(&mut self)
    {
        if self.label.consume_event()
        {
            (self.ev_handler)(self, &Event::LabelChanged(self.label.get()));
        }
        if self.size.consume_event()
        {
            let (w, h) = *self.size.get();
            (self.ev_handler)(self, &Event::Resized(w, h));
        }
    }
}

impl HasLabel for Label
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

impl HasSize for Label
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

impl HasEvents for Label
{
    fn on_event<F>(&mut self, handler: F)
        where F: Fn(&Self, &Event) -> bool + 'static
    {
        self.ev_handler = EventCallback::new(handler);
    }
}

impl CanDraw for Label
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
