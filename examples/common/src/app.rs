use std::{cell::RefCell, num::NonZero, rc::Rc, thread::sleep, time::Duration};

use features::MultisampleQualityLevelsFeature;
use oxidx::dx::*;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::Window,
};

use crate::game_timer::GameTimer;

#[derive(Debug)]
pub struct WindowContext {
    pub window: Window,
    pub hwnd: NonZero<isize>,

    pub swapchain: Swapchain1,
    pub current_back_buffer: usize,
    pub swapchain_buffer: [Resource; Self::SWAP_CHAIN_BUFFER_COUNT],
    pub depth_buffer: Resource,

    pub rtv_heap: DescriptorHeap,
    pub dsv_heap: DescriptorHeap,

    pub viewport: Viewport,
    pub rect: Rect,
}

#[derive(Debug)]
pub struct Base {
    pub device: Device,
    pub factory: Factory4,

    pub fence: Fence,
    pub current_fence: u64,

    pub cmd_queue: CommandQueue,
    pub cmd_list_alloc: CommandAllocator,
    pub cmd_list: GraphicsCommandList,

    pub rtv_descriptor_size: u32,
    pub dsv_descriptor_size: u32,
    pub cbv_srv_uav_descriptor_size: u32,

    pub client_width: u32,
    pub client_height: u32,
    pub back_buffer_format: Format,
    pub depth_stencil_format: Format,

    pub title: String,
    pub app_paused: bool,
    pub minimized: bool,
    pub maximized: bool,
    pub resizing: bool,
    pub fullscreen: bool,

    pub msaa_4x_quality: u32,
    pub msaa_state: bool,

    pub context: Option<WindowContext>,
    pub timer: GameTimer,
}

impl Base {
    pub(crate) fn new() -> Self {
        let client_width = 1280;
        let client_height = 720;
        let back_buffer_format = Format::Bgra8Unorm;
        let depth_stencil_format = Format::D24UnormS8Uint;

        let mut flags = FactoryCreationFlags::empty();
        if cfg!(debug_assertions) {
            let debug_controller: Debug = Entry.create_debug().unwrap();
            debug_controller.enable_debug_layer();

            flags = FactoryCreationFlags::Debug;
        }

        let factory: Factory4 = Entry.create_factory(flags).unwrap();

        let device: Device =
            if let Ok(device) = Entry.create_device(ADAPTER_NONE, FeatureLevel::Level11) {
                device
            } else {
                let adapter = factory.enum_warp_adapters().unwrap();
                Entry
                    .create_device(Some(&adapter), FeatureLevel::Level11)
                    .unwrap()
            };

        let mut feature = MultisampleQualityLevelsFeature::new(back_buffer_format, 4);
        device.check_feature_support(&mut feature).unwrap();

        assert!(feature.num_quality_levels() > 0);

        let fence = device.create_fence(0, FenceFlags::empty()).unwrap();

        let command_queue = device
            .create_command_queue(&CommandQueueDesc::direct())
            .unwrap();
        let cmd_list_alloc = device
            .create_command_allocator(CommandListType::Direct)
            .unwrap();
        let cmd_list: GraphicsCommandList = device
            .create_command_list(0, CommandListType::Direct, &cmd_list_alloc, PSO_NONE)
            .unwrap();
        cmd_list.reset(&cmd_list_alloc, PSO_NONE).unwrap();

        let rtv_descriptor_size =
            device.get_descriptor_handle_increment_size(DescriptorHeapType::Rtv);
        let dsv_descriptor_size =
            device.get_descriptor_handle_increment_size(DescriptorHeapType::Dsv);
        let cbv_srv_uav_descriptor_size =
            device.get_descriptor_handle_increment_size(DescriptorHeapType::CbvSrvUav);

        Self {
            device,
            factory,
            context: None,

            title: "Dx Sample".to_string(),
            app_paused: false,
            minimized: false,
            maximized: false,
            resizing: false,
            fullscreen: false,

            msaa_4x_quality: feature.num_quality_levels(),
            msaa_state: false,

            client_width,
            client_height,

            back_buffer_format,
            depth_stencil_format,

            fence,
            current_fence: 0,

            cmd_queue: command_queue,
            cmd_list_alloc,
            cmd_list,

            rtv_descriptor_size,
            dsv_descriptor_size,
            cbv_srv_uav_descriptor_size,
            timer: Default::default(),
        }
    }

    fn bind_window(&mut self, window: Window) {
        let Ok(RawWindowHandle::Win32(hwnd)) = window.window_handle().map(|h| h.as_raw()) else {
            panic!()
        };
        let hwnd = hwnd.hwnd;

        let swapchain = self.create_swapchain(hwnd);

        let rtv_heap: DescriptorHeap = self
            .device
            .create_descriptor_heap(&DescriptorHeapDesc::rtv(
                WindowContext::SWAP_CHAIN_BUFFER_COUNT as u32,
            ))
            .unwrap();
        let dsv_heap: DescriptorHeap = self
            .device
            .create_descriptor_heap(&DescriptorHeapDesc::dsv(1))
            .unwrap();

        let rtv_handle = rtv_heap.get_cpu_descriptor_handle_for_heap_start();

        let swapchain_buffer = std::array::from_fn(|i| {
            let render_target: Resource = swapchain.get_buffer(i).unwrap();
            self.device.create_render_target_view(
                Some(&render_target),
                None,
                rtv_handle.forward(i, self.rtv_descriptor_size as usize),
            );

            render_target
        });

        let depth_buffer = self
            .device
            .create_committed_resource(
                &HeapProperties::default(),
                HeapFlags::empty(),
                &ResourceDesc::texture_2d(self.client_width as u64, self.client_height)
                    .with_format(self.depth_stencil_format)
                    .with_sample_desc(if self.msaa_state {
                        SampleDesc::new(4, self.msaa_4x_quality)
                    } else {
                        SampleDesc::new(1, 0)
                    })
                    .with_layout(TextureLayout::Unknown),
                ResourceStates::Common,
                Some(&ClearValue::depth(self.depth_stencil_format, 1.0, 0)),
            )
            .unwrap();

        self.device.create_depth_stencil_view(
            Some(&depth_buffer),
            None,
            dsv_heap.get_cpu_descriptor_handle_for_heap_start(),
        );

        self.cmd_list
            .resource_barrier(&[ResourceBarrier::transition(
                &depth_buffer,
                ResourceStates::Common,
                ResourceStates::DepthWrite,
            )]);

        let viewport = Viewport::from_size((self.client_width as f32, self.client_height as f32));

        self.cmd_list.rs_set_viewports(&[viewport]);

        let rect = Rect::default().with_size((self.client_width as i32, self.client_height as i32));

        self.cmd_list.rs_set_scissor_rects(&[rect]);

        self.cmd_queue
            .execute_command_lists(&[Some(self.cmd_list.clone())]);
        self.flush_command_queue();

        let context = WindowContext {
            window,
            hwnd,
            swapchain,
            swapchain_buffer,
            current_back_buffer: 0,
            depth_buffer,
            rtv_heap,
            dsv_heap,
            viewport,
            rect,
        };

        self.context = Some(context);
    }

    fn on_resize(&mut self) {}

    fn create_swapchain(&self, hwnd: NonZero<isize>) -> Swapchain1 {
        let swapchain_desc = SwapchainDesc1::new(self.client_width, self.client_height)
            .with_buffer_count(WindowContext::SWAP_CHAIN_BUFFER_COUNT as u32)
            .with_usage(FrameBufferUsage::RenderTargetOutput)
            .with_sample_desc(if self.msaa_state {
                SampleDesc::new(4, self.msaa_4x_quality)
            } else {
                SampleDesc::new(1, 0)
            })
            .with_swap_effect(SwapEffect::FlipDiscard)
            .with_format(self.back_buffer_format);

        self.factory
            .create_swapchain_for_hwnd(&self.cmd_queue, hwnd, &swapchain_desc, None, OUTPUT_NONE)
            .unwrap()
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.client_width as f32 / self.client_height as f32
    }

    pub fn flush_command_queue(&mut self) {
        self.current_fence += 1;
        self.cmd_queue
            .signal(&self.fence, self.current_fence)
            .unwrap();

        if self.fence.get_completed_value() < self.current_fence {
            let event = Event::create(false, false).unwrap();
            self.fence
                .set_event_on_completion(self.current_fence, event)
                .unwrap();
            event.wait(u32::MAX);
            event.close().unwrap();
        }
    }
}

impl WindowContext {
    pub const SWAP_CHAIN_BUFFER_COUNT: usize = 2;

    pub fn current_back_buffer_view(&self, rtv_descriptor_size: u32) -> CpuDescriptorHandle {
        self.rtv_heap
            .get_cpu_descriptor_handle_for_heap_start()
            .forward(self.current_back_buffer, rtv_descriptor_size as usize)
    }

    pub fn depth_stencil_view(&self) -> CpuDescriptorHandle {
        self.dsv_heap.get_cpu_descriptor_handle_for_heap_start()
    }
}

pub trait DxSample {
    fn new(base: Rc<RefCell<Base>>) -> Self;
    fn init_resources(&mut self);
    fn update(&mut self, timer: &GameTimer);
    fn render(&mut self, timer: &GameTimer);
}

#[derive(Debug)]
pub struct SampleRunner<S: DxSample> {
    pub(crate) base: Rc<RefCell<Base>>,
    pub(crate) sample: S,
}

impl<S: DxSample> ApplicationHandler for SampleRunner<S> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        {
            let mut base = self.base.borrow_mut();
            let window_attributes = Window::default_attributes()
                .with_title(&base.title)
                .with_inner_size(PhysicalSize::new(base.client_width, base.client_height));
            let window = event_loop.create_window(window_attributes).unwrap();
            base.bind_window(window);
        }

        self.sample.init_resources();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.base.borrow_mut().timer.tick();
        let timer = self.base.borrow().timer;
        match event {
            WindowEvent::Resized(size) => {}

            WindowEvent::RedrawRequested => {
                if self.base.borrow().app_paused {
                    sleep(Duration::from_millis(100));
                    return;
                }

                self.sample.update(&timer);
                self.sample.render(&timer);
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(context) = self.base.borrow().context.as_ref() {
            context.window.request_redraw();
        }
    }
}
