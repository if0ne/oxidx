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

    fn bind_to_window(&mut self, hwnd: isize);

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

    sample.bind_to_window(hwnd.get());

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
    //root_signature: ID3D12RootSignature,
    //pso: ID3D12PipelineState,
    command_list: GraphicsCommandList,

    // we need to keep this around to keep the reference alive, even though
    // nothing reads from it
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

    fn bind_to_window(&mut self, hwnd: isize) {
        let command_queue: CommandQueue = self
            .device
            .create_command_queue(CommandQueueDesc {
                r#type: CommandListType::Direct,
                priority: 1,
                flags: todo!(),
                node_mask: todo!(),
            })
            .unwrap();

        let (width, height) = self.window_size();

        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC1 {
            BufferCount: FRAME_COUNT,
            Width: width as u32,
            Height: height as u32,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        let swap_chain: IDXGISwapChain3 = unsafe {
            self.dxgi_factory.CreateSwapChainForHwnd(
                &command_queue,
                *hwnd,
                &swap_chain_desc,
                None,
                None,
            )?
        }
        .cast()?;

        unsafe {
            self.dxgi_factory
                .MakeWindowAssociation(*hwnd, DXGI_MWA_NO_ALT_ENTER)?;
        }

        let frame_index = unsafe { swap_chain.GetCurrentBackBufferIndex() };

        let rtv_heap: ID3D12DescriptorHeap = unsafe {
            self.device
                .CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                    NumDescriptors: FRAME_COUNT,
                    Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                    ..Default::default()
                })
        }?;

        let rtv_descriptor_size = unsafe {
            self.device
                .GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV)
        } as usize;
        let rtv_handle = unsafe { rtv_heap.GetCPUDescriptorHandleForHeapStart() };

        let render_targets: [ID3D12Resource; FRAME_COUNT as usize] =
            array_init::try_array_init(|i: usize| -> Result<ID3D12Resource> {
                let render_target: ID3D12Resource = unsafe { swap_chain.GetBuffer(i as u32) }?;
                unsafe {
                    self.device.CreateRenderTargetView(
                        &render_target,
                        None,
                        D3D12_CPU_DESCRIPTOR_HANDLE {
                            ptr: rtv_handle.ptr + i * rtv_descriptor_size,
                        },
                    )
                };
                Ok(render_target)
            })?;

        let viewport = D3D12_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: width as f32,
            Height: height as f32,
            MinDepth: D3D12_MIN_DEPTH,
            MaxDepth: D3D12_MAX_DEPTH,
        };

        let scissor_rect = RECT {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        };

        let command_allocator = unsafe {
            self.device
                .CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT)
        }?;

        let root_signature = create_root_signature(&self.device)?;
        let pso = create_pipeline_state(&self.device, &root_signature)?;

        let command_list: ID3D12GraphicsCommandList = unsafe {
            self.device.CreateCommandList(
                0,
                D3D12_COMMAND_LIST_TYPE_DIRECT,
                &command_allocator,
                &pso,
            )
        }?;
        unsafe {
            command_list.Close()?;
        };

        let aspect_ratio = width as f32 / height as f32;

        let (vertex_buffer, vbv) = create_vertex_buffer(&self.device, aspect_ratio)?;

        let fence = unsafe { self.device.CreateFence(0, D3D12_FENCE_FLAG_NONE) }?;

        let fence_value = 1;

        let fence_event = unsafe { CreateEventA(None, false, false, None)? };

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
            //root_signature,
            //pso,
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
            populate_command_list(resources).unwrap();

            // Execute the command list.
            let command_list = Some(resources.command_list.cast().unwrap());
            unsafe { resources.command_queue.ExecuteCommandLists(&[command_list]) };

            // Present the frame.
            unsafe { resources.swap_chain.Present(1, 0) }.ok().unwrap();

            wait_for_previous_frame(resources);
        }
    }
}

fn create_device(command_line: &SampleCommandLine) -> (Factory4, Device) {
    if cfg!(debug_assertions) {
        unsafe {
            let mut debug: Option<Debug> = None;
            if let Some(debug) = D3D12GetDebugInterface(&mut debug).ok().and(debug) {
                debug.EnableDebugLayer();
            }
        }
    }

    let dxgi_factory_flags = if cfg!(debug_assertions) {
        DXGI_CREATE_FACTORY_DEBUG
    } else {
        0
    };

    let dxgi_factory: IDXGIFactory4 = unsafe { CreateDXGIFactory2(dxgi_factory_flags) }?;

    let adapter = if command_line.use_warp_device {
        unsafe { dxgi_factory.EnumWarpAdapter() }
    } else {
        get_hardware_adapter(&dxgi_factory)
    }?;

    let mut device: Option<ID3D12Device> = None;
    unsafe { D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device) }?;
    Ok((dxgi_factory, device.unwrap()))
}

fn create_root_signature(device: &ID3D12Device) -> Result<ID3D12RootSignature> {
    let desc = D3D12_ROOT_SIGNATURE_DESC {
        Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
        ..Default::default()
    };

    let mut signature = None;

    let signature = unsafe {
        D3D12SerializeRootSignature(&desc, D3D_ROOT_SIGNATURE_VERSION_1, &mut signature, None)
    }
    .map(|()| signature.unwrap())?;

    unsafe {
        device.CreateRootSignature(
            0,
            std::slice::from_raw_parts(
                signature.GetBufferPointer() as _,
                signature.GetBufferSize(),
            ),
        )
    }
}

fn create_pipeline_state(
    device: &ID3D12Device,
    root_signature: &ID3D12RootSignature,
) -> Result<ID3D12PipelineState> {
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
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            ..Default::default()
        },
        ..Default::default()
    };
    desc.RTVFormats[0] = DXGI_FORMAT_R8G8B8A8_UNORM;

    unsafe { device.CreateGraphicsPipelineState(&desc) }
}

fn create_vertex_buffer(
    device: &ID3D12Device,
    aspect_ratio: f32,
) -> Result<(ID3D12Resource, D3D12_VERTEX_BUFFER_VIEW)> {
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

    Ok((vertex_buffer, vbv))
}

#[repr(C)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

fn wait_for_previous_frame(resources: &mut Resources) {
    let fence = resources.fence_value;

    unsafe { resources.command_queue.Signal(&resources.fence, fence) }
        .ok()
        .unwrap();

    resources.fence_value += 1;

    if unsafe { resources.fence.GetCompletedValue() } < fence {
        unsafe {
            resources
                .fence
                .SetEventOnCompletion(fence, resources.fence_event)
        }
        .ok()
        .unwrap();

        unsafe { WaitForSingleObject(resources.fence_event, INFINITE) };
    }

    resources.frame_index = unsafe { resources.swap_chain.GetCurrentBackBufferIndex() };
}

fn main() {
    run_sample::<Sample>()
}
