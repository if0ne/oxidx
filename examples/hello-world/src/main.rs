use std::num::NonZeroIsize;

use oxidx::{
    adapter::*, command_allocator::*, command_list::*, command_queue::*, debug::*,
    descriptor_heap::*, device::*, factory::*, heap::*, pso::*, resources::*, swapchain::*,
    sync::*, types::*,
};
use smallvec::smallvec;
use winit::{
    dpi::PhysicalSize,
    event::{Event as EventWin, WindowEvent},
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
};

trait DXSample {
    fn new(command_line: &SampleCommandLine) -> Self
    where
        Self: Sized;

    fn bind_to_window(&mut self, hwnd: NonZeroIsize);

    fn update(&mut self) {}
    fn render(&mut self) {}

    fn title(&self) -> String {
        "DXSample".into()
    }

    fn window_size(&self) -> (i32, i32) {
        (640, 480)
    }
}

#[derive(Clone)]
struct SampleCommandLine {
    use_warp_device: bool,
}

fn build_command_line() -> SampleCommandLine {
    let mut use_warp_device = false;

    for arg in std::env::args() {
        if arg.eq_ignore_ascii_case("-warp") || arg.eq_ignore_ascii_case("/warp") {
            use_warp_device = true;
        }
    }

    SampleCommandLine { use_warp_device }
}

fn run_sample<S>()
where
    S: DXSample,
{
    let command_line = build_command_line();
    let mut sample = S::new(&command_line);

    let size = sample.window_size();

    let mut title = sample.title();

    if command_line.use_warp_device {
        title.push_str(" (WARP)");
    }

    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(size.0, size.1))
        .with_title(title)
        .build(&event_loop)
        .unwrap();

    let hwnd = if let RawWindowHandle::Win32(win) = window.window_handle().unwrap().as_raw() {
        win.hwnd
    } else {
        panic!("Unsupported OS");
    };

    sample.bind_to_window(hwnd);

    let _ = event_loop.run(|event, evt| {
        //control_flow.set_poll();

        match event {
            EventWin::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                evt.exit();
            }
            EventWin::AboutToWait => window.request_redraw(),
            EventWin::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                sample.update();
                sample.render();
            }
            _ => (),
        }
    });
}

const FRAME_COUNT: u32 = 2;

pub struct Sample {
    dxgi_factory: Factory4,
    device: Device,
    resources: Option<Resources>,
}

struct Resources {
    command_queue: CommandQueue,
    swap_chain: Swapchain3,
    frame_index: u32,
    render_targets: [Resource; FRAME_COUNT as usize],
    rtv_heap: DescriptorHeap,
    rtv_descriptor_size: usize,
    viewport: Viewport,
    scissor_rect: Rect,
    command_allocator: CommandAllocator,
    root_signature: RootSignature,
    pso: PipelineState,
    command_list: GraphicsCommandList,

    #[allow(dead_code)]
    vertex_buffer: Resource,

    vbv: VertexBufferView,
    fence: Fence,
    fence_value: u64,
    fence_event: Event,
}

impl DXSample for Sample {
    fn new(command_line: &SampleCommandLine) -> Self {
        let (dxgi_factory, device) = create_device(command_line);

        Sample {
            dxgi_factory,
            device,
            resources: None,
        }
    }

    fn bind_to_window(&mut self, hwnd: NonZeroIsize) {
        let command_queue: CommandQueue = self
            .device
            .create_command_queue(CommandQueueDesc {
                r#type: CommandListType::Direct,
                ..Default::default()
            })
            .unwrap();

        let (width, height) = self.window_size();

        let swap_chain_desc = SwapchainDesc {
            buffer_count: FRAME_COUNT,
            width: width as u32,
            height: height as u32,
            format: Format::Bgra8Unorm,
            usage: FrameBufferUsage::RenderTargetOutput,
            swap_effect: SwapEffect::FlipDiscard,
            sample_desc: SampleDesc {
                count: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        let swap_chain: Swapchain3 = self
            .dxgi_factory
            .create_swapchain_for_hwnd(
                &command_queue,
                hwnd,
                &swap_chain_desc,
                None,
                None::<&Output>,
            )
            .unwrap()
            .try_into()
            .unwrap();

        self.dxgi_factory
            .make_window_association(hwnd, WindowAssociationFlags::NoAltEnter)
            .unwrap();

        let frame_index = swap_chain.get_current_back_buffer_index();

        let rtv_heap: DescriptorHeap = self
            .device
            .create_descriptor_heap(DescriptorHeapDesc {
                num: FRAME_COUNT,
                r#type: DescriptorHeapType::Rtv,
                ..Default::default()
            })
            .unwrap();

        let rtv_descriptor_size =
            self.device
                .get_descriptor_handle_increment_size(DescriptorHeapType::Rtv) as usize;
        let rtv_handle = rtv_heap.get_cpu_descriptor_handle_for_heap_start();

        let render_targets: [Resource; FRAME_COUNT as usize] = std::array::from_fn(|i| {
            let render_target: Resource = swap_chain.get_buffer(i as u32).unwrap();
            self.device.create_render_target_view(
                &render_target,
                None,
                rtv_handle.offset(i * rtv_descriptor_size),
            );

            render_target
        });

        let viewport = Viewport::from_size((width as f32, height as f32));

        let scissor_rect = Rect::from_size((width, height));

        let command_allocator = self
            .device
            .create_command_allocator(CommandListType::Direct)
            .unwrap();

        let root_signature = create_root_signature(&self.device);
        let pso = create_pipeline_state(&self.device, &root_signature);

        let command_list: GraphicsCommandList = self
            .device
            .create_command_list(0, CommandListType::Direct, &command_allocator, &pso)
            .unwrap();

        command_list.close();

        let aspect_ratio = width as f32 / height as f32;

        let (vertex_buffer, vbv) = create_vertex_buffer(&self.device, aspect_ratio);

        let fence = self.device.create_fence(0).unwrap();

        let fence_value = 1;

        let fence_event = Event::create(false, false).unwrap();

        self.resources = Some(Resources {
            command_queue,
            swap_chain,
            frame_index,
            render_targets,
            rtv_heap,
            rtv_descriptor_size,
            viewport,
            scissor_rect,
            command_allocator,
            root_signature,
            pso,
            command_list,
            vertex_buffer,
            vbv,
            fence,
            fence_value,
            fence_event,
        });
    }

    fn title(&self) -> String {
        "D3D12 Hello Triangle".into()
    }

    fn window_size(&self) -> (i32, i32) {
        (1280, 720)
    }

    fn render(&mut self) {
        if let Some(resources) = &mut self.resources {
            populate_command_list(resources);

            let command_list = &resources.command_list;
            resources.command_queue.begin_event(0u64, c"Test");

            resources
                .command_queue
                .execute_command_lists([command_list]);

            resources.command_queue.end_event();

            resources
                .swap_chain
                .present(1, PresentFlags::empty())
                .ok()
                .unwrap();

            wait_for_previous_frame(resources);
        }
    }
}

fn populate_command_list(resources: &Resources) {
    resources.command_allocator.reset().unwrap();

    let command_list = &resources.command_list;

    command_list
        .reset(&resources.command_allocator, &resources.pso)
        .unwrap();

    command_list.set_graphics_root_signature(&resources.root_signature);
    command_list.rs_set_viewports([&resources.viewport]);
    command_list.rs_set_scissor_rects([&resources.scissor_rect]);

    let barrier = transition_barrier(
        &resources.render_targets[resources.frame_index as usize],
        ResourceState::Present,
        ResourceState::RenderTarget,
    );
    command_list.resource_barrier(&[barrier]);

    let rtv_handle = resources
        .rtv_heap
        .get_cpu_descriptor_handle_for_heap_start()
        .offset(resources.frame_index as usize * resources.rtv_descriptor_size);

    command_list.om_set_render_targets([&rtv_handle], false, None);

    command_list.clear_render_target_view(rtv_handle, [0.0_f32, 0.2_f32, 0.4_f32, 1.0_f32], []);
    command_list.ia_set_primitive_topology(PrimitiveTopology::Triangle);
    command_list.ia_set_vertex_buffers(0, [&resources.vbv]);
    command_list.draw_instanced(3, 1, 0, 0);

    let barrier = transition_barrier(
        &resources.render_targets[resources.frame_index as usize],
        ResourceState::RenderTarget,
        ResourceState::Present,
    );
    command_list.resource_barrier(&[barrier]);

    command_list.close();
}

fn transition_barrier(
    resource: &Resource,
    state_before: ResourceState,
    state_after: ResourceState,
) -> ResourceBarrier {
    ResourceBarrier {
        r#type: BarrierType::Transition {
            resource,
            subresource: BARRIER_ALL_SUBRESOURCES,
            before: state_before,
            after: state_after,
        },
        flags: BarrierFlags::empty(),
    }
}

fn create_device(command_line: &SampleCommandLine) -> (Factory4, Device) {
    let entry = Entry;

    if cfg!(debug_assertions) {
        let debug: Debug = entry.create_debug().unwrap();
        debug.enable_debug_layer();
    }

    let dxgi_factory_flags = if cfg!(debug_assertions) {
        FactoryCreationFlags::Debug
    } else {
        FactoryCreationFlags::empty()
    };

    let dxgi_factory: Factory4 = entry.create_factory(dxgi_factory_flags).unwrap();

    let adapter = if command_line.use_warp_device {
        dxgi_factory.enum_warp_adapters().unwrap()
    } else {
        get_hardware_adapter(&dxgi_factory)
    };

    let device = entry
        .create_device(&adapter, FeatureLevel::Level11)
        .unwrap();

    (dxgi_factory, device)
}

fn get_hardware_adapter(factory: &Factory4) -> Adapter3 {
    let entry = Entry;
    for i in 0.. {
        let adapter = factory.enum_adapters(i).unwrap();

        let desc = adapter.get_desc1();

        if (desc.flags & AdapterFlags::Sofware) != AdapterFlags::empty() {
            continue;
        }

        if entry
            .create_device::<_, Device>(&adapter, FeatureLevel::Level11)
            .is_ok()
        {
            return adapter;
        }
    }

    unreachable!()
}

fn create_root_signature(device: &Device) -> RootSignature {
    let desc = RootSignatureDesc {
        flags: RootSignatureFlags::AllowInputAssemblerInputLayout,
        ..Default::default()
    };

    device
        .serialize_create_root_signature(&desc, RootSignatureVersion::V1_0, 0)
        .unwrap()
}

fn create_pipeline_state(device: &Device, root_signature: &RootSignature) -> PipelineState {
    let compile_flags = if cfg!(debug_assertions) {
        COMPILE_DEBUG | COMPILE_SKIP_OPT
    } else {
        0
    };

    let exe_path = std::env::current_exe().ok().unwrap();
    let asset_path = exe_path.parent().unwrap();
    let shaders_hlsl_path = asset_path.join("shaders.hlsl");

    let vertex_shader =
        Blob::compile_from_file(&shaders_hlsl_path, c"VSMain", c"vs_5_0", compile_flags, 0)
            .unwrap();
    let pixel_shader =
        Blob::compile_from_file(&shaders_hlsl_path, c"PSMain", c"ps_5_0", compile_flags, 0)
            .unwrap();

    let input_element_descs: [InputElementDesc; 2] = [
        InputElementDesc {
            semantic_name: c"POSITION",
            semantic_index: 0,
            format: Format::Rgb32Float,
            input_slot: 0,
            offset: 0,
            slot_class: InputSlotClass::PerVertex,
            instance_data_step_rate: 0,
        },
        InputElementDesc {
            semantic_name: c"COLOR",
            semantic_index: 0,
            format: Format::Rgba32Float,
            input_slot: 0,
            offset: 12,
            slot_class: InputSlotClass::PerVertex,
            instance_data_step_rate: 0,
        },
    ];

    let desc = GraphicsPipelineDesc {
        root_signature,
        input_layout: &input_element_descs,
        vs: &vertex_shader,
        ps: Some(&pixel_shader),
        ds: None,
        hs: None,
        gs: None,
        stream_output: None,
        sample_mask: u32::MAX,
        rasterizer_state: RasterizerDesc {
            fill_mode: FillMode::Solid,
            cull_mode: CullMode::None,
        },
        blend_state: BlendDesc {
            alpha_to_coverage_enable: false,
            independent_blend_enable: false,
            render_targets: smallvec![RenderTargetBlendDesc {
                blend_enable: false,
                logic_op_enable: false,
                src_blend: Blend::One,
                dst_blend: Blend::Zero,
                blend_op: BlendOp::Add,
                src_blend_alpha: Blend::One,
                dst_blend_alpha: Blend::Zero,
                blend_op_alpha: BlendOp::Add,
                logic_op: LogicOp::Noop,
                mask: BlendMask::all(),
            }],
        },
        depth_stencil: None,
        primitive_topology: PrimitiveTopology::Triangle,
        sampler_desc: SampleDesc {
            count: 1,
            ..Default::default()
        },
        ib_strip_cut_value: None,
        rtv_formats: smallvec![Format::Bgra8Unorm],
        dsv_format: None,
        node_mask: 0,
        cached_pso: None,
        flags: PipelineFlags::empty(),
    };

    device.create_graphics_pipeline(&desc).unwrap()
}

fn create_vertex_buffer(device: &Device, aspect_ratio: f32) -> (Resource, VertexBufferView) {
    let vertices = [
        Vertex {
            position: [0.0, 0.25 * aspect_ratio, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.25, -0.25 * aspect_ratio, 0.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.25, -0.25 * aspect_ratio, 0.0],
            color: [0.0, 0.0, 1.0, 1.0],
        },
    ];

    let vertex_buffer: Resource = device
        .create_committed_resource(
            HeapProperties {
                r#type: HeapType::Upload,
                cpu_page_propery: CpuPageProperty::Unknown,
                memory_pool_preference: MemoryPool::Unknown,
                creation_node_mask: 0,
                visible_node_mask: 0,
            },
            HeapFlags::empty(),
            ResourceDesc {
                dimension: ResourceDimension::Buffer,
                alignment: 0,
                width: std::mem::size_of_val(&vertices) as u64,
                height: 1,
                depth_or_array_size: 1,
                mip_levels: 1,
                sample_desc: SampleDesc {
                    count: 1,
                    quality: 0,
                },
                format: Format::Less,
                layout: TextureLayout::RowMajor,
                flags: ResourceFlags::empty(),
            },
            ResourceState::GenericRead,
            None,
        )
        .unwrap();

    unsafe {
        let data = vertex_buffer.map(0, None).unwrap();
        std::ptr::copy_nonoverlapping(vertices.as_ptr(), data as *mut Vertex, vertices.len());
        vertex_buffer.unmap(0, None);
    }

    let vbv = VertexBufferView {
        buffer_location: vertex_buffer.get_gpu_virtual_address(),
        stride_in_bytes: std::mem::size_of::<Vertex>() as u32,
        size_in_bytes: std::mem::size_of_val(&vertices) as u32,
    };

    (vertex_buffer, vbv)
}

#[repr(C)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

fn wait_for_previous_frame(resources: &mut Resources) {
    let fence = resources.fence_value;

    resources
        .command_queue
        .signal(&resources.fence, fence)
        .ok()
        .unwrap();

    resources.fence_value += 1;

    if resources.fence.get_completed_value() < fence {
        resources
            .fence
            .set_event_on_completion(fence, resources.fence_event)
            .ok()
            .unwrap();

        resources.fence_event.wait(u32::MAX);
    }

    resources.frame_index = resources.swap_chain.get_current_back_buffer_index();
}

fn main() {
    run_sample::<Sample>()
}
