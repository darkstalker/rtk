use std::ops::Deref;
use std::cell::Cell;
use event::Event;

#[derive(Debug, Clone)]
pub struct Property<T>
{
    value: T,
    changed: Cell<bool>,
}

impl<T> Property<T>
{
    pub fn new(val: T) -> Property<T>
    {
        Property{ value: val, changed: Cell::new(false) }
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
        self.changed.set(true);
    }

    pub fn consume_event(&self) -> bool
    {
        if self.changed.get()
        {
            self.changed.set(false);
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

pub struct EventCallback<'a, T>(Box<Fn(&T, Event) -> bool + 'a>);

impl<'a, T> EventCallback<'a, T>
{
    pub fn new<F>(f: F) -> Self
        where F: Fn(&T, Event) -> bool + 'a
    {
        EventCallback(Box::new(f))
    }
}

impl<'a, T> Deref for EventCallback<'a, T>
{
    type Target = Fn(&T, Event) -> bool + 'a;

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        &*self.0
    }
}

impl<'a, T> Default for EventCallback<'a, T>
{
    fn default() -> Self
    {
        EventCallback::new(|_, _| false)
    }
}
