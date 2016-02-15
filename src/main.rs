extern crate rtk;
use rtk::traits::*;
use rtk::widgets::Window;

fn main()
{
    let mut w = Window::new().unwrap();
    w.set_label("waffle");
    w.set_size(320, 240);
    w.on_event(|_, ev| {
        println!("event1: {:?}", ev);
        true
    });
    w.event_loop();
}
