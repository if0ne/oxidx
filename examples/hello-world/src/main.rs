use std::num::NonZeroIsize;

use oxidx::prelude::*;

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

    //;vbv: D3D12_VERTEX_BUFFER_VIEW,
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
            .create_swapchain_for_hwnd(&command_queue, hwnd, &swap_chain_desc, None, None)
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

        let (vertex_buffer, vbv) = create_vertex_buffer(&self.device, aspect_ratio)?;

        let fence = self.device.create_fence(0, FenceFlags::empty()).unwrap();

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
            //vbv,
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
            resources
                .command_queue
                .execute_command_lists([command_list].into_iter());

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
    resources.command_allocator.reset();

    let command_list = &resources.command_list;

    command_list.Reset(&resources.command_allocator, &resources.pso)?;

    command_list.SetGraphicsRootSignature(&resources.root_signature);
    command_list.RSSetViewports(&[resources.viewport]);
    command_list.RSSetScissorRects(&[resources.scissor_rect]);

    let barrier = transition_barrier(
        &resources.render_targets[resources.frame_index as usize],
        D3D12_RESOURCE_STATE_PRESENT,
        D3D12_RESOURCE_STATE_RENDER_TARGET,
    );
    command_list.ResourceBarrier(&[barrier]);

    let rtv_handle = resources
        .rtv_heap
        .get_cpu_descriptor_handle_for_heap_start()
        .offset(resources.frame_index as usize * resources.rtv_descriptor_size);

    command_list.OMSetRenderTargets(1, Some(&rtv_handle), false, None);

    command_list.ClearRenderTargetView(rtv_handle, &[0.0_f32, 0.2_f32, 0.4_f32, 1.0_f32], None);
    command_list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
    command_list.IASetVertexBuffers(0, Some(&[resources.vbv]));
    command_list.DrawInstanced(3, 1, 0, 0);

    // Indicate that the back buffer will now be used to present.
    command_list.ResourceBarrier(&[transition_barrier(
        &resources.render_targets[resources.frame_index as usize],
        D3D12_RESOURCE_STATE_RENDER_TARGET,
        D3D12_RESOURCE_STATE_PRESENT,
    )]);

    command_list.close();
}

fn transition_barrier(
    resource: &Resource,
    state_before: D3D12_RESOURCE_STATES,
    state_after: D3D12_RESOURCE_STATES,
) -> D3D12_RESOURCE_BARRIER {
    D3D12_RESOURCE_BARRIER {
        Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
        Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
        Anonymous: D3D12_RESOURCE_BARRIER_0 {
            Transition: std::mem::ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                pResource: unsafe { std::mem::transmute_copy(resource) },
                StateBefore: state_before,
                StateAfter: state_after,
                Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
            }),
        },
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

        if (desc.flags & AdapterFlags::Sofware) != AdapterFlags::None {
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
        .create_root_signature(&desc, RootSignatureVersion::V1_0, 0)
        .unwrap()
}

fn create_pipeline_state(device: &Device, root_signature: &RootSignature) -> PipelineState {
    let compile_flags = if cfg!(debug_assertions) {
        D3DCOMPILE_DEBUG | D3DCOMPILE_SKIP_OPTIMIZATION
    } else {
        0
    };

    let exe_path = std::env::current_exe().ok().unwrap();
    let asset_path = exe_path.parent().unwrap();
    let shaders_hlsl_path = asset_path.join("shaders.hlsl");
    let shaders_hlsl = shaders_hlsl_path.to_str().unwrap();
    let shaders_hlsl: HSTRING = shaders_hlsl.into();

    let mut vertex_shader = None;
    let vertex_shader = unsafe {
        D3DCompileFromFile(
            &shaders_hlsl,
            None,
            None,
            s!("VSMain"),
            s!("vs_5_0"),
            compile_flags,
            0,
            &mut vertex_shader,
            None,
        )
    }
    .map(|()| vertex_shader.unwrap())?;

    let mut pixel_shader = None;
    let pixel_shader = unsafe {
        D3DCompileFromFile(
            &shaders_hlsl,
            None,
            None,
            s!("PSMain"),
            s!("ps_5_0"),
            compile_flags,
            0,
            &mut pixel_shader,
            None,
        )
    }
    .map(|()| pixel_shader.unwrap())?;

    let mut input_element_descs: [D3D12_INPUT_ELEMENT_DESC; 2] = [
        D3D12_INPUT_ELEMENT_DESC {
            SemanticName: s!("POSITION"),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
        D3D12_INPUT_ELEMENT_DESC {
            SemanticName: s!("COLOR"),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 12,
            InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
    ];

    let mut desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
        InputLayout: D3D12_INPUT_LAYOUT_DESC {
            pInputElementDescs: input_element_descs.as_mut_ptr(),
            NumElements: input_element_descs.len() as u32,
        },
        pRootSignature: unsafe { std::mem::transmute_copy(root_signature) },
        VS: D3D12_SHADER_BYTECODE {
            pShaderBytecode: unsafe { vertex_shader.GetBufferPointer() },
            BytecodeLength: unsafe { vertex_shader.GetBufferSize() },
        },
        PS: D3D12_SHADER_BYTECODE {
            pShaderBytecode: unsafe { pixel_shader.GetBufferPointer() },
            BytecodeLength: unsafe { pixel_shader.GetBufferSize() },
        },
        RasterizerState: D3D12_RASTERIZER_DESC {
            FillMode: D3D12_FILL_MODE_SOLID,
            CullMode: D3D12_CULL_MODE_NONE,
            ..Default::default()
        },
        BlendState: D3D12_BLEND_DESC {
            AlphaToCoverageEnable: false.into(),
            IndependentBlendEnable: false.into(),
            RenderTarget: [
                D3D12_RENDER_TARGET_BLEND_DESC {
                    BlendEnable: false.into(),
                    LogicOpEnable: false.into(),
                    SrcBlend: D3D12_BLEND_ONE,
                    DestBlend: D3D12_BLEND_ZERO,
                    BlendOp: D3D12_BLEND_OP_ADD,
                    SrcBlendAlpha: D3D12_BLEND_ONE,
                    DestBlendAlpha: D3D12_BLEND_ZERO,
                    BlendOpAlpha: D3D12_BLEND_OP_ADD,
                    LogicOp: D3D12_LOGIC_OP_NOOP,
                    RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8,
                },
                D3D12_RENDER_TARGET_BLEND_DESC::default(),
                D3D12_RENDER_TARGET_BLEND_DESC::default(),
                D3D12_RENDER_TARGET_BLEND_DESC::default(),
                D3D12_RENDER_TARGET_BLEND_DESC::default(),
                D3D12_RENDER_TARGET_BLEND_DESC::default(),
                D3D12_RENDER_TARGET_BLEND_DESC::default(),
                D3D12_RENDER_TARGET_BLEND_DESC::default(),
            ],
        },
        DepthStencilState: D3D12_DEPTH_STENCIL_DESC::default(),
        SampleMask: u32::MAX,
        PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
        NumRenderTargets: 1,
        SampleDesc: SampleDesc {
            Count: 1,
            ..Default::default()
        },
        ..Default::default()
    };
    desc.RTVFormats[0] = DXGI_FORMAT_R8G8B8A8_UNORM;

    unsafe { device.CreateGraphicsPipelineState(&desc) }
}

fn create_vertex_buffer(
    device: &Device,
    aspect_ratio: f32,
) -> (Resource, D3D12_VERTEX_BUFFER_VIEW) {
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

    let mut vertex_buffer: Option<ID3D12Resource> = None;
    unsafe {
        device.CreateCommittedResource(
            &D3D12_HEAP_PROPERTIES {
                Type: D3D12_HEAP_TYPE_UPLOAD,
                ..Default::default()
            },
            D3D12_HEAP_FLAG_NONE,
            &D3D12_RESOURCE_DESC {
                Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
                Width: std::mem::size_of_val(&vertices) as u64,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
                ..Default::default()
            },
            D3D12_RESOURCE_STATE_GENERIC_READ,
            None,
            &mut vertex_buffer,
        )?
    };
    let vertex_buffer = vertex_buffer.unwrap();

    unsafe {
        let mut data = std::ptr::null_mut();
        vertex_buffer.Map(0, None, Some(&mut data))?;
        std::ptr::copy_nonoverlapping(vertices.as_ptr(), data as *mut Vertex, vertices.len());
        vertex_buffer.Unmap(0, None);
    }

    let vbv = D3D12_VERTEX_BUFFER_VIEW {
        BufferLocation: unsafe { vertex_buffer.GetGPUVirtualAddress() },
        StrideInBytes: std::mem::size_of::<Vertex>() as u32,
        SizeInBytes: std::mem::size_of_val(&vertices) as u32,
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

    if resources.fence.get_value() < fence {
        resources
            .fence
            .set_event_on_completion(resources.fence_event, u64::MAX)
            .ok()
            .unwrap();

        resources.fence_event.wait(u32::MAX);
    }

    resources.frame_index = resources.swap_chain.get_current_back_buffer_index();
}

fn main() {
    run_sample::<Sample>()
}
