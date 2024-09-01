mod frame_resources;
mod render_item;

use std::{mem::offset_of, rc::Rc};

use common::{
    app::{DxSample, SwapchainContext},
    run_sample,
    utils::{ConstantBufferData, VertexAttr},
};
use frame_resources::{FrameResource, ObjectConstants, PassConstants};
use glam::{vec2, Mat4, Vec3, Vec4};
use oxidx::dx::*;
use render_item::RenderItem;
use tracing_subscriber::layer::SubscriberExt;

fn main() {
    let console_log = tracing_subscriber::fmt::Layer::new()
        .with_ansi(true)
        .with_writer(std::io::stdout);

    let subscriber = tracing_subscriber::registry().with(console_log);

    let _ = tracing::subscriber::set_global_default(subscriber);
    run_sample::<ShapesSample>();
}

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

    vs_byte_code: Blob,
    ps_byte_code: Blob,

    pso: PipelineState,

    eye_pos: Vec3,
    view: Mat4,
    proj: Mat4,

    theta: f32,
    phi: f32,
    radius: f32,

    is_lmb_pressed: bool,
    is_rmb_pressed: bool,
}

impl DxSample for ShapesSample {
    fn new(base: &mut common::app::Base) -> Self {
        base.cmd_list.reset(&base.cmd_list_alloc, PSO_NONE).unwrap();

        let cbv_heap: DescriptorHeap = base
            .device
            .create_descriptor_heap(
                &DescriptorHeapDesc::cbr_srv_uav(2).with_flags(DescriptorHeapFlags::ShaderVisible),
            )
            .unwrap();

        let frame_resources = std::array::from_fn(|_| FrameResource::new(&base.device, 1, 1));

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
            frame_resources,
            curr_frame_resource: 0,
            vs_byte_code,
            ps_byte_code,
            pso,
            eye_pos: Vec3::ZERO,
            view: Mat4::IDENTITY,
            proj: Mat4::IDENTITY,
            theta: 0.0,
            phi: 0.0,
            radius: 5.0,
            is_lmb_pressed: false,
            is_rmb_pressed: false,
            all_ritems: vec![],
            opaque_ritems: vec![],
            transparent_ritems: vec![],
        }
    }

    fn init_resources(&mut self, _: &common::app::Base) {}

    fn update(&mut self, base: &common::app::Base) {
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
            .ia_set_primitive_topology(PrimitiveTopology::Triangle);

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
                .with_offset(offset_of!(Vertex, color) as u32),
        ]
    }
}
