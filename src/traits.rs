pub trait HasLabel
{
    fn get_label(&self) -> &str;
    fn set_label(&mut self, label: &str);
}

pub trait HasSize
{
    fn get_size(&self) -> (i32, i32);
    fn set_size(&mut self, width: i32, height: i32);
}

pub trait DrawContext
{
    fn draw_text(&mut self, text: &str);
}

pub trait CanDraw
{
    fn draw<T: DrawContext>(&self, ctx: T);
}
