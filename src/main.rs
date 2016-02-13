extern crate rtk;
use std::io::{self, Write};
use rtk::traits::*;
use rtk::event::{self, ExtEvent};
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
    let mut w2 = Label::new("b");
    w2.on_event(|_, ev| {
        println!("event2: {:?}", ev);
        true
    });

    let mut w = Label::new("a");
    w.add(w2);
    w.set_label("waffle");
    w.set_size(320, 240);
    w.on_event(|_, ev| {
        println!("event1: {:?}", ev);
        true
    });

    println!("{:?}", w);
    w.draw(&mut TestRenderer(io::stdout()));
    event::push_event(&w, &ExtEvent::MouseButton(1, true));
    event::pull_events(&mut w);
}
