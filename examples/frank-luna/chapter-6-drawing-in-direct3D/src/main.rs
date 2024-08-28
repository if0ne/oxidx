use std::collections::HashMap;

use common::{
    app::{DxSample, SwapchainContext},
    geometry_mesh::{BoundingBox, MeshGeometry, SubmeshGeometry},
    run_sample,
    upload_buffer::UploadBuffer,
    utils::{create_default_buffer, ConstantBufferData, VertexAttr},
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

#[allow(unused)]
#[derive(Debug)]
pub struct BoxSample {
    root_signature: RootSignature,
    cbv_heap: DescriptorHeap,
    object_cb: UploadBuffer<ConstantBufferData<ObjectConstants>>,
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

    is_lmb_pressed: bool,
    is_rmb_pressed: bool,
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

        let object_cb = UploadBuffer::new(&base.device, 1);
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

        let vs_byte_code = Blob::compile_from_file(
            "shader.hlsl",
            &[],
            c"VS",
            c"vs_5_0",
            PACK_MATRIX_ROW_MAJOR,
            0,
        )
        .unwrap();
        let ps_byte_code = Blob::compile_from_file(
            "shader.hlsl",
            &[],
            c"PS",
            c"ps_5_0",
            PACK_MATRIX_ROW_MAJOR,
            0,
        )
        .unwrap();

        let box_geo = Self::build_box_geometry(&base.device, &base.cmd_list);

        let input_layout = Vertex::get_input_layout();

        let pso_desc = GraphicsPipelineDesc::new(&vs_byte_code)
            .with_ps(&ps_byte_code)
            .with_input_layout(&input_layout)
            .with_root_signature(&root_signature)
            .with_rasterizer_state(RasterizerDesc::default())
            .with_blend_desc(BlendDesc::default())
            .with_depth_stencil(DepthStencilDesc::default(), base.depth_stencil_format)
            .with_sample_mask(u32::MAX)
            .with_primitive_topology(PipelinePrimitiveTopology::Triangle)
            .with_render_targets([base.back_buffer_format])
            .with_sample_desc(if base.msaa_state {
                SampleDesc::new(4, base.msaa_4x_quality)
            } else {
                SampleDesc::new(1, 0)
            });

        let pso = base.device.create_graphics_pipeline(&pso_desc).unwrap();

        base.cmd_list.close().unwrap();
        base.cmd_queue
            .execute_command_lists(&[Some(base.cmd_list.clone())]);
        base.flush_command_queue();

        Self {
            root_signature,
            cbv_heap,
            object_cb,
            box_geo,
            vs_byte_code,
            ps_byte_code,
            pso,
            world: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            proj: Mat4::IDENTITY,
            theta: 1.5 * std::f32::consts::PI,
            phi: std::f32::consts::FRAC_PI_4,
            radius: 5.0,
            is_lmb_pressed: false,
            is_rmb_pressed: false,
        }
    }

    fn init_resources(&mut self, _: &common::app::Base) {}

    fn update(&mut self, _: &common::app::Base) {
        let x = self.radius * self.phi.sin() * self.theta.cos();
        let y = self.radius * self.phi.sin() * self.theta.sin();
        let z = self.radius * self.phi.cos();

        let pos = vec3(x, y, z);
        let target = Vec3::ZERO;
        let up = Vec3::Y;

        self.view = Mat4::look_at_lh(pos, target, up);

        let data = ConstantBufferData(ObjectConstants {
            world_view_proj: self.proj * self.view * self.world,
        });

        self.object_cb.copy_data(0, data);
    }

    fn render(&mut self, base: &mut common::app::Base) {
        let Some(ref context) = base.context else {
            return;
        };

        base.cmd_list_alloc.reset().unwrap();

        base.cmd_list
            .reset(&base.cmd_list_alloc, Some(&self.pso))
            .unwrap();

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
            .set_descriptor_heaps(&[Some(self.cbv_heap.clone())]);

        base.cmd_list
            .set_graphics_root_signature(Some(&self.root_signature));
        base.cmd_list
            .ia_set_vertex_buffers(0, &[self.box_geo.vertex_buffer_view()]);
        base.cmd_list
            .ia_set_index_buffer(Some(&self.box_geo.index_buffer_view()));
        base.cmd_list
            .ia_set_primitive_topology(PrimitiveTopology::Triangle);
        base.cmd_list.set_graphics_root_descriptor_table(
            0,
            self.cbv_heap.get_gpu_descriptor_handle_for_heap_start(),
        );

        base.cmd_list.draw_indexed_instanced(
            self.box_geo.draw_args.get("box").unwrap().index_count,
            1,
            0,
            0,
            0,
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

    fn on_resize(&mut self, base: &mut common::app::Base, _: u32, _: u32) {
        self.proj = Mat4::perspective_lh(
            0.25 * std::f32::consts::PI,
            base.aspect_ratio(),
            1.0,
            1000.0,
        );
    }

    fn on_key_down(&mut self, _: winit::keyboard::KeyCode, _: bool) {}

    fn on_key_up(&mut self, _: winit::keyboard::KeyCode) {}

    fn on_mouse_down(&mut self, mouse: winit::event::MouseButton) {
        match mouse {
            winit::event::MouseButton::Left => self.is_lmb_pressed = true,
            winit::event::MouseButton::Right => self.is_rmb_pressed = true,
            _ => {}
        }
    }

    fn on_mouse_up(&mut self, mouse: winit::event::MouseButton) {
        match mouse {
            winit::event::MouseButton::Left => self.is_lmb_pressed = false,
            winit::event::MouseButton::Right => self.is_rmb_pressed = false,
            _ => {}
        }
    }

    fn on_mouse_move(&mut self, x: f64, y: f64) {
        let x = x as f32;
        let y = y as f32;

        if self.is_lmb_pressed {
            let dx = (0.25 * x).to_radians();
            let dy = (0.25 * y).to_radians();

            self.theta += dx;
            self.phi = (self.phi + dy).clamp(0.01, std::f32::consts::PI - 0.1);
        } else if self.is_rmb_pressed {
            let dx = 0.005 * x;
            let dy = 0.005 * y;

            self.radius = (self.radius + dx - dy).clamp(3.0, 15.0);
        }
    }
}

impl BoxSample {
    fn build_box_geometry(device: &Device, cmd_list: &GraphicsCommandList) -> MeshGeometry {
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
            },
        ];

        let indices = [
            // front face
            0u16, 1, 2, 0, 2, 3, // back face
            4, 6, 5, 4, 7, 6, // left face
            4, 5, 1, 4, 1, 0, // right face
            3, 2, 6, 3, 6, 7, // top face
            1, 5, 6, 1, 6, 2, // bottom face
            4, 0, 3, 4, 3, 7,
        ];

        let vb_byte_size = size_of_val(&vertices);
        let ib_byte_size = size_of_val(&indices);

        let vertex_buffer_cpu = Blob::create_blob(vb_byte_size).unwrap();
        let index_buffer_cpu = Blob::create_blob(ib_byte_size).unwrap();

        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices.as_ptr(),
                vertex_buffer_cpu.get_buffer_ptr::<Vertex>().as_mut(),
                vertices.len(),
            );
            std::ptr::copy_nonoverlapping(
                indices.as_ptr(),
                index_buffer_cpu.get_buffer_ptr::<u16>().as_mut(),
                indices.len(),
            );
        }

        let (vertex_buffer_gpu, vertex_buffer_uploader) =
            create_default_buffer(device, cmd_list, &vertices);
        let (index_buffer_gpu, index_buffer_uploader) =
            create_default_buffer(device, cmd_list, &indices);

        MeshGeometry {
            name: "boxGeo".to_string(),
            vertex_buffer_cpu,
            index_buffer_cpu,
            vertex_buffer_gpu,
            index_buffer_gpu,
            vertex_buffer_uploader: Some(vertex_buffer_uploader),
            index_buffer_uploader: Some(index_buffer_uploader),
            vertex_byte_stride: size_of::<Vertex>() as u32,
            vertex_buffer_byte_size: vb_byte_size as u32,
            index_format: Format::R16Uint,
            index_buffer_byte_size: ib_byte_size as u32,
            draw_args: HashMap::from_iter([(
                "box".to_string(),
                SubmeshGeometry {
                    index_count: indices.len() as u32,
                    start_index_location: 0,
                    base_vertex_location: 0,
                    bounds: BoundingBox {
                        min: vec3(-1.0, -1.0, -1.0),
                        max: vec3(1.0, 1.0, 1.0),
                    },
                },
            )]),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub color: Vec4,
}

impl VertexAttr<2> for Vertex {
    fn get_input_layout() -> [InputElementDesc; 2] {
        [
            InputElementDesc::per_vertex(SemanticName::Position(0), Format::Rgb32Float, 0),
            InputElementDesc::per_vertex(SemanticName::Color(0), Format::Rgba32Float, 0)
                .with_offset(16),
        ]
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ObjectConstants {
    pub world_view_proj: Mat4,
}
