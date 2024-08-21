use std::{cell::RefCell, rc::Rc};

use app::{Base, DxSample, SampleRunner};
use winit::event_loop::{ControlFlow, EventLoop};

pub mod app;
pub mod game_timer;

pub fn run_sample<S: DxSample>() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let base = Rc::new(RefCell::new(Base::new()));
    let mut app = SampleRunner {
        sample: S::new(Rc::clone(&base)),
        base,
    };
    event_loop.run_app(&mut app).unwrap();
}
