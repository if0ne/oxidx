use app::{Base, DxSample, SampleRunner};
use winit::event_loop::{ControlFlow, EventLoop};

pub mod app;
pub mod game_timer;
pub mod geometry_generator;
pub mod geometry_mesh;
pub mod upload_buffer;
pub mod utils;
pub mod material;

pub fn run_sample<S: DxSample>() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut base = Base::new();
    let mut app = SampleRunner {
        sample: S::new(&mut base),
        base,
    };
    event_loop.run_app(&mut app).unwrap();
}
