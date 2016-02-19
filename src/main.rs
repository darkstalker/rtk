extern crate rtk;
use rtk::traits::*;
use rtk::event::Event;
use rtk::widgets::Window;

fn main()
{
    let mut w = Window::new().unwrap();
    w.set_title("waffle");
    w.set_size(320, 240);
    w.on_event(|src, ev| {
        println!("event: {:?}", ev);
        match ev {
            Event::ReceivedCharacter(_) => {
                println!("position: {:?} size: {:?}", src.get_position(), src.get_size());
                true
            },
            _ => false
        }
    });
    w.show();
    w.event_loop();
}
