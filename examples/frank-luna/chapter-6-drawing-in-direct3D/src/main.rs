use std::collections::HashMap;

use common::{
    app::{DxSample, SwapchainContext},
    geometry_mesh::{BoundingBox, SubmeshGeometry},
    run_sample,
    upload_buffer::UploadBuffer,
    utils::{create_default_buffer, ConstantBufferData},
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
    box_geo: MeshGeometrySplitted,

    vs_byte_code: Blob,
    ps_byte_code: Blob,

    pso: PipelineState,
    world_pyramid: Mat4,
    world_box: Mat4,
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
                &DescriptorHeapDesc::cbr_srv_uav(2).with_flags(DescriptorHeapFlags::ShaderVisible),
            )
            .unwrap();

        let object_cb = UploadBuffer::new(&base.device, 2);
        let object_size = size_of::<ConstantBufferData<ObjectConstants>>() as u64;

        let address = object_cb.resource().get_gpu_virtual_address();

        base.device.create_constant_buffer_view(
            Some(&ConstantBufferViewDesc::new(address, object_size as usize)),
            cbv_heap.get_cpu_descriptor_handle_for_heap_start(),
        );

        base.device.create_constant_buffer_view(
            Some(&ConstantBufferViewDesc::new(
                address + object_size,
                object_size as usize,
            )),
            cbv_heap
                .get_cpu_descriptor_handle_for_heap_start()
                .advance(1, base.cbv_srv_uav_descriptor_size),
        );

        let cbv_table = [DescriptorRange::cbv(1, 0)];

        let root_parameter = [
            RootParameter::descriptor_table(&cbv_table),
            RootParameter::constant_32bit(1, 0, 1),
        ];

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
            c"vs_5_1",
            PACK_MATRIX_ROW_MAJOR,
            0,
        )
        .unwrap();
        let ps_byte_code = Blob::compile_from_file(
            "shader.hlsl",
            &[],
            c"PS",
            c"ps_5_1",
            PACK_MATRIX_ROW_MAJOR,
            0,
        )
        .unwrap();

        let box_geo = Self::build_box_pyramid_geometry(&base.device, &base.cmd_list);

        let input_layout = [
            InputElementDesc::per_vertex(SemanticName::Position(0), Format::Rgb32Float, 0),
            InputElementDesc::per_vertex(SemanticName::Color(0), Format::Rgba32Float, 1),
        ];

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
            })
            .with_depth_stencil(
                DepthStencilDesc::default().enable_depth(ComparisonFunc::Less),
                base.depth_stencil_format,
            );

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
            world_box: Mat4::IDENTITY,
            world_pyramid: Mat4::from_translation(vec3(2.0, 0.0, 0.0)),
            view: Mat4::IDENTITY,
            proj: Mat4::IDENTITY,
            theta: 0.0,
            phi: 0.0,
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
            world_view_proj: self.proj * self.view * self.world_box,
        });

        self.object_cb.copy_data(0, data);

        let data = ConstantBufferData(ObjectConstants {
            world_view_proj: self.proj * self.view * self.world_pyramid,
        });

        self.object_cb.copy_data(1, data);
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
                None,
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
        base.cmd_list.ia_set_vertex_buffers(
            0,
            &[
                self.box_geo.vertex_buffer_position_view(),
                self.box_geo.vertex_buffer_color_view(),
            ],
        );
        base.cmd_list
            .ia_set_index_buffer(Some(&self.box_geo.index_buffer_view()));
        base.cmd_list
            .ia_set_primitive_topology(PrimitiveTopology::Triangle);

        base.cmd_list
            .set_graphics_root_32bit_constant(1, base.timer.total_time(), 0);

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

        base.cmd_list.set_graphics_root_descriptor_table(
            0,
            self.cbv_heap
                .get_gpu_descriptor_handle_for_heap_start()
                .advance(1, base.cbv_srv_uav_descriptor_size),
        );

        base.cmd_list.draw_indexed_instanced(
            self.box_geo.draw_args.get("pyramid").unwrap().index_count,
            1,
            self.box_geo
                .draw_args
                .get("pyramid")
                .unwrap()
                .start_index_location,
            self.box_geo
                .draw_args
                .get("pyramid")
                .unwrap()
                .base_vertex_location as i32,
            0,
        );

        base.cmd_list
            .resource_barrier(&[ResourceBarrier::transition(
                context.current_back_buffer(),
                ResourceStates::RenderTarget,
                ResourceStates::Present,
                None,
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

    fn on_key_down(&mut self, _: &common::app::Base, _: winit::keyboard::KeyCode, _: bool) {}

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
            let dy = -0.005 * y;
            self.radius = (self.radius + dx - dy).clamp(3.0, 15.0);
        }
    }
}

impl BoxSample {
    fn build_box_pyramid_geometry(
        device: &Device,
        cmd_list: &GraphicsCommandList,
    ) -> MeshGeometrySplitted {
        let position = [
            // Box
            VertexPos {
                pos: vec3(-1.0, -1.0, -1.0),
            },
            VertexPos {
                pos: vec3(-1.0, 1.0, -1.0),
            },
            VertexPos {
                pos: vec3(1.0, 1.0, -1.0),
            },
            VertexPos {
                pos: vec3(1.0, -1.0, -1.0),
            },
            VertexPos {
                pos: vec3(-1.0, -1.0, 1.0),
            },
            VertexPos {
                pos: vec3(-1.0, 1.0, 1.0),
            },
            VertexPos {
                pos: vec3(1.0, 1.0, 1.0),
            },
            VertexPos {
                pos: vec3(1.0, -1.0, 1.0),
            },
            // Pyramid
            VertexPos {
                pos: vec3(0.0, 1.0, 0.0),
            },
            VertexPos {
                pos: vec3(-1.0, -1.0, -1.0),
            },
            VertexPos {
                pos: vec3(-1.0, -1.0, 1.0),
            },
            VertexPos {
                pos: vec3(1.0, -1.0, 1.0),
            },
            VertexPos {
                pos: vec3(1.0, -1.0, -1.0),
            },
        ];

        let colors = [
            VertexColor {
                color: vec4(0.0, 0.0, 0.0, 1.0),
            },
            VertexColor {
                color: vec4(1.0, 1.0, 1.0, 1.0),
            },
            VertexColor {
                color: vec4(1.0, 0.0, 0.0, 1.0),
            },
            VertexColor {
                color: vec4(0.0, 1.0, 0.0, 1.0),
            },
            VertexColor {
                color: vec4(0.0, 0.0, 1.0, 1.0),
            },
            VertexColor {
                color: vec4(0.5, 0.32, 0.0, 1.0),
            },
            VertexColor {
                color: vec4(0.32, 0.0, 0.67, 1.0),
            },
            VertexColor {
                color: vec4(0.0, 0.67, 0.32, 1.0),
            },
            VertexColor {
                color: vec4(0.0, 1.0, 0.0, 1.0),
            },
            VertexColor {
                color: vec4(0.0, 0.0, 1.0, 1.0),
            },
            VertexColor {
                color: vec4(0.5, 0.32, 0.0, 1.0),
            },
            VertexColor {
                color: vec4(0.32, 0.0, 0.67, 1.0),
            },
            VertexColor {
                color: vec4(0.0, 0.67, 0.32, 1.0),
            },
        ];

        let indices = [
            0u16, 1, 2, 0, 2, 3, // front face
            4, 6, 5, 4, 7, 6, // back face
            4, 5, 1, 4, 1, 0, // left face
            3, 2, 6, 3, 6, 7, // right face
            1, 5, 6, 1, 6, 2, // top face
            4, 0, 3, 4, 3, 7, // bottom face
            0, 4, 1, 0, 1, 2, //
            0, 2, 3, 0, 3, 4, //
            1, 4, 2, 2, 4, 3, //
        ];

        let vb_pos_byte_size = size_of_val(&position);
        let vb_color_byte_size = size_of_val(&colors);
        let ib_byte_size = size_of_val(&indices);

        let vertex_buffer_pos_cpu = Blob::create_blob(vb_color_byte_size).unwrap();
        let vertex_buffer_color_cpu = Blob::create_blob(vb_color_byte_size).unwrap();
        let index_buffer_cpu = Blob::create_blob(ib_byte_size).unwrap();

        unsafe {
            std::ptr::copy_nonoverlapping(
                position.as_ptr(),
                vertex_buffer_pos_cpu.get_buffer_ptr::<VertexPos>().as_mut(),
                position.len(),
            );
            std::ptr::copy_nonoverlapping(
                colors.as_ptr(),
                vertex_buffer_color_cpu
                    .get_buffer_ptr::<VertexColor>()
                    .as_mut(),
                colors.len(),
            );
            std::ptr::copy_nonoverlapping(
                indices.as_ptr(),
                index_buffer_cpu.get_buffer_ptr::<u16>().as_mut(),
                indices.len(),
            );
        }

        let (vertex_buffer_pos_gpu, vertex_pos_buffer_uploader) =
            create_default_buffer(device, cmd_list, &position);
        let (vertex_buffer_color_gpu, vertex_buffer_color_uploader) =
            create_default_buffer(device, cmd_list, &colors);
        let (index_buffer_gpu, index_buffer_uploader) =
            create_default_buffer(device, cmd_list, &indices);

        MeshGeometrySplitted {
            name: "boxGeo".to_string(),
            vertex_buffer_pos_cpu,
            vertex_buffer_color_cpu,
            index_buffer_cpu,
            vertex_buffer_pos_gpu,
            vertex_buffer_color_gpu,
            index_buffer_gpu,
            vertex_buffer_pos_uploader: Some(vertex_pos_buffer_uploader),
            vertex_buffer_color_uploader: Some(vertex_buffer_color_uploader),
            index_buffer_uploader: Some(index_buffer_uploader),
            vertex_pos_byte_stride: size_of::<VertexPos>() as u32,
            vertex_pos_byte_size: vb_pos_byte_size as u32,
            vertex_color_byte_stride: size_of::<VertexColor>() as u32,
            vertex_color_byte_size: vb_color_byte_size as u32,
            index_format: Format::R16Uint,
            index_buffer_byte_size: ib_byte_size as u32,
            draw_args: HashMap::from_iter([
                (
                    "box".to_string(),
                    SubmeshGeometry {
                        index_count: 36,
                        start_index_location: 0,
                        base_vertex_location: 0,
                        bounds: BoundingBox {
                            min: vec3(-1.0, -1.0, -1.0),
                            max: vec3(1.0, 1.0, 1.0),
                        },
                    },
                ),
                (
                    "pyramid".to_string(),
                    SubmeshGeometry {
                        index_count: 18,
                        start_index_location: 36,
                        base_vertex_location: 8,
                        bounds: BoundingBox {
                            min: vec3(-1.0, -1.0, -1.0),
                            max: vec3(1.0, 1.0, 1.0),
                        },
                    },
                ),
            ]),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VertexPos {
    pub pos: Vec3,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VertexColor {
    pub color: Vec4,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ObjectConstants {
    pub world_view_proj: Mat4,
}

#[derive(Clone, Debug)]
pub struct MeshGeometrySplitted {
    pub name: String,

    pub vertex_buffer_pos_cpu: Blob,
    pub vertex_buffer_color_cpu: Blob,
    pub index_buffer_cpu: Blob,

    pub vertex_buffer_pos_gpu: Resource,
    pub vertex_buffer_color_gpu: Resource,
    pub index_buffer_gpu: Resource,

    pub vertex_buffer_pos_uploader: Option<Resource>,
    pub vertex_buffer_color_uploader: Option<Resource>,
    pub index_buffer_uploader: Option<Resource>,

    pub vertex_pos_byte_stride: u32,
    pub vertex_pos_byte_size: u32,
    pub vertex_color_byte_stride: u32,
    pub vertex_color_byte_size: u32,
    pub index_format: Format,
    pub index_buffer_byte_size: u32,

    pub draw_args: HashMap<String, SubmeshGeometry>,
}

impl MeshGeometrySplitted {
    pub fn vertex_buffer_position_view(&self) -> VertexBufferView {
        VertexBufferView::new(
            self.vertex_buffer_pos_gpu.get_gpu_virtual_address(),
            self.vertex_pos_byte_stride as usize,
            self.vertex_pos_byte_size as usize,
        )
    }

    pub fn vertex_buffer_color_view(&self) -> VertexBufferView {
        VertexBufferView::new(
            self.vertex_buffer_color_gpu.get_gpu_virtual_address(),
            self.vertex_color_byte_stride as usize,
            self.vertex_color_byte_size as usize,
        )
    }

    pub fn index_buffer_view(&self) -> IndexBufferView {
        IndexBufferView::new(
            self.index_buffer_gpu.get_gpu_virtual_address(),
            self.index_buffer_byte_size as usize,
            self.index_format,
        )
    }

    pub fn dispose_uploaders(&mut self) {
        self.vertex_buffer_pos_uploader.take();
        self.vertex_buffer_color_uploader.take();
        self.index_buffer_uploader.take();
    }
}
