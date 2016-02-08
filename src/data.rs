use std::ops::Deref;

#[derive(Debug)]
pub enum Event<'a>
{
    // push events
    MouseButton(u8, bool),
    // pull events
    LabelChanged(&'a str),
    Resized(u32, u32),
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
