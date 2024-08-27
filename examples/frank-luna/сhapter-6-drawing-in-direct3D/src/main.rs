use common::{
    app::{DxSample, SwapchainContext},
    geometry_mesh::MeshGeometry,
    run_sample,
    upload_buffer::UploadBuffer,
    utils::{ConstantBufferData, VertexAttr},
};
use glam::{vec3, vec4, Mat4, Vec3, Vec4};
use oxidx::dx::*;
use tracing_subscriber::layer::SubscriberExt;

fn main() {
    let console_log = tracing_subscriber::fmt::Layer::new()
        .with_ansi(true)
        .with_writer(std::io::stdout);

    let subscriber = tracing_subscriber::registry().with(console_log);

    let _ = tracing::subscriber::set_global_default(subscriber);
    run_sample::<BoxSample>();
}

#[derive(Debug)]
pub struct BoxSample {
    root_signature: RootSignature,
    cbv_heap: DescriptorHeap,
    object_cb: UploadBuffer<true, ObjectConstants>,
    box_geo: MeshGeometry,

    vs_byte_code: Blob,
    ps_byte_code: Blob,

    pso: PipelineState,
    world: Mat4,
    view: Mat4,
    proj: Mat4,

    theta: f32,
    phi: f32,
    radius: f32,

    last_moust_pos: (f32, f32),
}

impl DxSample for BoxSample {
    fn new(base: &mut common::app::Base) -> Self {
        base.cmd_list.reset(&base.cmd_list_alloc, PSO_NONE).unwrap();

        let cbv_heap: DescriptorHeap = base
            .device
            .create_descriptor_heap(
                &DescriptorHeapDesc::cbr_srv_uav(1).with_flags(DescriptorHeapFlags::ShaderVisible),
            )
            .unwrap();

        let object_cb = UploadBuffer::<true, ObjectConstants>::new(&base.device, 1);
        let object_size = size_of::<ConstantBufferData<ObjectConstants>>() as u64;

        let mut address = object_cb.resource().get_gpu_virtual_address();
        let box_index = 0;
        address += box_index * object_size;

        base.device.create_constant_buffer_view(
            Some(&ConstantBufferViewDesc::new(address, object_size as u32)),
            cbv_heap.get_cpu_descriptor_handle_for_heap_start(),
        );

        let cbv_table = [DescriptorRange::cbv(1)];

        let root_parameter = [RootParameter::descriptor_table(&cbv_table)];

        let root_signature_desc = RootSignatureDesc::default()
            .with_parameters(&root_parameter)
            .with_flags(RootSignatureFlags::AllowInputAssemblerInputLayout);

        let root_signature = base
            .device
            .serialize_and_create_root_signature(
                &root_signature_desc,
                RootSignatureVersion::V1_0,
                0,
            )
            .unwrap();

        let vs_byte_code =
            Blob::compile_from_file("../assets/shader.hlsl", &[], c"VS", c"vs_5_0", 0, 0).unwrap();
        let ps_byte_code =
            Blob::compile_from_file("../assets/shader.hlsl", &[], c"PS", c"ps_5_0", 0, 0).unwrap();

        let box_geo = Self::build_box_geometry();

        base.cmd_list.close().unwrap();
        base.cmd_queue
            .execute_command_lists(&[Some(base.cmd_list.clone())]);
        base.flush_command_queue();

        Self {
            root_signature,
            cbv_heap,
            object_cb,
            box_geo: todo!(),
            vs_byte_code,
            ps_byte_code,
            pso: todo!(),
            world: todo!(),
            view: todo!(),
            proj: todo!(),
            theta: todo!(),
            phi: todo!(),
            radius: todo!(),
            last_moust_pos: todo!(),
        }
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
            [204.0 / 255.0, 102.0 / 255.0, 102.0 / 255.0, 1.0],
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

impl BoxSample {
    fn build_box_geometry() -> MeshGeometry {
        let vertices = [
            Vertex {
                pos: vec3(-1.0, -1.0, -1.0),
                color: vec4(0.0, 0.0, 0.0, 1.0),
            },
            Vertex {
                pos: vec3(-1.0, 1.0, -1.0),
                color: vec4(1.0, 1.0, 1.0, 1.0),
            },
            Vertex {
                pos: vec3(1.0, 1.0, -1.0),
                color: vec4(1.0, 0.0, 0.0, 1.0),
            },
            Vertex {
                pos: vec3(1.0, -1.0, -1.0),
                color: vec4(0.0, 1.0, 0.0, 1.0),
            },
            Vertex {
                pos: vec3(-1.0, -1.0, 1.0),
                color: vec4(0.0, 0.0, 1.0, 1.0),
            },
            Vertex {
                pos: vec3(-1.0, 1.0, 1.0),
                color: vec4(0.5, 0.32, 0.0, 1.0),
            },
            Vertex {
                pos: vec3(1.0, 1.0, 1.0),
                color: vec4(0.32, 0.0, 0.67, 1.0),
            },
            Vertex {
                pos: vec3(1.0, -1.0, 1.0),
                color: vec4(0.0, 0.67, 0.32, 1.0),
            }
        ];

        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub color: Vec4,
}

impl VertexAttr for Vertex {
    fn get_input_layout() -> impl Iterator<Item = InputElementDesc> {
        [
            InputElementDesc::per_vertex(SemanticName::Position(0), Format::Rgb32Float, 0),
            InputElementDesc::per_vertex(SemanticName::Color(0), Format::Rgba32Float, 0),
        ]
        .into_iter()
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ObjectConstants {
    pub world_view_proj: Mat4,
}
