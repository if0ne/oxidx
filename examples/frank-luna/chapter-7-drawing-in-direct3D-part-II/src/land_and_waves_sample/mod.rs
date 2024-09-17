mod frame_resources;
mod render_item;
mod waves;

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use common::{
    app::{DxSample, SwapchainContext},
    geometry_generator::GeometryGenerator,
    geometry_mesh::{BoundingBox, MeshGeometry, SubmeshGeometry},
    utils::{create_default_buffer, ConstantBufferData},
};
use glam::{vec2, vec3, vec4, Mat4, Vec3};
use oxidx::dx::*;

use rand::Rng;
use waves::Waves;
use winit::keyboard::KeyCode;

use frame_resources::{FrameResource, ObjectConstants, PassConstants, Vertex};
use render_item::RenderItem;

#[allow(unused)]
#[derive(Debug)]
pub struct LandAndWavesSample {
    root_signature: RootSignature,
    frame_resources: [FrameResource; Self::FRAME_COUNT],
    curr_frame_resource: usize,

    all_ritems: Vec<Rc<RenderItem>>,
    opaque_ritems: Vec<Rc<RenderItem>>,
    waves_ritem: Rc<RenderItem>,
    waves: Box<Waves>,

    geometries: HashMap<String, Rc<RefCell<MeshGeometry>>>,
    shaders: HashMap<String, Blob>,
    pso: HashMap<String, PipelineState>,

    eye_pos: Vec3,
    view: Mat4,
    proj: Mat4,

    main_pass_cb: ConstantBufferData<PassConstants>,

    is_wireframe: bool,

    theta: f32,
    phi: f32,
    radius: f32,

    is_lmb_pressed: bool,
    is_rmb_pressed: bool,
}

impl DxSample for LandAndWavesSample {
    fn new(base: &mut common::app::Base) -> Self {
        base.cmd_list.reset(&base.cmd_list_alloc, PSO_NONE).unwrap();

        let waves = Box::new(Waves::new(128, 128, 1.0, 0.03, 4.0, 0.2));

        let root_parameter = [RootParameter::cbv(0, 0), RootParameter::cbv(1, 0)];

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

        let shaders = HashMap::from_iter([
            ("standardVS".to_string(), vs_byte_code),
            ("opaquePS".to_string(), ps_byte_code),
        ]);

        let input_layout = [
            InputElementDesc::per_vertex(SemanticName::Position(0), Format::Rgb32Float, 0),
            InputElementDesc::per_vertex(SemanticName::Color(0), Format::Rgba32Float, 0)
                .with_offset(16),
        ];

        let geometries = HashMap::from_iter([
            (
                "landGeo".to_string(),
                Rc::new(RefCell::new(Self::build_land_geometry(
                    &base.device,
                    &base.cmd_list,
                ))),
            ),
            (
                "waterGeo".to_string(),
                Rc::new(RefCell::new(Self::build_waves_geometry(
                    &base.device,
                    &base.cmd_list,
                    &waves,
                ))),
            ),
        ]);

        let all_ritems = vec![
            Rc::new(RenderItem {
                world: Mat4::IDENTITY,
                num_frames_dirty: Cell::new(Self::FRAME_COUNT),
                obj_cb_index: 0,
                geo: Rc::clone(geometries.get("waterGeo").unwrap()),
                primitive_type: PrimitiveTopology::Triangle,
                index_count: geometries
                    .get("waterGeo")
                    .unwrap()
                    .borrow()
                    .draw_args
                    .get("grid")
                    .unwrap()
                    .index_count,
                start_index_location: geometries
                    .get("waterGeo")
                    .unwrap()
                    .borrow()
                    .draw_args
                    .get("grid")
                    .unwrap()
                    .start_index_location,
                base_vertex_location: geometries
                    .get("waterGeo")
                    .unwrap()
                    .borrow()
                    .draw_args
                    .get("grid")
                    .unwrap()
                    .base_vertex_location,
            }),
            Rc::new(RenderItem {
                world: Mat4::IDENTITY,
                num_frames_dirty: Cell::new(Self::FRAME_COUNT),
                obj_cb_index: 1,
                geo: Rc::clone(geometries.get("landGeo").unwrap()),
                primitive_type: PrimitiveTopology::Triangle,
                index_count: geometries
                    .get("landGeo")
                    .unwrap()
                    .borrow()
                    .draw_args
                    .get("grid")
                    .unwrap()
                    .index_count,
                start_index_location: geometries
                    .get("landGeo")
                    .unwrap()
                    .borrow()
                    .draw_args
                    .get("grid")
                    .unwrap()
                    .start_index_location,
                base_vertex_location: geometries
                    .get("landGeo")
                    .unwrap()
                    .borrow()
                    .draw_args
                    .get("grid")
                    .unwrap()
                    .base_vertex_location,
            }),
        ];
        let opaque_ritems = all_ritems.clone();

        let frame_resources = std::array::from_fn(|_| {
            FrameResource::new(
                &base.device,
                1,
                opaque_ritems.len(),
                waves.vertex_count as usize,
            )
        });

        let pso_desc = GraphicsPipelineDesc::new(shaders.get("standardVS").unwrap())
            .with_ps(shaders.get("opaquePS").unwrap())
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

        let pso_opaque = base.device.create_graphics_pipeline(&pso_desc).unwrap();

        let pso_desc = pso_desc
            .with_rasterizer_state(RasterizerDesc::default().with_fill_mode(FillMode::Wireframe));

        let pso_wireframe = base.device.create_graphics_pipeline(&pso_desc).unwrap();

        let pso = HashMap::from_iter([
            ("opaque".to_string(), pso_opaque),
            ("opaque_wireframe".to_string(), pso_wireframe),
        ]);

        base.cmd_list.close().unwrap();

        base.cmd_queue
            .execute_command_lists(&[Some(base.cmd_list.clone())]);
        base.flush_command_queue();

        Self {
            root_signature,
            frame_resources,
            curr_frame_resource: 0,
            pso,
            eye_pos: Vec3::ZERO,
            view: Mat4::IDENTITY,
            proj: Mat4::IDENTITY,
            theta: 0.0,
            phi: 0.0,
            radius: 200.0,
            is_lmb_pressed: false,
            is_rmb_pressed: false,
            waves_ritem: Rc::clone(&all_ritems[0]),
            waves,
            all_ritems,
            opaque_ritems,
            geometries,
            shaders,
            main_pass_cb: ConstantBufferData(PassConstants::default()),
            is_wireframe: false,
        }
    }

    fn init_resources(&mut self, _: &common::app::Base) {}

    fn update(&mut self, base: &common::app::Base) {
        self.eye_pos = vec3(
            self.radius * self.phi.sin() * self.theta.cos(),
            self.radius * self.phi.cos(),
            self.radius * self.phi.sin() * self.theta.sin(),
        );
        self.view = Mat4::look_at_lh(self.eye_pos, Vec3::ZERO, Vec3::Y);

        self.curr_frame_resource = (self.curr_frame_resource + 1) % Self::FRAME_COUNT;
        let curr_frame_resource = &self.frame_resources[self.curr_frame_resource];

        if curr_frame_resource.fence != 0
            && base.fence.get_completed_value() < curr_frame_resource.fence
        {
            let event = Event::create(false, false).unwrap();
            base.fence
                .set_event_on_completion(curr_frame_resource.fence, event)
                .unwrap();
            event.wait(u32::MAX);
            event.close().unwrap();
        }

        self.update_object_cb(base);
        self.update_pass_cb(base);
        self.update_waves(base);
    }

    fn render(&mut self, base: &mut common::app::Base) {
        let Some(ref context) = base.context else {
            return;
        };

        let cmd_list_alloc = &self.frame_resources[self.curr_frame_resource].cmd_list_alloc;
        cmd_list_alloc.reset().unwrap();

        if self.is_wireframe {
            base.cmd_list
                .reset(
                    cmd_list_alloc,
                    Some(self.pso.get("opaque_wireframe").unwrap()),
                )
                .unwrap();
        } else {
            base.cmd_list
                .reset(cmd_list_alloc, Some(self.pso.get("opaque").unwrap()))
                .unwrap();
        }

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
            .set_graphics_root_signature(Some(&self.root_signature));

        let pass_cb = self.frame_resources[self.curr_frame_resource]
            .pass_cb
            .resource();
        base.cmd_list
            .set_graphics_root_constant_buffer_view(1, pass_cb.get_gpu_virtual_address());

        self.draw_render_items(&base.cmd_list, &self.opaque_ritems);

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

        base.current_fence += 1;
        self.frame_resources[self.curr_frame_resource].fence = base.current_fence;
        base.cmd_queue
            .signal(&base.fence, base.current_fence)
            .unwrap();
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

    fn on_key_up(&mut self, key: winit::keyboard::KeyCode) {
        match key {
            KeyCode::Digit1 => self.is_wireframe = false,
            KeyCode::Digit2 => self.is_wireframe = true,
            _ => {}
        }
    }

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
            self.radius = (self.radius + dx - dy).clamp(3.0, 400.0);
        }
    }
}

impl LandAndWavesSample {
    const FRAME_COUNT: usize = 3;

    fn update_object_cb(&mut self, _: &common::app::Base) {
        let curr_obj_cb = &self.frame_resources[self.curr_frame_resource].object_cb;

        for e in &mut self.all_ritems {
            let num_frames_dirty = e.num_frames_dirty.get();
            if num_frames_dirty > 0 {
                curr_obj_cb.copy_data(
                    e.obj_cb_index,
                    ConstantBufferData(ObjectConstants { world: e.world }),
                );
                e.num_frames_dirty.set(num_frames_dirty - 1);
            }
        }
    }

    fn update_pass_cb(&mut self, base: &common::app::Base) {
        let view = self.view;
        let proj = self.proj;

        let view_proj = proj * view;
        let inv_view = view.inverse();
        let inv_proj = proj.inverse();
        let inv_view_proj = view_proj.inverse();

        let pass_const = PassConstants {
            view,
            inv_view,
            proj,
            inv_proj,
            view_proj,
            inv_view_proj,
            eye_pos: self.eye_pos,
            cb_per_object_pad1: 0.0,
            render_target_size: vec2(base.client_width as f32, base.client_height as f32),
            inv_render_target_size: vec2(
                1.0 / base.client_width as f32,
                1.0 / base.client_height as f32,
            ),
            near_z: 1.0,
            far_z: 1000.0,
            total_time: base.timer.total_time(),
            delta_time: base.timer.delta_time(),
        };

        self.frame_resources[self.curr_frame_resource]
            .pass_cb
            .copy_data(0, ConstantBufferData(pass_const));
    }

    fn update_waves(&mut self, base: &common::app::Base) {
        thread_local! {
            static T: RefCell<f32> = Default::default();
        }

        T.with_borrow_mut(|t_base| {
            if base.timer.total_time() - *t_base >= 0.25 {
                *t_base += 0.25;

                let i = rand::thread_rng().gen_range(4..(self.waves.rows - 5));
                let j = rand::thread_rng().gen_range(4..(self.waves.cols - 5));

                let r = rand::thread_rng().gen_range(0.2..0.5) as f32;

                self.waves.disturb(i, j, r);
            }

            self.waves.update(base.timer.delta_time());

            let curr_waves_vb = &self.frame_resources[self.curr_frame_resource].wave_cb;
            for i in 0..self.waves.vertex_count {
                let v = Vertex {
                    pos: self.waves.curr_solution[i as usize],
                    color: vec4(0.0, 0.0, 1.0, 1.0),
                };

                curr_waves_vb.copy_data(i as usize, v);
            }

            self.waves_ritem.geo.borrow_mut().vertex_buffer_gpu =
                Some(curr_waves_vb.resource().clone());
        });
    }

    fn build_land_geometry(device: &Device, cmd_list: &GraphicsCommandList) -> MeshGeometry {
        let mut grid = GeometryGenerator::create_grid(160.0, 160.0, 50, 50);

        let mut vertices = Vec::with_capacity(grid.vertices.len());
        for v in grid.vertices.iter_mut() {
            let x = v.pos.x;
            let z = v.pos.z;
            let y = Self::get_hills_height(x, z);

            if y < -10.0 {
                // Sandy beach color.
                vertices.push(Vertex {
                    pos: vec3(x, y, z),
                    color: vec4(1.0, 0.96, 0.62, 1.0),
                });
            } else if y < 5.0 {
                // Light yellow-green.
                vertices.push(Vertex {
                    pos: vec3(x, y, z),
                    color: vec4(0.48, 0.77, 0.46, 1.0),
                });
            } else if y < 12.0 {
                // Dark yellow-green.
                vertices.push(Vertex {
                    pos: vec3(x, y, z),
                    color: vec4(0.1, 0.48, 0.19, 1.0),
                });
            } else if y < 20.0 {
                // Dark brown.
                vertices.push(Vertex {
                    pos: vec3(x, y, z),
                    color: vec4(0.45, 0.39, 0.34, 1.0),
                });
            } else {
                // White snow.
                vertices.push(Vertex {
                    pos: vec3(x, y, z),
                    color: vec4(1.0, 1.0, 1.0, 1.0),
                });
            }
        }

        let vertex_buffer_cpu = Blob::create_blob(size_of_val(vertices.as_slice())).unwrap();
        let index_buffer_cpu = Blob::create_blob(size_of_val(grid.indices16().as_slice())).unwrap();

        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices.as_ptr(),
                vertex_buffer_cpu.get_buffer_ptr::<Vertex>().as_mut(),
                vertices.len(),
            );
            std::ptr::copy_nonoverlapping(
                grid.indices16().as_ptr(),
                index_buffer_cpu.get_buffer_ptr::<u16>().as_mut(),
                grid.indices32.len(),
            );
        }

        let (vertex_buffer_gpu, vertex_buffer_uploader) =
            create_default_buffer(device, cmd_list, &vertices);
        let (index_buffer_gpu, index_buffer_uploader) =
            create_default_buffer(device, cmd_list, grid.indices16().as_slice());

        let index_buffer_byte_size = size_of_val(grid.indices16().as_slice()) as u32;

        MeshGeometry {
            name: "landGeo".to_string(),
            vertex_buffer_cpu,
            index_buffer_cpu,
            vertex_buffer_gpu: Some(vertex_buffer_gpu),
            index_buffer_gpu: Some(index_buffer_gpu),
            vertex_buffer_uploader: Some(vertex_buffer_uploader),
            index_buffer_uploader: Some(index_buffer_uploader),
            vertex_byte_stride: size_of::<Vertex>() as u32,
            vertex_byte_size: size_of_val(vertices.as_slice()) as u32,
            index_format: Format::R16Uint,
            index_buffer_byte_size,
            draw_args: HashMap::from_iter([(
                "grid".to_string(),
                SubmeshGeometry {
                    index_count: grid.indices32.len() as u32,
                    start_index_location: 0,
                    base_vertex_location: 0,
                    bounds: BoundingBox::default(),
                },
            )]),
        }
    }

    fn build_waves_geometry(
        device: &Device,
        cmd_list: &GraphicsCommandList,
        waves: &Waves,
    ) -> MeshGeometry {
        let mut indices = Vec::with_capacity(3 * waves.triangle_count as usize);

        let m = waves.rows;
        let n = waves.cols;

        for i in 0..(m - 1) {
            for j in 0..(n - 1) {
                indices.push((i * n + j) as u16);
                indices.push((i * n + j + 1) as u16);
                indices.push(((i + 1) * n + j) as u16);

                indices.push(((i + 1) * n + j) as u16);
                indices.push((i * n + j + 1) as u16);
                indices.push(((i + 1) * n + j + 1) as u16);
            }
        }

        let vertex_buffer_cpu =
            Blob::create_blob(waves.vertex_count as usize * size_of::<Vertex>()).unwrap();
        let index_buffer_cpu = Blob::create_blob(size_of_val(indices.as_slice())).unwrap();

        unsafe {
            std::ptr::copy_nonoverlapping(
                indices.as_ptr(),
                index_buffer_cpu.get_buffer_ptr::<u16>().as_mut(),
                indices.len(),
            );
        }

        let (index_buffer_gpu, index_buffer_uploader) =
            create_default_buffer(device, cmd_list, indices.as_slice());

        let index_buffer_byte_size = size_of_val(indices.as_slice()) as u32;

        MeshGeometry {
            name: "waterGeo".to_string(),
            vertex_buffer_cpu,
            index_buffer_cpu,
            vertex_buffer_gpu: None,
            index_buffer_gpu: Some(index_buffer_gpu),
            vertex_buffer_uploader: None,
            index_buffer_uploader: Some(index_buffer_uploader),
            vertex_byte_stride: size_of::<Vertex>() as u32,
            vertex_byte_size: waves.vertex_count * size_of::<Vertex>() as u32,
            index_format: Format::R16Uint,
            index_buffer_byte_size,
            draw_args: HashMap::from_iter([(
                "grid".to_string(),
                SubmeshGeometry {
                    index_count: indices.len() as u32,
                    start_index_location: 0,
                    base_vertex_location: 0,
                    bounds: BoundingBox::default(),
                },
            )]),
        }
    }

    fn get_hills_height(x: f32, z: f32) -> f32 {
        0.3 * (z * (0.1 * x).sin()) + x * (0.1 * z).cos()
    }

    fn draw_render_items(&self, cmd_list: &GraphicsCommandList, ritems: &[Rc<RenderItem>]) {
        let obj_size = size_of::<ConstantBufferData<ObjectConstants>>();
        let obj_cb = self.frame_resources[self.curr_frame_resource]
            .object_cb
            .resource();

        for item in ritems {
            cmd_list.ia_set_vertex_buffers(0, &[item.geo.borrow().vertex_buffer_view()]);
            cmd_list.ia_set_index_buffer(Some(&item.geo.borrow().index_buffer_view()));
            cmd_list.ia_set_primitive_topology(item.primitive_type);

            let obj_addr = obj_cb.get_gpu_virtual_address() + (item.obj_cb_index * obj_size) as u64;

            cmd_list.set_graphics_root_constant_buffer_view(0, obj_addr);
            cmd_list.draw_indexed_instanced(
                item.index_count,
                1,
                item.start_index_location,
                item.base_vertex_location as i32,
                0,
            );
        }
    }
}
