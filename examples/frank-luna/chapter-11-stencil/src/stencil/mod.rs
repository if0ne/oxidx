mod frame_resources;
mod render_item;

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    f32::consts::PI,
    io::{BufRead, BufReader},
    mem::offset_of,
    rc::Rc,
};

use common::{
    app::{DxSample, SwapchainContext},
    geometry_mesh::{BoundingBox, MeshGeometry, SubmeshGeometry},
    material::Material,
    texture::Texture,
    utils::{create_default_buffer, load_texture_from_file, ConstantBufferData},
};
use glam::{vec2, vec3, vec4, Mat4, Vec2, Vec3};
use oxidx::dx::*;

use winit::keyboard::KeyCode;

use frame_resources::{FrameResource, MaterialConstant, ObjectConstants, PassConstants, Vertex};
use render_item::RenderItem;

use crate::utils::MatrixExt;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum RenderLayer {
    Opaque,
    Transparent,
    AlphaTested,
    Mirrors,
    Reflected,
    Shadow,
}

#[allow(unused)]
#[derive(Debug)]
pub struct LandAndWavesSample {
    root_signature: RootSignature,
    frame_resources: [FrameResource; Self::FRAME_COUNT],
    srv_descriptor_heap: DescriptorHeap,
    curr_frame_resource: usize,
    cbv_srv_descriptor_size: usize,

    all_ritems: Vec<Rc<RenderItem>>,
    ritems_by_layer: HashMap<RenderLayer, Vec<Rc<RenderItem>>>,

    skull_ritem: Rc<RenderItem>,
    skull_reflected: Rc<RenderItem>,
    skull_shadow: Rc<RenderItem>,

    geometries: HashMap<String, Rc<RefCell<MeshGeometry>>>,
    shaders: HashMap<String, Blob>,
    materials: HashMap<String, Rc<RefCell<Material>>>,
    textures: HashMap<String, Texture>,
    pso: HashMap<String, PipelineState>,

    eye_pos: Vec3,
    view: Mat4,
    proj: Mat4,

    main_pass_cb: ConstantBufferData<PassConstants>,
    reflected_pass_cb: ConstantBufferData<PassConstants>,

    is_wireframe: bool,

    theta: f32,
    phi: f32,
    radius: f32,

    is_lmb_pressed: bool,
    is_rmb_pressed: bool,

    skull_translation: Vec3,
}

impl DxSample for LandAndWavesSample {
    fn new(base: &mut common::app::Base) -> Self {
        base.cmd_list.reset(&base.cmd_list_alloc, PSO_NONE).unwrap();

        let cbv_srv_descriptor_size = base
            .device
            .get_descriptor_handle_increment_size(DescriptorHeapType::CbvSrvUav);

        let textures = Self::load_textures(&base.device, &base.cmd_list);

        let heap_desc =
            DescriptorHeapDesc::cbr_srv_uav(4).with_flags(DescriptorHeapFlags::ShaderVisible);

        let descriptor_heap = base
            .device
            .create_descriptor_heap::<DescriptorHeap>(&heap_desc)
            .unwrap();

        let srv_desc = ShaderResourceViewDesc::texture_2d(Format::Rgba8Unorm, 0, u32::MAX, 0.0, 0);

        let descriptor = descriptor_heap.get_cpu_descriptor_handle_for_heap_start();
        base.device.create_shader_resource_view(
            Some(&textures.get("bricks").unwrap().image),
            Some(&srv_desc),
            descriptor,
        );

        let descriptor = descriptor.advance(1, cbv_srv_descriptor_size);
        base.device.create_shader_resource_view(
            Some(&textures.get("checkboard").unwrap().image),
            Some(&srv_desc),
            descriptor,
        );

        let descriptor = descriptor.advance(1, cbv_srv_descriptor_size);
        base.device.create_shader_resource_view(
            Some(&textures.get("ice").unwrap().image),
            Some(&srv_desc),
            descriptor,
        );

        let descriptor = descriptor.advance(1, cbv_srv_descriptor_size);
        base.device.create_shader_resource_view(
            Some(&textures.get("white1x1").unwrap().image),
            Some(&srv_desc),
            descriptor,
        );

        let table = [DescriptorRange::srv(1, 0)];
        let root_parameter = [
            RootParameter::descriptor_table(&table).with_visibility(ShaderVisibility::Pixel),
            RootParameter::cbv(0, 0),
            RootParameter::cbv(1, 0),
            RootParameter::cbv(2, 0),
        ];

        let static_samplers = Self::get_static_samplers();

        let root_signature_desc = RootSignatureDesc::default()
            .with_parameters(&root_parameter)
            .with_samplers(&static_samplers)
            .with_flags(RootSignatureFlags::AllowInputAssemblerInputLayout);

        let root_signature = base
            .device
            .serialize_and_create_root_signature(
                &root_signature_desc,
                RootSignatureVersion::V1_0,
                0,
            )
            .unwrap();

        let opaque_defines = [ShaderMacro::new(c"FOG", c"1"), ShaderMacro::default()];

        let alpha_tested_defines = [
            ShaderMacro::new(c"FOG", c"1"),
            ShaderMacro::new(c"ALPHA_TEST", c"1"),
            ShaderMacro::default(),
        ];

        let standard_vs = Blob::compile_from_file(
            "shader.hlsl",
            &[],
            c"VS",
            c"vs_5_1",
            PACK_MATRIX_ROW_MAJOR | COMPILE_DEBUG | COMPILE_SKIP_OPT,
            0,
        )
        .unwrap();
        let opaque_ps = Blob::compile_from_file(
            "shader.hlsl",
            &opaque_defines,
            c"PS",
            c"ps_5_1",
            PACK_MATRIX_ROW_MAJOR | COMPILE_DEBUG | COMPILE_SKIP_OPT,
            0,
        )
        .unwrap();

        let alpha_ps = Blob::compile_from_file(
            "shader.hlsl",
            &alpha_tested_defines,
            c"PS",
            c"ps_5_1",
            PACK_MATRIX_ROW_MAJOR | COMPILE_DEBUG | COMPILE_SKIP_OPT,
            0,
        )
        .unwrap();

        let shaders = HashMap::from_iter([
            ("standardVS".to_string(), standard_vs),
            ("opaquePS".to_string(), opaque_ps),
            ("alphaTestedPS".to_string(), alpha_ps),
        ]);

        let input_layout = [
            InputElementDesc::per_vertex(SemanticName::Position(0), Format::Rgb32Float, 0),
            InputElementDesc::per_vertex(SemanticName::Normal(0), Format::Rgb32Float, 0)
                .with_offset(offset_of!(Vertex, normal)),
            InputElementDesc::per_vertex(SemanticName::TexCoord(0), Format::Rgb32Float, 0)
                .with_offset(offset_of!(Vertex, uv)),
        ];

        let geometries = HashMap::from_iter([
            (
                "roomGeo".to_string(),
                Rc::new(RefCell::new(Self::build_room_geometry(
                    &base.device,
                    &base.cmd_list,
                ))),
            ),
            (
                "skullGeo".to_string(),
                Rc::new(RefCell::new(Self::build_skull_geometry(
                    &base.device,
                    &base.cmd_list,
                ))),
            ),
        ]);

        let materials = HashMap::from_iter([
            (
                "bricks".to_string(),
                Rc::new(RefCell::new(Material {
                    name: "bricks".to_string(),
                    cb_index: 0,
                    diffuse_srv_heap_index: Some(0),
                    num_frames_dirty: Self::FRAME_COUNT,
                    diffuse_albedo: vec4(1.0, 1.0, 1.0, 1.0),
                    fresnel_r0: vec3(0.05, 0.05, 0.05),
                    roughness: 0.25,
                    transform: Mat4::IDENTITY,
                })),
            ),
            (
                "checkertile".to_string(),
                Rc::new(RefCell::new(Material {
                    name: "checkertile".to_string(),
                    cb_index: 1,
                    diffuse_srv_heap_index: Some(1),
                    num_frames_dirty: Self::FRAME_COUNT,
                    diffuse_albedo: vec4(1.0, 1.0, 1.0, 1.0),
                    fresnel_r0: vec3(0.07, 0.07, 0.07),
                    roughness: 0.3,
                    transform: Mat4::IDENTITY,
                })),
            ),
            (
                "icemirror".to_string(),
                Rc::new(RefCell::new(Material {
                    name: "icemirror".to_string(),
                    cb_index: 2,
                    diffuse_srv_heap_index: Some(2),
                    num_frames_dirty: Self::FRAME_COUNT,
                    diffuse_albedo: vec4(1.0, 1.0, 1.0, 1.0),
                    fresnel_r0: vec3(0.1, 0.1, 0.1),
                    roughness: 0.5,
                    transform: Mat4::IDENTITY,
                })),
            ),
            (
                "skullMat".to_string(),
                Rc::new(RefCell::new(Material {
                    name: "skullMat".to_string(),
                    cb_index: 3,
                    diffuse_srv_heap_index: Some(3),
                    num_frames_dirty: Self::FRAME_COUNT,
                    diffuse_albedo: vec4(1.0, 1.0, 1.0, 1.0),
                    fresnel_r0: vec3(0.05, 0.05, 0.05),
                    roughness: 0.3,
                    transform: Mat4::IDENTITY,
                })),
            ),
            (
                "shadowMat".to_string(),
                Rc::new(RefCell::new(Material {
                    name: "shadowMat".to_string(),
                    cb_index: 4,
                    diffuse_srv_heap_index: Some(4),
                    num_frames_dirty: Self::FRAME_COUNT,
                    diffuse_albedo: vec4(0.0, 0.0, 0.0, 1.0),
                    fresnel_r0: vec3(0.001, 0.001, 0.001),
                    roughness: 0.0,
                    transform: Mat4::IDENTITY,
                })),
            ),
        ]);

        let ri_floor = Rc::new(RenderItem {
            world: RefCell::new(Mat4::IDENTITY),
            num_frames_dirty: Cell::new(Self::FRAME_COUNT),
            obj_cb_index: 0,
            geo: Rc::clone(geometries.get("roomGeo").unwrap()),
            material: Rc::clone(materials.get("checkertile").unwrap()),
            primitive_type: PrimitiveTopology::Triangle,
            index_count: geometries
                .get("roomGeo")
                .unwrap()
                .borrow()
                .draw_args
                .get("floor")
                .unwrap()
                .index_count,
            start_index_location: geometries
                .get("roomGeo")
                .unwrap()
                .borrow()
                .draw_args
                .get("floor")
                .unwrap()
                .start_index_location,
            base_vertex_location: geometries
                .get("roomGeo")
                .unwrap()
                .borrow()
                .draw_args
                .get("floor")
                .unwrap()
                .base_vertex_location,
        });

        let ri_walls = Rc::new(RenderItem {
            world: RefCell::new(Mat4::IDENTITY),
            num_frames_dirty: Cell::new(Self::FRAME_COUNT),
            obj_cb_index: 1,
            geo: Rc::clone(geometries.get("roomGeo").unwrap()),
            material: Rc::clone(materials.get("bricks").unwrap()),
            primitive_type: PrimitiveTopology::Triangle,
            index_count: geometries
                .get("roomGeo")
                .unwrap()
                .borrow()
                .draw_args
                .get("wall")
                .unwrap()
                .index_count,
            start_index_location: geometries
                .get("roomGeo")
                .unwrap()
                .borrow()
                .draw_args
                .get("wall")
                .unwrap()
                .start_index_location,
            base_vertex_location: geometries
                .get("roomGeo")
                .unwrap()
                .borrow()
                .draw_args
                .get("wall")
                .unwrap()
                .base_vertex_location,
        });

        let ri_skull = Rc::new(RenderItem {
            world: RefCell::new(Mat4::IDENTITY),
            num_frames_dirty: Cell::new(Self::FRAME_COUNT),
            obj_cb_index: 2,
            geo: Rc::clone(geometries.get("skullGeo").unwrap()),
            material: Rc::clone(materials.get("skull").unwrap()),
            primitive_type: PrimitiveTopology::Triangle,
            index_count: geometries
                .get("skullGeo")
                .unwrap()
                .borrow()
                .draw_args
                .get("skull")
                .unwrap()
                .index_count,
            start_index_location: geometries
                .get("skullGeo")
                .unwrap()
                .borrow()
                .draw_args
                .get("skull")
                .unwrap()
                .start_index_location,
            base_vertex_location: geometries
                .get("skullGeo")
                .unwrap()
                .borrow()
                .draw_args
                .get("skull")
                .unwrap()
                .base_vertex_location,
        });

        let ri_skull_reflected = Rc::new(RenderItem {
            obj_cb_index: 3,
            ..(*ri_skull).clone()
        });

        let ri_skull_shadow = Rc::new(RenderItem {
            obj_cb_index: 4,
            material: Rc::clone(materials.get("shadow").unwrap()),
            ..(*ri_skull).clone()
        });

        let ri_mirror = Rc::new(RenderItem {
            world: RefCell::new(Mat4::IDENTITY),
            num_frames_dirty: Cell::new(Self::FRAME_COUNT),
            obj_cb_index: 5,
            geo: Rc::clone(geometries.get("roomGeo").unwrap()),
            material: Rc::clone(materials.get("icemirror").unwrap()),
            primitive_type: PrimitiveTopology::Triangle,
            index_count: geometries
                .get("roomGeo")
                .unwrap()
                .borrow()
                .draw_args
                .get("mirror")
                .unwrap()
                .index_count,
            start_index_location: geometries
                .get("roomGeo")
                .unwrap()
                .borrow()
                .draw_args
                .get("mirror")
                .unwrap()
                .start_index_location,
            base_vertex_location: geometries
                .get("roomGeo")
                .unwrap()
                .borrow()
                .draw_args
                .get("mirror")
                .unwrap()
                .base_vertex_location,
        });

        let ritems_by_layer = HashMap::from_iter([
            (
                RenderLayer::Opaque,
                vec![
                    Rc::clone(&ri_floor),
                    Rc::clone(&ri_walls),
                    Rc::clone(&ri_skull),
                ],
            ),
            (RenderLayer::Reflected, vec![Rc::clone(&ri_skull_reflected)]),
            (RenderLayer::Shadow, vec![Rc::clone(&ri_skull_shadow)]),
            (RenderLayer::Mirrors, vec![Rc::clone(&ri_mirror)]),
            (RenderLayer::Transparent, vec![Rc::clone(&ri_mirror)]),
        ]);

        let all_ritems = vec![
            ri_floor,
            ri_walls,
            Rc::clone(&ri_skull),
            Rc::clone(&ri_skull_reflected),
            Rc::clone(&ri_skull_shadow),
            ri_mirror,
        ];

        let frame_resources = std::array::from_fn(|_| {
            FrameResource::new(&base.device, 2, all_ritems.len(), materials.len())
        });

        let pso_desc_opaque = GraphicsPipelineDesc::new(shaders.get("standardVS").unwrap())
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

        let pso_opaque = base
            .device
            .create_graphics_pipeline(&pso_desc_opaque)
            .unwrap();

        let pso_desc = pso_desc_opaque
            .clone()
            .with_rasterizer_state(RasterizerDesc::default().with_fill_mode(FillMode::Wireframe));

        let pso_wireframe = base.device.create_graphics_pipeline(&pso_desc).unwrap();

        let pso_desc_transparent = GraphicsPipelineDesc::new(shaders.get("standardVS").unwrap())
            .with_ps(shaders.get("opaquePS").unwrap())
            .with_input_layout(&input_layout)
            .with_root_signature(&root_signature)
            .with_rasterizer_state(RasterizerDesc::default())
            .with_blend_desc(BlendDesc::default().with_render_targets([
                RenderTargetBlendDesc::blend_with_alpha(
                    Blend::SrcAlpha,
                    Blend::InvSrcAlpha,
                    BlendOp::Add,
                    Blend::One,
                    Blend::Zero,
                    BlendOp::Add,
                    ColorWriteEnable::all(),
                ),
            ]))
            .with_depth_stencil(
                DepthStencilDesc::default()
                    .enable_depth(ComparisonFunc::Less)
                    .with_depth_write_mask(DepthWriteMask::empty()),
                base.depth_stencil_format,
            )
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

        let pso_transparent = base
            .device
            .create_graphics_pipeline(&pso_desc_transparent)
            .unwrap();

        let pso_desc = pso_desc_opaque
            .clone()
            .with_blend_desc(BlendDesc::default().with_render_targets([
                RenderTargetBlendDesc::default().with_write_mask(ColorWriteEnable::empty()),
            ]))
            .with_depth_stencil(
                DepthStencilDesc::default()
                    .enable_depth(ComparisonFunc::Less)
                    .with_depth_write_mask(DepthWriteMask::empty())
                    .enable_stencil(0xFF, 0xFF)
                    .with_front_face(
                        DepthStencilOpDesc::default()
                            .with_stencil_fail_op(StencilOp::Keep)
                            .with_stencil_depth_fail_op(StencilOp::Keep)
                            .with_stencil_pass_op(StencilOp::Replace)
                            .with_stencil_func(ComparisonFunc::Always),
                    )
                    .with_back_face(
                        DepthStencilOpDesc::default()
                            .with_stencil_fail_op(StencilOp::Keep)
                            .with_stencil_depth_fail_op(StencilOp::Keep)
                            .with_stencil_pass_op(StencilOp::Replace)
                            .with_stencil_func(ComparisonFunc::Always),
                    ),
                Format::D24UnormS8Uint,
            );

        let pso_mark_mirror = base.device.create_graphics_pipeline(&pso_desc).unwrap();

        let pso_desc = pso_desc_opaque
            .clone()
            .with_blend_desc(BlendDesc::default().with_render_targets([
                RenderTargetBlendDesc::default().with_write_mask(ColorWriteEnable::empty()),
            ]))
            .with_depth_stencil(
                DepthStencilDesc::default()
                    .enable_depth(ComparisonFunc::Less)
                    .with_depth_write_mask(DepthWriteMask::All)
                    .enable_stencil(0xFF, 0xFF)
                    .with_front_face(
                        DepthStencilOpDesc::default()
                            .with_stencil_fail_op(StencilOp::Keep)
                            .with_stencil_depth_fail_op(StencilOp::Keep)
                            .with_stencil_pass_op(StencilOp::Keep)
                            .with_stencil_func(ComparisonFunc::Equal),
                    )
                    .with_back_face(
                        DepthStencilOpDesc::default()
                            .with_stencil_fail_op(StencilOp::Keep)
                            .with_stencil_depth_fail_op(StencilOp::Keep)
                            .with_stencil_pass_op(StencilOp::Keep)
                            .with_stencil_func(ComparisonFunc::Equal),
                    ),
                Format::D24UnormS8Uint,
            )
            .with_rasterizer_state(
                RasterizerDesc::default()
                    .with_cull_mode(CullMode::Back)
                    .enable_front_counter_clockwise(),
            );

        let pso_reflection = base.device.create_graphics_pipeline(&pso_desc).unwrap();

        let pso_desc = pso_desc_transparent.with_depth_stencil(
            DepthStencilDesc::default()
                .enable_depth(ComparisonFunc::Less)
                .with_depth_write_mask(DepthWriteMask::All)
                .enable_stencil(0xFF, 0xFF)
                .with_front_face(
                    DepthStencilOpDesc::default()
                        .with_stencil_fail_op(StencilOp::Keep)
                        .with_stencil_depth_fail_op(StencilOp::Keep)
                        .with_stencil_pass_op(StencilOp::Incr)
                        .with_stencil_func(ComparisonFunc::Equal),
                )
                .with_back_face(
                    DepthStencilOpDesc::default()
                        .with_stencil_fail_op(StencilOp::Keep)
                        .with_stencil_depth_fail_op(StencilOp::Keep)
                        .with_stencil_pass_op(StencilOp::Incr)
                        .with_stencil_func(ComparisonFunc::Equal),
                ),
            Format::D24UnormS8Uint,
        );

        let pso_shadow = base.device.create_graphics_pipeline(&pso_desc).unwrap();

        let pso = HashMap::from_iter([
            ("opaque".to_string(), pso_opaque),
            ("opaque_wireframe".to_string(), pso_wireframe),
            ("transparent".to_string(), pso_transparent),
            ("mark_mirror".to_string(), pso_mark_mirror),
            ("reflection".to_string(), pso_reflection),
            ("shadow".to_string(), pso_shadow),
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
            all_ritems,
            ritems_by_layer,
            geometries,
            shaders,
            materials,
            main_pass_cb: ConstantBufferData(PassConstants::default()),
            reflected_pass_cb: ConstantBufferData(PassConstants::default()),
            is_wireframe: false,
            cbv_srv_descriptor_size,
            textures,
            srv_descriptor_heap: descriptor_heap,
            skull_ritem: ri_skull,
            skull_reflected: ri_skull_reflected,
            skull_shadow: ri_skull_shadow,
            skull_translation: vec3(0.0, 1.0, -5.0),
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
        self.update_reflected_pass_cb(base);
        self.update_material_cb(base);
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
            .set_descriptor_heaps(&[Some(self.srv_descriptor_heap.clone())]);

        base.cmd_list
            .set_graphics_root_signature(Some(&self.root_signature));

        let pass_cb = self.frame_resources[self.curr_frame_resource]
            .pass_cb
            .resource();
        base.cmd_list
            .set_graphics_root_constant_buffer_view(3, pass_cb.get_gpu_virtual_address());

        self.draw_render_items(
            &base.cmd_list,
            self.ritems_by_layer.get(&RenderLayer::Opaque).unwrap(),
        );

        base.cmd_list.om_set_stencil_ref(1);
        base.cmd_list
            .set_pipeline_state(self.pso.get("mark_mirror").unwrap());
        self.draw_render_items(
            &base.cmd_list,
            self.ritems_by_layer.get(&RenderLayer::Mirrors).unwrap(),
        );

        base.cmd_list.set_graphics_root_constant_buffer_view(
            3,
            pass_cb.get_gpu_virtual_address() + size_of_val(&self.reflected_pass_cb) as u64,
        );
        base.cmd_list
            .set_pipeline_state(self.pso.get("reflection").unwrap());
        self.draw_render_items(
            &base.cmd_list,
            self.ritems_by_layer.get(&RenderLayer::Reflected).unwrap(),
        );

        base.cmd_list
            .set_graphics_root_constant_buffer_view(3, pass_cb.get_gpu_virtual_address());
        base.cmd_list.om_set_stencil_ref(0);
        base.cmd_list
            .set_pipeline_state(self.pso.get("transparent").unwrap());
        self.draw_render_items(
            &base.cmd_list,
            self.ritems_by_layer.get(&RenderLayer::Transparent).unwrap(),
        );

        base.cmd_list
            .set_pipeline_state(self.pso.get("shadow").unwrap());
        self.draw_render_items(
            &base.cmd_list,
            self.ritems_by_layer.get(&RenderLayer::Shadow).unwrap(),
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

    fn on_key_down(&mut self, base: &common::app::Base, key: winit::keyboard::KeyCode, _: bool) {
        match key {
            KeyCode::KeyW => self.skull_translation.y += 0.1 * base.timer.delta_time(),
            KeyCode::KeyS => self.skull_translation.y -= 0.1 * base.timer.delta_time(),
            KeyCode::KeyD => self.skull_translation.x += 0.1 * base.timer.delta_time(),
            KeyCode::KeyA => self.skull_translation.x -= 0.1 * base.timer.delta_time(),
            _ => {}
        }

        self.skull_translation.y = self.skull_translation.y.max(0.0);

        let skull_rotate = Mat4::from_rotation_y(0.5 * PI);
        let skull_scale = Mat4::from_scale(vec3(0.45, 0.45, 0.45));
        let skull_offset = Mat4::from_translation(self.skull_translation);

        let skull_world = skull_rotate * skull_scale * skull_offset;

        *self.skull_ritem.world.borrow_mut() = skull_world;

        let mirror_plane = vec4(0.0, 0.0, 1.0, 0.0);
        let r = Mat4::reflect(mirror_plane);

        *self.skull_reflected.world.borrow_mut() = skull_world * r;

        let shadow_plane = vec4(0.0, 1.0, 0.0, 0.0);
        let to_main_light = -self.main_pass_cb.lights[0].direction;
        let s = Mat4::shadow(
            shadow_plane,
            vec4(to_main_light.x, to_main_light.y, to_main_light.z, 1.0),
        );
        let shadow_offset_y = Mat4::from_translation(vec3(0.0, 0.001, 0.0));

        *self.skull_shadow.world.borrow_mut() = skull_world * s * shadow_offset_y;

        self.skull_ritem.num_frames_dirty.set(Self::FRAME_COUNT);
        self.skull_reflected.num_frames_dirty.set(Self::FRAME_COUNT);
        self.skull_shadow.num_frames_dirty.set(Self::FRAME_COUNT);
    }

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
            let dx = 0.2 * x;
            let dy = -0.2 * y;
            self.radius = (self.radius + dx - dy).clamp(5.0, 400.0);
        }
    }
}

impl LandAndWavesSample {
    const FRAME_COUNT: usize = 3;

    fn get_static_samplers() -> [StaticSamplerDesc; 6] {
        [
            StaticSamplerDesc::point()
                .with_address_u(AddressMode::Wrap)
                .with_address_v(AddressMode::Wrap)
                .with_address_w(AddressMode::Wrap)
                .with_shader_register(0),
            StaticSamplerDesc::point()
                .with_address_u(AddressMode::Clamp)
                .with_address_v(AddressMode::Clamp)
                .with_address_w(AddressMode::Clamp)
                .with_shader_register(1),
            StaticSamplerDesc::linear()
                .with_address_u(AddressMode::Wrap)
                .with_address_v(AddressMode::Wrap)
                .with_address_w(AddressMode::Wrap)
                .with_shader_register(2),
            StaticSamplerDesc::linear()
                .with_address_u(AddressMode::Clamp)
                .with_address_v(AddressMode::Clamp)
                .with_address_w(AddressMode::Clamp)
                .with_shader_register(3),
            StaticSamplerDesc::anisotropic()
                .with_address_u(AddressMode::Wrap)
                .with_address_v(AddressMode::Wrap)
                .with_address_w(AddressMode::Wrap)
                .with_shader_register(4)
                .with_mip_lod_bias(0.0)
                .with_max_anisotropy(8),
            StaticSamplerDesc::anisotropic()
                .with_address_u(AddressMode::Clamp)
                .with_address_v(AddressMode::Clamp)
                .with_address_w(AddressMode::Clamp)
                .with_shader_register(5)
                .with_mip_lod_bias(0.0)
                .with_max_anisotropy(8),
        ]
    }

    fn load_textures(device: &Device, cmd_list: &GraphicsCommandList) -> HashMap<String, Texture> {
        HashMap::from_iter([
            (
                "bricks".to_string(),
                load_texture_from_file(device, cmd_list, "bricks", "textures/bricks.png").unwrap(),
            ),
            (
                "checkboard".to_string(),
                load_texture_from_file(device, cmd_list, "checkboard", "textures/checkboard.png")
                    .unwrap(),
            ),
            (
                "ice".to_string(),
                load_texture_from_file(device, cmd_list, "ice", "textures/ice.png").unwrap(),
            ),
            (
                "white1x1".to_string(),
                load_texture_from_file(device, cmd_list, "white1x1", "textures/white1x1.png")
                    .unwrap(),
            ),
        ])
    }

    fn update_object_cb(&mut self, _: &common::app::Base) {
        let curr_obj_cb = &self.frame_resources[self.curr_frame_resource].object_cb;

        for e in &mut self.all_ritems {
            let num_frames_dirty = e.num_frames_dirty.get();
            if num_frames_dirty > 0 {
                curr_obj_cb.copy_data(
                    e.obj_cb_index,
                    ConstantBufferData(ObjectConstants {
                        world: *e.world.borrow(),
                        tex_transform: Mat4::IDENTITY,
                    }),
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

        self.main_pass_cb.0 = PassConstants {
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
            fog_color: vec4(0.7, 0.7, 0.7, 1.0),
            fog_start: 255.0,
            fog_range: 550.0,
            cb_per_object_pad2: Default::default(),
            ambient_light: vec4(0.25, 0.25, 0.35, 1.0),
            lights: Default::default(),
        };

        self.main_pass_cb.0.lights[0].direction = vec3(0.57735, -0.57735, 0.57735);
        self.main_pass_cb.0.lights[0].strength = vec3(0.6, 0.6, 0.6);

        self.main_pass_cb.0.lights[1].direction = vec3(-0.57735, -0.57735, 0.57735);
        self.main_pass_cb.0.lights[1].strength = vec3(0.3, 0.3, 0.3);

        self.main_pass_cb.0.lights[2].direction = vec3(0.0, -0.707, -0.707);
        self.main_pass_cb.0.lights[2].strength = vec3(0.15, 0.15, 0.15);

        self.frame_resources[self.curr_frame_resource]
            .pass_cb
            .copy_data(0, self.main_pass_cb);
    }

    fn update_reflected_pass_cb(&mut self, _: &common::app::Base) {
        self.reflected_pass_cb = self.main_pass_cb;

        let mirror_plane = vec4(0.0, 0.0, 1.0, 0.0);

        let r = Mat4::reflect(mirror_plane);

        for i in 0..3 {
            let light_dir = self.main_pass_cb.0.lights[i].direction;
            let reflected_light_dir = r.transform_vector3(light_dir).normalize();
            self.reflected_pass_cb.0.lights[i].direction = reflected_light_dir;
        }

        self.frame_resources[self.curr_frame_resource]
            .pass_cb
            .copy_data(1, self.reflected_pass_cb);
    }

    fn update_material_cb(&mut self, _: &common::app::Base) {
        let curr_obj_cb = &self.frame_resources[self.curr_frame_resource].material_cb;

        for e in &mut self.materials.values_mut() {
            let mut e = e.borrow_mut();
            if e.num_frames_dirty > 0 {
                curr_obj_cb.copy_data(
                    e.cb_index,
                    ConstantBufferData(MaterialConstant {
                        diffuse_albedo: e.diffuse_albedo,
                        fresnel_r0: e.fresnel_r0,
                        roughness: e.roughness,
                        transform: e.transform,
                    }),
                );
                e.num_frames_dirty -= 1;
            }
        }
    }

    fn build_room_geometry(device: &Device, cmd_list: &GraphicsCommandList) -> MeshGeometry {
        let vertices = [
            Vertex::new(-3.5, 0.0, -10.0, 0.0, 1.0, 0.0, 0.0, 4.0), // 0
            Vertex::new(-3.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0),
            Vertex::new(7.5, 0.0, 0.0, 0.0, 1.0, 0.0, 4.0, 0.0),
            Vertex::new(7.5, 0.0, -10.0, 0.0, 1.0, 0.0, 4.0, 4.0),
            // Wall: Observe we tile texture coordinates, and that we
            // leave a gap in the middle for the mirror.
            Vertex::new(-3.5, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 2.0), // 4
            Vertex::new(-3.5, 4.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0),
            Vertex::new(-2.5, 4.0, 0.0, 0.0, 0.0, -1.0, 0.5, 0.0),
            Vertex::new(-2.5, 0.0, 0.0, 0.0, 0.0, -1.0, 0.5, 2.0),
            Vertex::new(2.5, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 2.0), // 8
            Vertex::new(2.5, 4.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0),
            Vertex::new(7.5, 4.0, 0.0, 0.0, 0.0, -1.0, 2.0, 0.0),
            Vertex::new(7.5, 0.0, 0.0, 0.0, 0.0, -1.0, 2.0, 2.0),
            Vertex::new(-3.5, 4.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0), // 12
            Vertex::new(-3.5, 6.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0),
            Vertex::new(7.5, 6.0, 0.0, 0.0, 0.0, -1.0, 6.0, 0.0),
            Vertex::new(7.5, 4.0, 0.0, 0.0, 0.0, -1.0, 6.0, 1.0),
            // Mirror
            Vertex::new(-2.5, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0), // 16
            Vertex::new(-2.5, 4.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0),
            Vertex::new(2.5, 4.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0),
            Vertex::new(2.5, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 1.0),
        ];

        let indices = [
            0u16, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16,
            17, 18, 16, 18, 19,
        ];

        let vertex_buffer_cpu = Blob::create_blob(size_of_val(vertices.as_slice())).unwrap();
        let index_buffer_cpu = Blob::create_blob(size_of_val(indices.as_slice())).unwrap();

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
            name: "roomGeo".to_string(),
            vertex_buffer_cpu,
            index_buffer_cpu,
            vertex_buffer_gpu: Some(vertex_buffer_gpu),
            index_buffer_gpu: Some(index_buffer_gpu),
            vertex_buffer_uploader: Some(vertex_buffer_uploader),
            index_buffer_uploader: Some(index_buffer_uploader),
            vertex_byte_stride: size_of::<Vertex>() as u32,
            vertex_byte_size: size_of_val(vertices.as_slice()) as u32,
            index_format: Format::R16Uint,
            index_buffer_byte_size: size_of_val(indices.as_slice()) as u32,
            draw_args: HashMap::from_iter([
                (
                    "floor".to_string(),
                    SubmeshGeometry {
                        index_count: 6,
                        start_index_location: 0,
                        base_vertex_location: 0,
                        bounds: BoundingBox::default(),
                    },
                ),
                (
                    "wall".to_string(),
                    SubmeshGeometry {
                        index_count: 18,
                        start_index_location: 6,
                        base_vertex_location: 0,
                        bounds: BoundingBox::default(),
                    },
                ),
                (
                    "mirror".to_string(),
                    SubmeshGeometry {
                        index_count: 6,
                        start_index_location: 24,
                        base_vertex_location: 0,
                        bounds: BoundingBox::default(),
                    },
                ),
            ]),
        }
    }

    fn build_skull_geometry(device: &Device, cmd_list: &GraphicsCommandList) -> MeshGeometry {
        let file = std::fs::File::open("models/skull.txt").unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut vcount = 0;
        let mut tcount = 0;

        lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .map(|v| vcount = v)
            .unwrap();
        lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .map(|t| tcount = t)
            .unwrap();

        for _ in 0..2 {
            lines.next();
        }

        let mut vertices = vec![];
        let mut indices = vec![];

        for _ in 0..vcount {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();

            vertices.push(Vertex {
                pos: vec3(
                    parts.next().and_then(|s| s.parse::<f32>().ok()).unwrap(),
                    parts.next().and_then(|s| s.parse::<f32>().ok()).unwrap(),
                    parts.next().and_then(|s| s.parse::<f32>().ok()).unwrap(),
                ),
                normal: vec3(
                    parts.next().and_then(|s| s.parse::<f32>().ok()).unwrap(),
                    parts.next().and_then(|s| s.parse::<f32>().ok()).unwrap(),
                    parts.next().and_then(|s| s.parse::<f32>().ok()).unwrap(),
                ),
                uv: Vec2::ZERO,
            });
        }

        for _ in 0..2 {
            lines.next();
        }

        for _ in 0..tcount {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            indices.push(parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap());
            indices.push(parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap());
            indices.push(parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap());
        }

        let vertex_buffer_cpu = Blob::create_blob(size_of_val(vertices.as_slice())).unwrap();
        let index_buffer_cpu = Blob::create_blob(size_of_val(indices.as_slice())).unwrap();

        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices.as_ptr(),
                vertex_buffer_cpu.get_buffer_ptr::<Vertex>().as_mut(),
                vertices.len(),
            );
            std::ptr::copy_nonoverlapping(
                indices.as_ptr(),
                index_buffer_cpu.get_buffer_ptr::<u32>().as_mut(),
                indices.len(),
            );
        }

        let (vertex_buffer_gpu, vertex_buffer_uploader) =
            create_default_buffer(device, cmd_list, &vertices);
        let (index_buffer_gpu, index_buffer_uploader) =
            create_default_buffer(device, cmd_list, &indices);

        MeshGeometry {
            name: "skullGeo".to_string(),
            vertex_buffer_cpu,
            index_buffer_cpu,
            vertex_buffer_gpu: Some(vertex_buffer_gpu),
            index_buffer_gpu: Some(index_buffer_gpu),
            vertex_buffer_uploader: Some(vertex_buffer_uploader),
            index_buffer_uploader: Some(index_buffer_uploader),
            vertex_byte_stride: size_of::<Vertex>() as u32,
            vertex_byte_size: size_of_val(vertices.as_slice()) as u32,
            index_format: Format::R32Uint,
            index_buffer_byte_size: size_of_val(indices.as_slice()) as u32,
            draw_args: HashMap::from_iter([(
                "skull".to_string(),
                SubmeshGeometry {
                    index_count: indices.len() as u32,
                    start_index_location: 0,
                    base_vertex_location: 0,
                    bounds: BoundingBox::default(),
                },
            )]),
        }
    }

    fn draw_render_items(&self, cmd_list: &GraphicsCommandList, ritems: &[Rc<RenderItem>]) {
        let obj_size = size_of::<ConstantBufferData<ObjectConstants>>();
        let obj_cb = self.frame_resources[self.curr_frame_resource]
            .object_cb
            .resource();

        let mat_size = size_of::<ConstantBufferData<MaterialConstant>>();
        let mat_cb = self.frame_resources[self.curr_frame_resource]
            .material_cb
            .resource();

        for item in ritems {
            cmd_list.ia_set_vertex_buffers(0, &[item.geo.borrow().vertex_buffer_view()]);
            cmd_list.ia_set_index_buffer(Some(&item.geo.borrow().index_buffer_view()));
            cmd_list.ia_set_primitive_topology(item.primitive_type);

            let tex = self
                .srv_descriptor_heap
                .get_gpu_descriptor_handle_for_heap_start();
            let tex = tex.advance(
                item.material.borrow().diffuse_srv_heap_index.unwrap(),
                self.cbv_srv_descriptor_size,
            );
            cmd_list.set_graphics_root_descriptor_table(0, tex);

            let obj_addr = obj_cb.get_gpu_virtual_address() + (item.obj_cb_index * obj_size) as u64;
            cmd_list.set_graphics_root_constant_buffer_view(1, obj_addr);

            let mat_addr = mat_cb.get_gpu_virtual_address()
                + (item.material.borrow().cb_index * mat_size) as u64;
            cmd_list.set_graphics_root_constant_buffer_view(2, mat_addr);

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
