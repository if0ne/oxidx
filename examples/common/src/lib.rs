use app::{Base, DxSample, SampleRunner};
use winit::event_loop::{ControlFlow, EventLoop};

pub mod app;
pub mod game_timer;
pub mod utils;

pub fn run_sample<S: DxSample>() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let base = Base::new();
    let mut app = SampleRunner {
        sample: S::new(&base),
        base,
    };
    event_loop.run_app(&mut app).unwrap();
}
