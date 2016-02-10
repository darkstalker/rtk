use std::ops::Deref;
use traits::*;

#[derive(Debug, Clone, Copy)]
pub enum Event<'a>
{
    // push events
    MouseButton(u8, bool),
    // pull events
    LabelChanged(&'a str),
    Resized(u32, u32),
}

#[derive(Debug, Clone, Copy)]
pub enum ExtEvent
{
    MouseButton(u8, bool),
}

impl ExtEvent
{
    pub fn push<T: PushEvents + Container>(self, obj: &T) -> bool
    {
        if obj.get_children().iter().map(|c| self.push(c)).any(|a| a)
        {
            return true
        }

        obj.push_event(self)
    }
}

impl<'a> Into<Event<'a>> for ExtEvent
{
    fn into(self) -> Event<'a>
    {
        match self {
            ExtEvent::MouseButton(b, p) => Event::MouseButton(b, p),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Property<T>
{
    value: T,
    changed: bool,
}

impl<T> Property<T>
{
    pub fn new(val: T) -> Property<T>
    {
        Property{ value: val, changed: false }
    }

    #[inline]
    pub fn get(&self) -> &T
    {
        &self.value
    }

    #[inline]
    pub fn set(&mut self, val: T)
    {
        self.value = val;
        self.changed = true;
    }

    pub fn consume_event(&mut self) -> bool
    {
        if self.changed
        {
            self.changed = false;
            return true;
        }
        false
    }
}

impl<T> Deref for Property<T>
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        self.get()
    }
}

impl<T> Default for Property<T> where T: Default
{
    fn default() -> Property<T>
    {
        Property::new(T::default())
    }
}

pub struct EventCallback<T>(Box<Fn(&T, Event) -> bool>);

impl<T> EventCallback<T>
{
    pub fn new<F>(f: F) -> EventCallback<T>
        where F: Fn(&T, Event) -> bool + 'static
    {
        EventCallback(Box::new(f))
    }
}

impl<T> Deref for EventCallback<T>
{
    type Target = Fn(&T, Event) -> bool;

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        &*self.0
    }
}

impl<T> Default for EventCallback<T>
{
    fn default() -> EventCallback<T>
    {
        EventCallback::new(|_, _| false)
    }
}
