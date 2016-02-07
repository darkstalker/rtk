use std::ops::Deref;
use traits::PropChanged;

#[derive(Debug)]
pub enum Event
{
    // push events
    MouseButton(u8, bool),
    // pull events
    LabelChanged,
    Resized,
}

#[derive(Debug)]
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

    pub fn get(&self) -> &T
    {
        &self.value
    }

    pub fn set(&mut self, val: T)
    {
        self.value = val;
        self.changed = true;
    }
}

impl<T> PropChanged for Property<T>
{
    fn is_changed(&self) -> bool
    {
        self.changed
    }

    fn reset_changed(&mut self)
    {
        self.changed = false;
    }
}

impl<T> Deref for Property<T>
{
    type Target = T;

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

pub struct EventCallback<T>(Box<Fn(&T, &Event) -> bool>);

impl<T> EventCallback<T>
{
    pub fn new<F>(f: F) -> EventCallback<T>
        where F: Fn(&T, &Event) -> bool + 'static
    {
        EventCallback(Box::new(f))
    }

    pub fn call(&self, obj: &T, ev: &Event) -> bool
    {
        (self.0)(obj, ev)
    }
}

impl<T> Default for EventCallback<T>
{
    fn default() -> EventCallback<T>
    {
        EventCallback::new(|_, _| false)
    }
}
