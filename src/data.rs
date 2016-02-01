use std::ops::Deref;

#[derive(Debug)]
pub struct Property<T>(T);

impl<T> Property<T>
{
    pub fn new(val: T) -> Property<T>
    {
        Property(val)
    }

    pub fn get(&self) -> &T
    {
        &self.0
    }

    pub fn set(&mut self, val: T)
    {
        self.0 = val;
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
