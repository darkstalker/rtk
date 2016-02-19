use std::ops;
use ref_slice;

use traits::*;
use data::EventCallback;
use event::{self, Event, ExtEvent};
use backend::{self, GliumWindow, GliumWindowError, GliumDrawContext};

pub struct Window<'a>
{
    window: GliumWindow,
    visible: bool,
    ev_handler: EventCallback<'a, Window<'a>>,
    child: Option<Box<Containable + 'a>>,
}

impl<'a> Window<'a>
{
    pub fn new() -> Result<Window<'a>, GliumWindowError>
    {
        let mut window = Window{
            window: try!(backend::create_window()),
            visible: false,
            ev_handler: Default::default(),
            child: None,
        };
        window.set_title("Window");
        Ok(window)
    }

    pub fn set_title(&mut self, text: &str)
    {
        self.window.get_window().unwrap().set_title(text);
    }

    pub fn add<T: Containable + 'a>(&mut self, obj: T)
    {
        self.child = Some(Box::new(obj))
    }

    // there should be a single event loop for the entire app, but glium manages this
    // per-window (in a wrong way), so we implement a single window event loop (for now)
    pub fn event_loop(&self)
    {
        for ev in self.window.wait_events()
        {
            if let ExtEvent::Closed = ev
            {
                //if !(self.ev_handler)(self, Event::WindowClosing) { break }
                break
            }

            self.push_ext_event(ev);
        }
    }
}

impl<'a> ops::Deref for Window<'a>
{
    type Target = [Box<Containable + 'a>];

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

impl<'a> HasSize for Window<'a>
{
    fn get_size(&self) -> (u32, u32)
    {
        self.window.get_window().unwrap().get_inner_size_pixels().unwrap_or((0,0))
    }

    fn set_size(&mut self, width: u32, height: u32)
    {
        self.window.get_window().unwrap().set_inner_size(width, height);
    }
}

impl<'a> HasPosition for Window<'a>
{
    fn get_position(&self) -> (i32, i32)
    {
        self.window.get_window().unwrap().get_position().unwrap_or((0, 0))
    }

    fn set_position(&mut self, x: i32, y: i32)
    {
        self.window.get_window().unwrap().set_position(x, y)
    }
}

impl<'a> TopLevel for Window<'a>
{
    fn push_ext_event(&self, ext_ev: ExtEvent)
    {
        // can propagate, pass to regular events
        if let Some(ev) = event::cast(&ext_ev)
        {
            self.push_event(ev);
            return;
        }

        // events that don't propagate
        match ext_ev {
            ExtEvent::Resized(w, h) => {
                (self.ev_handler)(self, Event::Resized(w, h));
            },
            ExtEvent::Moved(x, y) => {
                (self.ev_handler)(self, Event::Moved(x, y));
            },
            ExtEvent::Refresh => {
                let mut surface = self.window.draw();
                self.draw(&mut GliumDrawContext::new(&mut surface));
                surface.finish().unwrap();
            },
            ExtEvent::Focused(f) => {
                (self.ev_handler)(self, Event::WindowFocused(f));
            },
            ExtEvent::Suspended(s) => {
                (self.ev_handler)(self, Event::Suspended(s));
            },
            _ => ()
        }
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
