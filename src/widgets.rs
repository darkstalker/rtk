use traits::*;
use data::{Event, Property, EventCallback};

//#[derive(Debug)]
pub struct Label
{
    pub label: Property<String>,
    pub size: Property<(u32, u32)>,
    ev_handler: EventCallback<Label>,
    child: Option<Box<PushEvents>>,
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

impl HasEvents for Label
{
    fn set_ev_handler(&mut self, cb: EventCallback<Self>)
    {
        self.ev_handler = cb;
    }
}

impl PushEvents for Label
{
    fn get_nested_push_handlers<'a>(&'a self) -> Box<Iterator<Item=&'a Box<PushEvents>> + 'a>
    {
        Box::new(self.child.iter())
    }

    fn push_local_events(&self, event: &Event) -> bool
    {
        self.ev_handler.call(self, event)
    }
}

impl PullEvents for Label
{
    fn pull_events(&mut self)
    {
        if self.label.is_changed()
        {
            self.label.reset_changed();
            self.ev_handler.call(self, &Event::LabelChanged);
        }
        if self.size.is_changed()
        {
            self.label.reset_changed();
            self.ev_handler.call(self, &Event::Resized);
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

impl CanDraw for Label
{
    fn draw<T: DrawContext>(&self, mut ctx: T)
    {
        ctx.draw_text(&self.label);
    }
}
