use std::fmt;
use ref_slice;

use traits::*;
use data::{Property, EventCallback};
use event::{self, Event, ExtEvent};
use backend::{self, GliumWindow, GliumWindowError, GliumDrawContext};

pub struct Window
{
    label: Property<String>,
    size: Property<(u32, u32)>,
    position: Property<(i32, i32)>,
    visible: bool,
    ev_handler: EventCallback<Window>,
    child: Option<Box<Widget>>,
    window: GliumWindow,
}

impl Window
{
    pub fn new() -> Result<Window, GliumWindowError>
    {
        let mut window = Window{
            label: Default::default(),
            size: Default::default(),
            position: Default::default(),
            visible: false,
            ev_handler: Default::default(),
            child: None,
            window: try!(backend::create_window())
        };
        window.set_label("Window");
        Ok(window)
    }

    //TODO: decouple the loop from event processing
    pub fn event_loop(&mut self)
    {
        self.show();
        for ev in self.window.wait_events()
        {
            match ev {
                ExtEvent::Resized(w, h) => {
                    println!("resized {} {}", w, h);
                    self.size.set((w, h));
                },
                ExtEvent::Moved(x, y) => {
                    println!("moved {} {}", x, y);
                    self.position.set((x, y));
                },
                ExtEvent::Closed => {
                    if !self.push_event(&ev) { break }
                },
                ExtEvent::Awakened => (),
                ExtEvent::Refresh => {
                    //println!("refresh");
                    let mut surface = self.window.draw();
                    self.draw(&mut GliumDrawContext::new(&mut surface));
                    surface.finish().unwrap();
                },
                _ => { event::push_event(self, &ev); }
            }

            //event::pull_events(self);
        }
    }
}

impl fmt::Debug for Window
{
    debug_fmt!(Window, label, size);
}

impl Container for Window
{
    fn get_children(&self) -> &[Box<Widget>]
    {
        ref_slice::opt_slice(&self.child)
    }

    fn get_children_mut(&mut self) -> &mut [Box<Widget>]
    {
        ref_slice::mut_opt_slice(&mut self.child)
    }

    fn add<T>(&mut self, obj: T)
        where T: Widget + 'static
    {
        self.child = Some(Box::new(obj))
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
        self.window.get_window().unwrap().set_title(text);
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
        self.window.get_window().unwrap().set_inner_size(width, height);
        self.size.set((width, height))
    }
}

impl HasEvents for Window
{
    fn push_event(&self, event: &ExtEvent) -> bool
    {
        (self.ev_handler)(self, event.into())
    }

    fn pull_events(&mut self)
    {
        if self.label.consume_event()
        {
            (self.ev_handler)(self, Event::LabelChanged(&self.label));
        }
        if self.size.consume_event()
        {
            let (w, h) = *self.size;
            (self.ev_handler)(self, Event::Resized(w, h));
        }
        if self.position.consume_event()
        {
            let (x, y) = *self.position;
            (self.ev_handler)(self, Event::Moved(x, y));
        }
    }

    fn on_event<F>(&mut self, handler: F)
        where F: Fn(&Self, Event) -> bool + 'static
    {
        self.ev_handler = EventCallback::new(handler);
    }
}

impl HasVisibility for Window
{
    fn is_visible(&self) -> bool
    {
        self.visible
    }

    fn set_visible(&mut self, vis: bool)
    {
        if self.visible != vis
        {
            self.visible = vis;
            let win = self.window.get_window().unwrap();
            if vis { win.show() } else { win.hide() }
        }
    }
}

impl CanDraw for Window
{
    fn draw(&self, ctx: &mut DrawContext)
    {
        ctx.clear();

        if let Some(ref child) = self.child
        {
            child.draw(ctx)
        }
    }
}
