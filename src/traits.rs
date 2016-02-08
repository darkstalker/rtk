use data::{Event, EventCallback};

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

pub trait DrawContext
{
    fn draw_text(&mut self, text: &str);
}

pub trait CanDraw
{
    fn draw(&self, ctx: &mut DrawContext);
}

pub trait Containable: PushEvents + CanDraw {}

pub trait Container
{
    fn get_children(&self) -> &[Box<Containable>];
}

pub trait HasEvents where Self: Sized
{
    fn set_ev_handler(&mut self, cb: EventCallback<Self>);

    fn on_event<F>(&mut self, handler: F)
        where F: Fn(&Self, &Event) -> bool + 'static
    {
        self.set_ev_handler(EventCallback::new(handler));
    }
}

pub trait PushEvents: Container
{
    fn push_local_events(&self, event: &Event) -> bool;

    fn push_events(&self, event: &Event) -> bool
    {
        if self.get_children().iter().map(|c| c.push_events(event)).any(|a| a)
        {
            return true
        }

        self.push_local_events(event)
    }
}

pub trait PullEvents
{
    fn pull_events(&mut self);
}
