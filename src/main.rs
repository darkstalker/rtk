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
    let mut w = Label::new("waff");
    //w.set_label("waff");
    w.set_size(320, 240);
    w.on_event(|_, ev| {
        println!("event: {:?}", ev);
        true
    });
    w.draw(TestRenderer(io::stdout()));
    //println!("{:?}", w);
    w.push_events(&rtk::data::Event::MouseButton(1, true));
    w.pull_events();
}
