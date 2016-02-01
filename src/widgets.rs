use traits::*;
use data::Property;

#[derive(Debug)]
pub struct Label
{
    label: Property<String>,
    size: Property<(i32, i32)>,
}

impl Label
{
    pub fn new(text: &str) -> Label
    {
        Label{
            label: Property::new(text.to_owned()),
            size: Property::new((0, 0)),
        }
    }
}

impl Default for Label
{
    fn default() -> Label
    {
        Label{
            label: Property::new(String::with_capacity(0)),
            size: Property::new((0, 0)),
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
    fn get_size(&self) -> (i32, i32)
    {
        *self.size.get()
    }

    fn set_size(&mut self, width: i32, height: i32)
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
