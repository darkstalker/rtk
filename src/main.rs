extern crate rtk;
use std::io::{self, Write};
use rtk::traits::*;
use rtk::widgets::Label;

// test stuff
struct TestRenderer(io::Stdout);

impl DrawContext for TestRenderer
{
    fn draw_text(&mut self, text: &str)
    {
        writeln!(self.0, "[{}]", text).unwrap()
    }
}

fn main()
{
    let mut w = Label::default();
    w.set_label("waff");
    w.set_size(320, 240);
    w.draw(TestRenderer(io::stdout()));
    println!("{:?}", w);
}
