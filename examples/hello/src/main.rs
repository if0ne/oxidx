use common::{
    app::{DxSample, SwapchainContext},
    run_sample,
};
use oxidx::dx::{
    ClearFlags, ICommandAllocator, ICommandQueue, IGraphicsCommandList, ISwapchain1, PresentFlags,
    ResourceBarrier, ResourceStates, PSO_NONE,
};
use tracing_subscriber::layer::SubscriberExt;

fn main() {
    let console_log = tracing_subscriber::fmt::Layer::new()
        .with_ansi(true)
        .with_writer(std::io::stdout);

    let subscriber = tracing_subscriber::registry().with(console_log);

    let _ = tracing::subscriber::set_global_default(subscriber);
    run_sample::<HelloSample>();
}

#[derive(Debug)]
pub struct HelloSample;

impl DxSample for HelloSample {
    fn new(_: &common::app::Base) -> Self {
        HelloSample
    }

    fn init_resources(&mut self, _: &common::app::Base) {}

    fn update(&mut self, _: &common::app::Base) {}

    fn render(&mut self, base: &mut common::app::Base) {
        let Some(ref context) = base.context else {
            return;
        };

        base.cmd_list_alloc.reset().unwrap();

        base.cmd_list.reset(&base.cmd_list_alloc, PSO_NONE).unwrap();

        base.cmd_list
            .resource_barrier(&[ResourceBarrier::transition(
                context.current_back_buffer(),
                ResourceStates::Present,
                ResourceStates::RenderTarget,
            )]);

        base.cmd_list.rs_set_viewports(&[context.viewport]);
        base.cmd_list.rs_set_scissor_rects(&[context.rect]);

        base.cmd_list.clear_render_target_view(
            context.current_back_buffer_view(base.rtv_descriptor_size),
            [0.32, 0.78, 0.0, 1.0],
            &[],
        );
        base.cmd_list.clear_depth_stencil_view(
            context.depth_stencil_view(),
            ClearFlags::Depth | ClearFlags::Stencil,
            1.0,
            0,
            &[],
        );

        base.cmd_list.om_set_render_targets(
            &[context.current_back_buffer_view(base.rtv_descriptor_size)],
            true,
            Some(context.depth_stencil_view()),
        );

        base.cmd_list
            .resource_barrier(&[ResourceBarrier::transition(
                context.current_back_buffer(),
                ResourceStates::RenderTarget,
                ResourceStates::Present,
            )]);

        base.cmd_list.close().unwrap();
        base.cmd_queue
            .execute_command_lists(&[Some(base.cmd_list.clone())]);
        context.swapchain.present(0, PresentFlags::empty()).unwrap();
        context
            .current_back_buffer
            .set((context.current_back_buffer.get() + 1) % SwapchainContext::BUFFER_COUNT);

        base.flush_command_queue();
    }

    fn on_key_down(&mut self, _: winit::keyboard::KeyCode, _: bool) {}

    fn on_key_up(&mut self, _: winit::keyboard::KeyCode) {}

    fn on_mouse_down(&mut self, _: winit::event::MouseButton) {}

    fn on_mouse_up(&mut self, _: winit::event::MouseButton) {}

    fn on_mouse_move(&mut self, _: f64, _: f64) {}
}
