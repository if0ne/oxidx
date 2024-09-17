use std::{
    cell::{Cell, RefCell},
    num::NonZero,
    thread::sleep,
    time::Duration,
};

use features::MultisampleQualityLevelsFeature;
use oxidx::dx::*;
use tracing::debug;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{DeviceEvent, DeviceId, MouseButton, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::Window,
};

use crate::game_timer::GameTimer;

#[derive(Debug)]
pub struct SwapchainContext {
    pub window: Window,
    pub hwnd: NonZero<isize>,

    pub swapchain: Swapchain1,
    pub current_back_buffer: Cell<usize>,
    pub swapchain_buffer: Option<[Resource; Self::BUFFER_COUNT]>,
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
    pub info_queue: Option<InfoQueue1>,

    pub fence: Fence,
    pub current_fence: u64,

    pub cmd_queue: CommandQueue,
    pub cmd_list_alloc: CommandAllocator,
    pub cmd_list: GraphicsCommandList,

    pub rtv_descriptor_size: usize,
    pub dsv_descriptor_size: usize,
    pub cbv_srv_uav_descriptor_size: usize,

    pub client_width: u32,
    pub client_height: u32,
    pub back_buffer_format: Format,
    pub depth_stencil_format: Format,

    pub title: String,
    pub app_paused: bool,

    pub msaa_4x_quality: u32,
    pub msaa_state: bool,

    pub context: Option<SwapchainContext>,
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
            let debug_controller: Debug = create_debug().unwrap();
            debug_controller.enable_debug_layer();

            flags = FactoryCreationFlags::Debug;
        }

        let factory: Factory4 = create_factory(flags).unwrap();

        let device: Device = if let Ok(device) = create_device(ADAPTER_NONE, FeatureLevel::Level11)
        {
            device
        } else {
            let adapter = factory.enum_warp_adapters().unwrap();
            create_device(Some(&adapter), FeatureLevel::Level11).unwrap()
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

        cmd_list.close().unwrap();

        let rtv_descriptor_size =
            device.get_descriptor_handle_increment_size(DescriptorHeapType::Rtv);
        let dsv_descriptor_size =
            device.get_descriptor_handle_increment_size(DescriptorHeapType::Dsv);
        let cbv_srv_uav_descriptor_size =
            device.get_descriptor_handle_increment_size(DescriptorHeapType::CbvSrvUav);

        if cfg!(debug_assertions) {
            Self::log_adapters(&factory, back_buffer_format);
        }

        Self {
            device,
            factory,
            info_queue: None,
            context: None,

            title: "Dx Sample".to_string(),
            app_paused: false,

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
            .create_descriptor_heap(&DescriptorHeapDesc::rtv(SwapchainContext::BUFFER_COUNT))
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
                rtv_handle.advance(i, self.rtv_descriptor_size),
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
                    .with_layout(TextureLayout::Unknown)
                    .with_mip_levels(1)
                    .with_flags(ResourceFlags::AllowDepthStencil),
                ResourceStates::Common,
                Some(&ClearValue::depth(self.depth_stencil_format, 1.0, 0)),
            )
            .unwrap();

        self.device.create_depth_stencil_view(
            Some(&depth_buffer),
            None,
            dsv_heap.get_cpu_descriptor_handle_for_heap_start(),
        );

        self.cmd_list.reset(&self.cmd_list_alloc, PSO_NONE).unwrap();

        self.cmd_list
            .resource_barrier(&[ResourceBarrier::transition(
                &depth_buffer,
                ResourceStates::Common,
                ResourceStates::DepthWrite,
            )]);

        self.cmd_list.close().unwrap();

        let viewport = Viewport::from_size((self.client_width as f32, self.client_height as f32));
        let rect = Rect::default().with_size((self.client_width as i32, self.client_height as i32));

        self.cmd_queue
            .execute_command_lists(&[Some(self.cmd_list.clone())]);
        self.flush_command_queue();

        let context = SwapchainContext {
            window,
            hwnd,
            swapchain,
            swapchain_buffer: Some(swapchain_buffer),
            current_back_buffer: Default::default(),
            depth_buffer,
            rtv_heap,
            dsv_heap,
            viewport,
            rect,
        };

        self.context = Some(context);
    }

    fn on_resize(&mut self, new_width: u32, new_height: u32) {
        self.flush_command_queue();

        self.cmd_list.reset(&self.cmd_list_alloc, PSO_NONE).unwrap();

        let Some(ref mut context) = self.context else {
            return;
        };

        self.client_width = new_width;
        self.client_height = new_height;

        if let Some(i) = std::mem::take(&mut context.swapchain_buffer) {
            drop(i);
        }

        context
            .swapchain
            .resize_buffers(
                SwapchainContext::BUFFER_COUNT,
                self.client_width,
                self.client_height,
                self.back_buffer_format,
                SwapchainFlags::AllowModeSwitch,
            )
            .unwrap();
        context.current_back_buffer = Default::default();

        let rtv_handle = context.rtv_heap.get_cpu_descriptor_handle_for_heap_start();

        let swapchain_buffer = std::array::from_fn(|i| {
            let render_target: Resource = context.swapchain.get_buffer(i).unwrap();
            self.device.create_render_target_view(
                Some(&render_target),
                None,
                rtv_handle.advance(i, self.rtv_descriptor_size),
            );

            render_target
        });

        let depth_buffer = self
            .device
            .create_committed_resource(
                &HeapProperties::default(),
                HeapFlags::empty(),
                &ResourceDesc::texture_2d(self.client_width as u64, self.client_height)
                    .with_format(Format::R24G8Typeless)
                    .with_sample_desc(if self.msaa_state {
                        SampleDesc::new(4, self.msaa_4x_quality)
                    } else {
                        SampleDesc::new(1, 0)
                    })
                    .with_layout(TextureLayout::Unknown)
                    .with_mip_levels(1)
                    .with_flags(ResourceFlags::AllowDepthStencil),
                ResourceStates::Common,
                Some(&ClearValue::depth(self.depth_stencil_format, 1.0, 0)),
            )
            .unwrap();

        let dsv_desc = DepthStencilViewDesc::texture_2d(self.depth_stencil_format, 0);
        self.device.create_depth_stencil_view(
            Some(&depth_buffer),
            Some(&dsv_desc),
            context.dsv_heap.get_cpu_descriptor_handle_for_heap_start(),
        );

        self.cmd_list
            .resource_barrier(&[ResourceBarrier::transition(
                &depth_buffer,
                ResourceStates::Common,
                ResourceStates::DepthWrite,
            )]);

        self.cmd_list.close().unwrap();

        let viewport = Viewport::from_size((self.client_width as f32, self.client_height as f32));
        let rect = Rect::default().with_size((self.client_width as i32, self.client_height as i32));

        self.cmd_queue
            .execute_command_lists(&[Some(self.cmd_list.clone())]);

        context.swapchain_buffer = Some(swapchain_buffer);
        context.depth_buffer = depth_buffer;
        context.viewport = viewport;
        context.rect = rect;

        self.flush_command_queue();
    }

    fn create_swapchain(&self, hwnd: NonZero<isize>) -> Swapchain1 {
        let swapchain_desc = SwapchainDesc1::new(self.client_width, self.client_height)
            .with_buffer_count(SwapchainContext::BUFFER_COUNT as u32)
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

    fn set_msaa_4x_state(&mut self, state: bool) {
        if self.msaa_state == state {
            return;
        }

        self.msaa_state = state;

        let Some(hwnd) = self.context.as_ref().map(|c| c.hwnd) else {
            return;
        };

        let swapchain = self.create_swapchain(hwnd);
        self.on_resize(self.client_width, self.client_height);

        let Some(ref mut context) = self.context else {
            return;
        };
        context.swapchain = swapchain;
    }

    fn calculate_frame_stats(&self) {
        thread_local! {
            static FRAME_CNT: RefCell<i32> = Default::default();
            static TIME_ELAPSED: RefCell<f32> = Default::default();
        }

        FRAME_CNT.with_borrow_mut(|frame_cnt| {
            *frame_cnt += 1;
        });

        TIME_ELAPSED.with_borrow_mut(|time_elapsed| {
            if self.timer.total_time() - *time_elapsed > 1.0 {
                FRAME_CNT.with_borrow_mut(|frame_cnt| {
                    let fps = *frame_cnt as f32;
                    let mspf = 1000.0 / fps;

                    if let Some(ref context) = self.context {
                        context
                            .window
                            .set_title(&format!("{} Fps: {fps} Ms: {mspf}", self.title))
                    }

                    *frame_cnt = 0;
                    *time_elapsed += 1.0;
                });
            }
        })
    }

    fn log_adapters(factory: &Factory4, format: Format) {
        let mut i = 0;

        while let Ok(adapter) = factory.enum_adapters(i) {
            let desc = adapter.get_desc1().unwrap();
            debug!(name: "Adapter", description = %desc.description());

            Self::log_adapter_outputs(&adapter, format);

            i += 1;
        }
    }

    fn log_adapter_outputs(adapter: &Adapter3, format: Format) {
        let mut i = 0;

        while let Ok(output) = adapter.enum_outputs(i) {
            let desc = output.get_desc().unwrap();

            debug!(name: "  Output", device_name = %desc.device_name());

            Self::log_output_display_mode(&output, format);

            i += 1;
        }
    }

    fn log_output_display_mode(output: &Output1, format: Format) {
        let modes = output
            .get_display_mode_list1(format, EnumModeFlags::empty())
            .unwrap();

        for mode in modes {
            debug!(name: "    Mode", width = %mode.width());
            debug!(name: "    Mode", height = %mode.height());
            debug!(name: "    Mode", refresh_rate = ?mode.refresh_rate());
        }
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

impl Drop for Base {
    fn drop(&mut self) {
        self.flush_command_queue();
    }
}

impl SwapchainContext {
    pub const BUFFER_COUNT: usize = 2;

    pub fn current_back_buffer(&self) -> &Resource {
        &self.swapchain_buffer.as_ref().unwrap()[self.current_back_buffer.get()]
    }

    pub fn current_back_buffer_view(&self, rtv_descriptor_size: usize) -> CpuDescriptorHandle {
        self.rtv_heap
            .get_cpu_descriptor_handle_for_heap_start()
            .advance(self.current_back_buffer.get(), rtv_descriptor_size)
    }

    pub fn depth_stencil_view(&self) -> CpuDescriptorHandle {
        self.dsv_heap.get_cpu_descriptor_handle_for_heap_start()
    }
}

pub trait DxSample {
    fn new(base: &mut Base) -> Self;
    fn init_resources(&mut self, base: &Base);
    fn update(&mut self, base: &Base);
    fn render(&mut self, base: &mut Base);
    fn on_resize(&mut self, base: &mut Base, width: u32, height: u32);

    fn on_key_down(&mut self, base: &Base, key: KeyCode, repeat: bool);
    fn on_key_up(&mut self, key: KeyCode);

    fn on_mouse_down(&mut self, btn: MouseButton);
    fn on_mouse_up(&mut self, btn: MouseButton);
    fn on_mouse_move(&mut self, x: f64, y: f64);
}

#[derive(Debug)]
pub(crate) struct SampleRunner<S: DxSample> {
    pub(crate) base: Base,
    pub(crate) sample: S,
}

impl<S: DxSample> ApplicationHandler for SampleRunner<S> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        {
            let window_attributes = Window::default_attributes()
                .with_title(&self.base.title)
                .with_inner_size(PhysicalSize::new(
                    self.base.client_width,
                    self.base.client_height,
                ));
            let window = event_loop.create_window(window_attributes).unwrap();
            self.base.bind_window(window);
        }

        self.sample.init_resources(&self.base);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.base.timer.tick();
        match event {
            WindowEvent::Focused(focused) => {
                if focused {
                    self.base.app_paused = false;
                    self.base.timer.start();
                } else {
                    self.base.app_paused = true;
                    self.base.timer.stop();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => match event.state {
                winit::event::ElementState::Pressed => {
                    if let PhysicalKey::Code(code) = event.physical_key {
                        self.sample.on_key_down(&self.base, code, event.repeat);
                    }
                }
                winit::event::ElementState::Released => {
                    if event.physical_key == PhysicalKey::Code(KeyCode::F2) {
                        self.base.set_msaa_4x_state(!self.base.msaa_state);
                    } else if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                        event_loop.exit()
                    }

                    if let PhysicalKey::Code(code) = event.physical_key {
                        self.sample.on_key_up(code);
                    }
                }
            },
            WindowEvent::MouseInput { state, button, .. } => match state {
                winit::event::ElementState::Pressed => self.sample.on_mouse_down(button),
                winit::event::ElementState::Released => self.sample.on_mouse_up(button),
            },
            WindowEvent::Resized(size) => {
                let Some(ref mut context) = self.base.context else {
                    return;
                };

                if context.window.is_minimized().is_some_and(|minized| minized) {
                    self.base.app_paused = true;
                } else {
                    self.base.app_paused = false;
                    self.base.on_resize(size.width, size.height);
                    self.sample
                        .on_resize(&mut self.base, size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if self.base.app_paused {
                    sleep(Duration::from_millis(100));
                    return;
                }
                self.base.calculate_frame_stats();
                self.sample.update(&self.base);
                self.sample.render(&mut self.base);
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }
    }

    #[allow(clippy::single_match)]
    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => self.sample.on_mouse_move(delta.0, delta.0),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(context) = self.base.context.as_ref() {
            context.window.request_redraw();
        }
    }
}
