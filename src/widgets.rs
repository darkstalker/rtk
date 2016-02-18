use std::fmt;
use std::ops;
use std::thread;
use std::time::Duration;
use ref_slice;

use traits::*;
use data::{Property, EventCallback};
use event::{self, Event, ExtEvent};
use backend::{self, GliumWindow, GliumWindowError, GliumDrawContext};

const EVENT_LOOP_DELAY: u64 = 1000 / 125;

pub struct Window<'a>
{
    label: Property<String>,
    size: Property<(u32, u32)>,
    position: Property<(i32, i32)>,
    visible: bool,
    ev_handler: EventCallback<'a, Window<'a>>,
    child: Option<Box<Widget + 'a>>,
    window: GliumWindow,
}

impl<'a> Window<'a>
{
    pub fn new() -> Result<Window<'a>, GliumWindowError>
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

    pub fn add<T: Widget + 'a>(&mut self, obj: T)
    {
        self.child = Some(Box::new(obj))
    }

    pub fn event_loop(&mut self)
    {
        self.show();

        'ev: loop
        {
            let events: Vec<ExtEvent> = self.window.poll_events().collect();
            if events.is_empty()
            {
                thread::sleep(Duration::from_millis(EVENT_LOOP_DELAY));
                continue;
            }

            for ev in events
            {
                if self.push_ext_event(&ev) { break 'ev }
                self.pull_events();
            }
        }
    }
}

impl<'a> fmt::Debug for Window<'a>
{
    debug_fmt!(Window, label, size);
}

impl<'a> ops::Deref for Window<'a>
{
    type Target = [Box<Widget + 'a>];

    fn deref(&self) -> &Self::Target
    {
        ref_slice::opt_slice(&self.child)
    }
}

impl<'a> ops::DerefMut for Window<'a>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        ref_slice::mut_opt_slice(&mut self.child)
    }
}

impl<'a> HasLabel for Window<'a>
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

impl<'a> HasSize for Window<'a>
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

impl<'a> TopLevel for Window<'a>
{
    fn push_ext_event(&mut self, ext_ev: &ExtEvent) -> bool
    {
        match event::cast(ext_ev) {
            // can propagate, pass to regular events
            Some(ev) => {
                self.push_event(ev);
            },
            // events that don't propagate
            None => match *ext_ev {
                ExtEvent::Resized(w, h) => {
                    println!("resized {} {}", w, h);
                    self.size.set((w, h));
                },
                ExtEvent::Moved(x, y) => {
                    println!("moved {} {}", x, y);
                    self.position.set((x, y));
                },
                ExtEvent::Refresh => {
                    let mut surface = self.window.draw();
                    self.draw(&mut GliumDrawContext::new(&mut surface));
                    surface.finish().unwrap();
                },
                // pass directly to handler
                ExtEvent::Focused(f) => {
                    (self.ev_handler)(self, Event::WindowFocused(f));
                },
                ExtEvent::Suspended(s) => {
                    (self.ev_handler)(self, Event::Suspended(s));
                },
                // this should be conditional, but it isn't atm
                ExtEvent::Closed => {
                    //return !(self.ev_handler)(self, Event::WindowClosing);
                    return true;
                },
                _ => ()
            },
        }
        false
    }
}

impl<'a> HasEvents for Window<'a>
{
    fn push_event(&self, ev: Event) -> bool
    {
        if let Some(ref child) = self.child
        {
            if child.push_event(ev)
            {
                return true
            }
        }

        (self.ev_handler)(self, ev)
    }

    fn pull_events(&self)
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

        if let Some(ref child) = self.child
        {
            child.pull_events();
        }
    }

    fn on_event<F>(&mut self, handler: F)
        where F: Fn(&Self, Event) -> bool + 'a
    {
        self.ev_handler = EventCallback::new(handler);
    }
}

impl<'a> HasVisibility for Window<'a>
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

impl<'a> CanDraw for Window<'a>
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
