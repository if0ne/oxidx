mod frame_resources;
mod render_item;

use std::{cell::Cell, collections::HashMap, rc::Rc};

use common::{
    app::{DxSample, SwapchainContext},
    geometry_generator::GeometryGenerator,
    geometry_mesh::{BoundingBox, MeshGeometry, SubmeshGeometry},
    utils::{create_default_buffer, ConstantBufferData},
};
use glam::{vec2, vec3, vec4, Mat4, Vec3, Vec4};
use oxidx::dx::*;

use winit::keyboard::KeyCode;

use frame_resources::{FrameResource, ObjectConstants, PassConstants};
use render_item::RenderItem;

#[allow(unused)]
#[derive(Debug)]
pub struct ShapesSample {
    root_signature: RootSignature,
    cbv_heap: DescriptorHeap,
    frame_resources: [FrameResource; Self::FRAME_COUNT],
    curr_frame_resource: usize,

    all_ritems: Vec<Rc<RenderItem>>,
    opaque_ritems: Vec<Rc<RenderItem>>,
    transparent_ritems: Vec<Rc<RenderItem>>,

    geometries: HashMap<String, Rc<MeshGeometry>>,
    shaders: HashMap<String, Blob>,
    pso: HashMap<String, PipelineState>,

    eye_pos: Vec3,
    view: Mat4,
    proj: Mat4,

    main_pass_cb: ConstantBufferData<PassConstants>,
    pass_cbv_offset: u32,

    is_wireframe: bool,

    theta: f32,
    phi: f32,
    radius: f32,

    is_lmb_pressed: bool,
    is_rmb_pressed: bool,
}

impl DxSample for ShapesSample {
    fn new(base: &mut common::app::Base) -> Self {
        base.cmd_list.reset(&base.cmd_list_alloc, PSO_NONE).unwrap();

        let cbv_table1 = [DescriptorRange::cbv(1, 0)];
        let cbv_table2 = [DescriptorRange::cbv(1, 1)];

        let root_parameter = [
            RootParameter::descriptor_table(&cbv_table1),
            RootParameter::descriptor_table(&cbv_table2),
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

        let shaders = HashMap::from_iter([
            ("standardVS".to_string(), vs_byte_code),
            ("opaquePS".to_string(), ps_byte_code),
        ]);

        let input_layout = [
            InputElementDesc::per_vertex(SemanticName::Position(0), Format::Rgb32Float, 0),
            InputElementDesc::per_vertex(SemanticName::Color(0), Format::Rgba32Float, 0)
                .with_offset(12),
        ];

        let geometries = HashMap::from_iter([(
            "shapeGeo".to_string(),
            Rc::new(Self::build_geometry(&base.device, &base.cmd_list)),
        )]);

        let all_ritems = Self::build_render_items(&geometries);
        let opaque_ritems = all_ritems.clone();

        let frame_resources =
            std::array::from_fn(|_| FrameResource::new(&base.device, 1, opaque_ritems.len()));

        let pass_cbv_offset = opaque_ritems.len() * Self::FRAME_COUNT;
        let cbv_heap: DescriptorHeap = base
            .device
            .create_descriptor_heap(
                &DescriptorHeapDesc::cbr_srv_uav((opaque_ritems.len() + 1) * Self::FRAME_COUNT)
                    .with_flags(DescriptorHeapFlags::ShaderVisible),
            )
            .unwrap();

        Self::build_constant_buffer_view(
            &base.device,
            &cbv_heap,
            pass_cbv_offset,
            opaque_ritems.len(),
            &frame_resources,
            base.cbv_srv_uav_descriptor_size as usize,
        );

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
            });

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
            cbv_heap,
            frame_resources,
            curr_frame_resource: 0,
            pso,
            eye_pos: Vec3::ZERO,
            view: Mat4::IDENTITY,
            proj: Mat4::IDENTITY,
            theta: 0.0,
            phi: 0.0,
            radius: 5.0,
            is_lmb_pressed: false,
            is_rmb_pressed: false,
            all_ritems,
            opaque_ritems,
            transparent_ritems: vec![],
            geometries,
            shaders,
            main_pass_cb: ConstantBufferData(PassConstants::default()),
            pass_cbv_offset: pass_cbv_offset as u32,
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
            .set_descriptor_heaps(&[Some(self.cbv_heap.clone())]);

        base.cmd_list
            .set_graphics_root_signature(Some(&self.root_signature));

        let pass_cbv_index = self.pass_cbv_offset as usize + self.curr_frame_resource;
        let pass_cbv_handle = self
            .cbv_heap
            .get_gpu_descriptor_handle_for_heap_start()
            .advance(pass_cbv_index, base.cbv_srv_uav_descriptor_size as usize);
        base.cmd_list
            .set_graphics_root_descriptor_table(1, pass_cbv_handle);

        self.draw_render_items(
            &base.cmd_list,
            &self.opaque_ritems,
            base.cbv_srv_uav_descriptor_size as usize,
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

    fn on_key_down(&mut self, _: winit::keyboard::KeyCode, _: bool) {}

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
            self.radius = (self.radius + dx - dy).clamp(3.0, 15.0);
        }
    }
}

impl ShapesSample {
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

    fn build_geometry(device: &Device, cmd_list: &GraphicsCommandList) -> MeshGeometry {
        let r#box = GeometryGenerator::create_box(1.5, 0.5, 1.5, 3);
        let grid = GeometryGenerator::create_grid(20.0, 30.0, 60, 40);
        let sphere = GeometryGenerator::create_sphere(0.5, 20, 20);
        let cylinder = GeometryGenerator::create_cylinder(0.5, 0.3, 3.0, 20, 20);

        let box_vert_offset = 0;
        let grid_vert_offset = r#box.vertices.len() as u32;
        let sphere_vert_offset = grid_vert_offset + grid.vertices.len() as u32;
        let cylinder_vert_offset = sphere_vert_offset + sphere.vertices.len() as u32;

        let box_idx_offset = 0;
        let grid_idx_offset = r#box.indices32.len() as u32;
        let sphere_idx_offset = grid_idx_offset + grid.indices32.len() as u32;
        let cylinder_idx_offset = sphere_idx_offset + sphere.indices32.len() as u32;

        let box_submesh = SubmeshGeometry {
            index_count: r#box.indices32.len() as u32,
            start_index_location: box_idx_offset,
            base_vertex_location: box_vert_offset,
            bounds: BoundingBox::default(),
        };

        let grid_submesh = SubmeshGeometry {
            index_count: grid.indices32.len() as u32,
            start_index_location: grid_idx_offset,
            base_vertex_location: grid_vert_offset,
            bounds: BoundingBox::default(),
        };

        let sphere_submesh = SubmeshGeometry {
            index_count: sphere.indices32.len() as u32,
            start_index_location: sphere_idx_offset,
            base_vertex_location: sphere_vert_offset,
            bounds: BoundingBox::default(),
        };

        let cylinder_submesh = SubmeshGeometry {
            index_count: cylinder.indices32.len() as u32,
            start_index_location: cylinder_idx_offset,
            base_vertex_location: cylinder_vert_offset,
            bounds: BoundingBox::default(),
        };

        let vertices = r#box
            .vertices
            .iter()
            .map(|v| Vertex {
                pos: v.pos,
                color: vec4(1.0 / 255.0, 50.0 / 255.0, 32.0 / 255.0, 1.0),
            })
            .chain(grid.vertices.iter().map(|v| Vertex {
                pos: v.pos,
                color: vec4(34.0 / 255.0, 139.0 / 255.0, 34.0 / 255.0, 1.0),
            }))
            .chain(sphere.vertices.iter().map(|v| Vertex {
                pos: v.pos,
                color: vec4(220.0 / 255.0, 20.0 / 255.0, 60.0 / 255.0, 1.0),
            }))
            .chain(cylinder.vertices.iter().map(|v| Vertex {
                pos: v.pos,
                color: vec4(0.0 / 255.0, 0.0 / 255.0, 255.0, 1.0),
            }))
            .collect::<Vec<_>>();

        let indices = r#box
            .indices32
            .iter()
            .chain(grid.indices32.iter())
            .chain(sphere.indices32.iter())
            .chain(cylinder.indices32.iter())
            .map(|i| *i as u16)
            .collect::<Vec<_>>();

        let vertex_buffer = Blob::create_blob(size_of_val(vertices.as_slice())).unwrap();
        let index_buffer = Blob::create_blob(size_of_val(indices.as_slice())).unwrap();

        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices.as_ptr(),
                vertex_buffer.get_buffer_ptr::<Vertex>().as_mut(),
                vertices.len(),
            );
            std::ptr::copy_nonoverlapping(
                indices.as_ptr(),
                index_buffer.get_buffer_ptr::<u16>().as_mut(),
                indices.len(),
            );
        }

        let (vertex_buffer_gpu, vertex_buffer_uploader) =
            create_default_buffer(device, cmd_list, &vertices);
        let (index_buffer_gpu, index_buffer_uploader) =
            create_default_buffer(device, cmd_list, &indices);

        MeshGeometry {
            name: "shapeGeo".to_string(),
            vertex_buffer_cpu: vertex_buffer,
            index_buffer_cpu: index_buffer,
            vertex_buffer_gpu,
            index_buffer_gpu,
            vertex_buffer_uploader: Some(vertex_buffer_uploader),
            index_buffer_uploader: Some(index_buffer_uploader),
            vertex_byte_stride: size_of::<Vertex>() as u32,
            vertex_byte_size: size_of_val(vertices.as_slice()) as u32,
            index_format: Format::R16Uint,
            index_buffer_byte_size: size_of_val(indices.as_slice()) as u32,
            draw_args: HashMap::from_iter([
                ("box".to_string(), box_submesh),
                ("grid".to_string(), grid_submesh),
                ("cylinder".to_string(), cylinder_submesh),
                ("sphere".to_string(), sphere_submesh),
            ]),
        }
    }

    fn build_render_items(geometries: &HashMap<String, Rc<MeshGeometry>>) -> Vec<Rc<RenderItem>> {
        let mut vec = vec![];
        let geo = geometries.get("shapeGeo").unwrap();

        vec.push(Rc::new(RenderItem {
            world: Mat4::from_scale(vec3(2.0, 2.0, 2.0))
                * Mat4::from_translation(vec3(0.0, 0.5, 0.0)),
            num_frames_dirty: Cell::new(Self::FRAME_COUNT),
            obj_cb_index: 0,
            geo: Rc::clone(geo),
            primitive_type: PrimitiveTopology::Triangle,
            index_count: geo.draw_args.get("box").unwrap().index_count,
            start_index_location: geo.draw_args.get("box").unwrap().start_index_location,
            base_vertex_location: geo.draw_args.get("box").unwrap().base_vertex_location,
        }));

        vec.push(Rc::new(RenderItem {
            world: Mat4::IDENTITY,
            num_frames_dirty: Cell::new(Self::FRAME_COUNT),
            obj_cb_index: 1,
            geo: Rc::clone(geo),
            primitive_type: PrimitiveTopology::Triangle,
            index_count: geo.draw_args.get("grid").unwrap().index_count,
            start_index_location: geo.draw_args.get("grid").unwrap().start_index_location,
            base_vertex_location: geo.draw_args.get("grid").unwrap().base_vertex_location,
        }));

        let mut obj_index = 2;

        for i in 0..5 {
            vec.push(Rc::new(RenderItem {
                world: Mat4::from_translation(vec3(-5.0, 1.5, -10.0 + i as f32 * 5.0)),
                num_frames_dirty: Cell::new(Self::FRAME_COUNT),
                obj_cb_index: obj_index,
                geo: Rc::clone(geo),
                primitive_type: PrimitiveTopology::Triangle,
                index_count: geo.draw_args.get("cylinder").unwrap().index_count,
                start_index_location: geo.draw_args.get("cylinder").unwrap().start_index_location,
                base_vertex_location: geo.draw_args.get("cylinder").unwrap().base_vertex_location,
            }));

            obj_index += 1;

            vec.push(Rc::new(RenderItem {
                world: Mat4::from_translation(vec3(5.0, 1.5, -10.0 + i as f32 * 5.0)),
                num_frames_dirty: Cell::new(Self::FRAME_COUNT),
                obj_cb_index: obj_index,
                geo: Rc::clone(geo),
                primitive_type: PrimitiveTopology::Triangle,
                index_count: geo.draw_args.get("cylinder").unwrap().index_count,
                start_index_location: geo.draw_args.get("cylinder").unwrap().start_index_location,
                base_vertex_location: geo.draw_args.get("cylinder").unwrap().base_vertex_location,
            }));

            obj_index += 1;

            vec.push(Rc::new(RenderItem {
                world: Mat4::from_translation(vec3(-5.0, 3.5, -10.0 + i as f32 * 5.0)),
                num_frames_dirty: Cell::new(Self::FRAME_COUNT),
                obj_cb_index: obj_index,
                geo: Rc::clone(geo),
                primitive_type: PrimitiveTopology::Triangle,
                index_count: geo.draw_args.get("sphere").unwrap().index_count,
                start_index_location: geo.draw_args.get("sphere").unwrap().start_index_location,
                base_vertex_location: geo.draw_args.get("sphere").unwrap().base_vertex_location,
            }));

            obj_index += 1;

            vec.push(Rc::new(RenderItem {
                world: Mat4::from_translation(vec3(5.0, 3.5, -10.0 + i as f32 * 5.0)),
                num_frames_dirty: Cell::new(Self::FRAME_COUNT),
                obj_cb_index: obj_index,
                geo: Rc::clone(geo),
                primitive_type: PrimitiveTopology::Triangle,
                index_count: geo.draw_args.get("sphere").unwrap().index_count,
                start_index_location: geo.draw_args.get("sphere").unwrap().start_index_location,
                base_vertex_location: geo.draw_args.get("sphere").unwrap().base_vertex_location,
            }));

            obj_index += 1;
        }

        vec
    }

    fn build_constant_buffer_view(
        device: &Device,
        cbv_heap: &DescriptorHeap,
        pass_offset: usize,
        obj_count: usize,
        frame_resources: &[FrameResource; Self::FRAME_COUNT],
        handle_size: usize,
    ) {
        let obj_size = size_of::<ConstantBufferData<ObjectConstants>>();

        for (frame, resource) in frame_resources.iter().enumerate() {
            let obj_cb = resource.object_cb.resource();

            for i in 0..obj_count {
                let mut addr = obj_cb.get_gpu_virtual_address();

                addr += (i * obj_size) as u64;

                let heap_idx = frame * obj_count + i;
                let handle = cbv_heap.get_cpu_descriptor_handle_for_heap_start();
                let handle = handle.advance(heap_idx, handle_size);

                device.create_constant_buffer_view(
                    Some(&ConstantBufferViewDesc::new(addr, obj_size as u32)),
                    handle,
                );
            }
        }

        let pass_size = size_of::<ConstantBufferData<PassConstants>>();

        for (frame, resource) in frame_resources.iter().enumerate() {
            let pass_cb = resource.pass_cb.resource();

            let addr = pass_cb.get_gpu_virtual_address();

            let heap_idx = frame + pass_offset;
            let handle = cbv_heap.get_cpu_descriptor_handle_for_heap_start();
            let handle = handle.advance(heap_idx, handle_size);

            device.create_constant_buffer_view(
                Some(&ConstantBufferViewDesc::new(addr, pass_size as u32)),
                handle,
            );
        }
    }

    fn draw_render_items(
        &self,
        cmd_list: &GraphicsCommandList,
        ritems: &[Rc<RenderItem>],
        cbv_descriptor_size: usize,
    ) {
        for item in ritems {
            cmd_list.ia_set_vertex_buffers(0, &[item.geo.vertex_buffer_view()]);
            cmd_list.ia_set_index_buffer(Some(&item.geo.index_buffer_view()));
            cmd_list.ia_set_primitive_topology(item.primitive_type);

            let cbv_index = self.curr_frame_resource * self.opaque_ritems.len() + item.obj_cb_index;
            let cbv_handle = self.cbv_heap.get_gpu_descriptor_handle_for_heap_start();
            let cbv_handle = cbv_handle.advance(cbv_index, cbv_descriptor_size);

            cmd_list.set_graphics_root_descriptor_table(0, cbv_handle);
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

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
struct Vertex {
    pos: Vec3,
    color: Vec4,
}
