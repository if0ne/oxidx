use smallvec::SmallVec;
use windows::{
    core::Interface,
    Win32::Graphics::{
        Direct3D12::{
            ID3D12Device, D3D12_BLEND_DESC, D3D12_FENCE_FLAG_NONE,
            D3D12_GRAPHICS_PIPELINE_STATE_DESC, D3D12_INPUT_LAYOUT_DESC,
            D3D12_PIPELINE_STATE_FLAGS, D3D12_RENDER_TARGET_BLEND_DESC, D3D12_STREAM_OUTPUT_DESC,
        },
        Dxgi::Common::DXGI_FORMAT,
    },
};

use crate::{
    command_allocator::CommandAllocatorInterface,
    command_list::CommandListInterface,
    command_queue::CommandQueueInterface,
    command_signature::CommandSignatureInterface,
    create_type,
    descriptor_heap::DescriptorHeapInterface,
    error::DxError,
    impl_trait,
    pso::{
        BlobInterface, GraphicsPipelineDesc, PipelineStateInterface, RootSignatureDesc,
        RootSignatureInterface,
    },
    resources::{RenderTargetViewDesc, ResourceDesc, ResourceInterface, ResourceState},
    sync::FenceInterface,
    types::*,
    FeatureObject, HasInterface,
};

/// Represents a virtual adapter; it is used to create
/// * command allocators
/// * command lists
/// * command queues
/// * fences
/// * resources
/// * pipeline state objects,
/// * heaps
/// * root signatures
/// * samplers
/// * and many resource views.
///
/// For more information: [`ID3D12Device interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12device)
pub trait DeviceInterface: HasInterface<Raw: Interface> {
    /// Gets information about the features that are supported by the current graphics driver.
    ///
    /// # Arguments
    /// * `feature` - A input data structure for type that implement [`FeatureObject`].
    ///
    /// # Returns
    /// A output data structure for type that implement [`FeatureObject`].
    ///
    /// For more information: [`ID3D12Device::CheckFeatureSupport method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-checkfeaturesupport)
    fn check_feature_support<F: FeatureObject>(
        &self,
        feature_input: F::Input<'_>,
    ) -> Result<F::Output, DxError>;

    /// Copies descriptors from a source to a destination.
    ///
    /// # Arguments
    /// * `dest_descriptor_range_starts` - An array of [`CpuDescriptorHandle`] objects to copy to.
    /// * `dest_descriptor_range_sizes` - An array of destination descriptor range sizes to copy to.
    /// * `src_descriptor_range_starts` - An array of [`CpuDescriptorHandle`] objects to copy from.
    /// * `src_descriptor_range_sizes` - An array of source  descriptor range sizes to copy from.
    /// * `descriptor_heaps_type` - The [`DescriptorHeapType`]-typed value that specifies the type of descriptor heap to copy with. This is required as different descriptor types may have different sizes.
    ///
    /// For more information: [`ID3D12Device::CopyDescriptors method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-copydescriptors)
    fn copy_descriptors<'a>(
        &self,
        dest_descriptor_range_starts: &'a [CpuDescriptorHandle],
        dest_descriptor_range_sizes: Option<&'a [u32]>,
        src_descriptor_range_starts: &'a [CpuDescriptorHandle],
        src_descriptor_range_sizes: Option<&'a [u32]>,
        descriptor_heaps_type: DescriptorHeapType,
    );

    /// Copies descriptors from a source to a destination.
    ///
    /// # Arguments
    /// * `num_descriptors` - The number of descriptors to copy.
    /// * `dest_descriptor_range_start` - A [`CpuDescriptorHandle`] that describes the destination descriptors to start to copy to.
    /// * `src_descriptor_range_start` - A [`CpuDescriptorHandle`] that describes the source descriptors to start to copy from.
    /// * `descriptor_heaps_type` - The [`DescriptorHeapType`]-typed value that specifies the type of descriptor heap to copy with. This is required as different descriptor types may have different sizes.
    ///
    /// For more information: [`ID3D12Device::CopyDescriptorsSimple method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-copydescriptorssimple)
    fn copy_descriptors_simple(
        &self,
        num_descriptors: u32,
        dest_descriptor_range_start: CpuDescriptorHandle,
        src_descriptor_range_start: CpuDescriptorHandle,
        descriptor_heaps_type: DescriptorHeapType,
    );

    /// Creates a command allocator object.
    ///
    /// # Arguments
    /// * `type` - A [`CommandListType`]-typed value that specifies the type of command allocator to create. The type of command allocator can be the type that records either direct command lists or bundles.
    ///
    /// For more information: [`ID3D12Device::CreateCommandAllocator method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcommandallocator)
    fn create_command_allocator<CA: CommandAllocatorInterface>(
        &self,
        r#type: CommandListType,
    ) -> Result<CA, DxError>;

    /// Creates a command list.
    ///
    /// # Arguments
    /// * `node_mask` - For single-GPU operation, set this to zero. If there are multiple GPU nodes, then set a bit to identify the node (the device's physical adapter) for which to create the command list. Each bit in the mask corresponds to a single node. Only one bit must be set.
    /// * `type` - Specifies the type of command list to create.
    /// * `command_allocator` - A reference to the command allocator object from which the device creates command lists.
    /// * `initial_state` - An optional pointer to the pipeline state object that contains the initial pipeline state for the command list.
    ///   If it is nullptr, then the runtime sets a dummy initial pipeline state, so that drivers don't have to deal with undefined state.
    ///   The overhead for this is low, particularly for a command list, for which the overall cost of recording the command list likely dwarfs the cost of a single initial state
    ///   setting. So there's little cost in not setting the initial pipeline state parameter, if doing so is inconvenient.
    ///
    /// For more information: [`ID3D12Device::CreateCommandList method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcommandlist)
    fn create_command_list<CL: CommandListInterface>(
        &self,
        node_mask: u32,
        r#type: CommandListType,
        command_allocator: &impl CommandAllocatorInterface,
        pso: Option<&impl PipelineStateInterface>,
    ) -> Result<CL, DxError>;

    /// Creates a command queue.
    ///
    /// # Arguments
    /// * `desc` - Specifies a [`CommandQueueDesc`] that describes the command queue.
    ///
    /// For more information: [`ID3D12Device::CreateCommandQueue method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcommandqueue)
    fn create_command_queue<CQ: CommandQueueInterface>(
        &self,
        desc: CommandQueueDesc,
    ) -> Result<CQ, DxError>;

    /// Creates a command queue.
    ///
    /// # Arguments
    /// * `desc` - Describes the command signature to be created with the [`CommandSignatureDesc`] structure.
    /// * `root_signature` - Specifies the [`RootSignatureInterface`] that the command signature applies to.
    ///   The root signature is required if any of the commands in the signature will update bindings on the pipeline.
    ///   If the only command present is a draw or dispatch, the root signature parameter can be set to None.
    ///
    /// For more information: [`ID3D12Device::CreateCommandSignature method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcommandsignature)
    fn create_command_signature<CS: CommandSignatureInterface>(
        &self,
        desc: &CommandSignatureDesc<'_>,
        root_signature: Option<&impl RootSignatureInterface>,
    ) -> Result<CS, DxError>;

    fn create_fence<F: FenceInterface>(&self, initial_value: u64) -> Result<F, DxError>;

    fn create_descriptor_heap<H: DescriptorHeapInterface>(
        &self,
        desc: DescriptorHeapDesc,
    ) -> Result<H, DxError>;

    fn get_descriptor_handle_increment_size(&self, r#type: DescriptorHeapType) -> u32;

    fn create_render_target_view(
        &self,
        resource: &impl ResourceInterface,
        view_desc: Option<&RenderTargetViewDesc>,
        handle: CpuDescriptorHandle,
    );

    fn create_root_signature<RS: RootSignatureInterface>(
        &self,
        node_mask: u32,
        blob: &[u8],
    ) -> Result<RS, DxError>;

    fn serialize_create_root_signature<RS: RootSignatureInterface>(
        &self,
        desc: &RootSignatureDesc<'_>,
        version: RootSignatureVersion,
        node_mask: u32,
    ) -> Result<RS, DxError>;

    fn create_graphics_pipeline<G: PipelineStateInterface>(
        &self,
        desc: &GraphicsPipelineDesc<'_>,
    ) -> Result<G, DxError>;

    fn create_committed_resource<R: ResourceInterface>(
        &self,
        heap_properties: HeapProperties,
        heap_flags: HeapFlags,
        desc: ResourceDesc,
        init_state: ResourceState,
        optimized_clear_value: Option<ClearValue>,
    ) -> Result<R, DxError>;
}

create_type! {
    /// Represents a virtual adapter; it is used to create
    /// * command allocators
    /// * command lists
    /// * command queues
    /// * fences
    /// * resources
    /// * pipeline state objects,
    /// * heaps
    /// * root signatures
    /// * samplers
    /// * and many resource views.
    ///
    /// For more information: [`ID3D12Device interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12device)
    Device wrap ID3D12Device
}

impl_trait! {
    impl DeviceInterface =>
    Device;

    fn check_feature_support<F: FeatureObject>(&self, feature_input: F::Input<'_>) -> Result<F::Output, DxError> {
        unsafe {
            let mut raw = F::into_raw(feature_input);

            self.0
                .CheckFeatureSupport(
                    F::TYPE.as_raw(),
                    &mut raw as *mut F::Raw as *mut _,
                    core::mem::size_of::<F::Raw>() as u32,
                )
                .map_err(DxError::from)?;

            Ok(F::from_raw(raw))
        }
    }

    fn copy_descriptors<'a>(
        &self,
        dest_descriptor_range_starts: &'a [CpuDescriptorHandle],
        dest_descriptor_range_sizes: Option<&'a [u32]>,
        src_descriptor_range_starts: &'a [CpuDescriptorHandle],
        src_descriptor_range_sizes: Option<&'a [u32]>,
        descriptor_heaps_type: DescriptorHeapType,
    ) {
        unsafe {
            let dest_num = dest_descriptor_range_starts.len() as u32;
            let dest_descriptor_range_starts = dest_descriptor_range_starts.as_ptr() as *const _;
            let dest_descriptor_range_sizes = dest_descriptor_range_sizes.map(|r| r.as_ptr());
            let src_num = src_descriptor_range_starts.len() as u32;
            let src_descriptor_range_starts = src_descriptor_range_starts.as_ptr() as *const _;
            let src_descriptor_range_sizes = src_descriptor_range_sizes.map(|r| r.as_ptr());
            let descriptor_heaps_type = descriptor_heaps_type.as_raw();

            self.0.CopyDescriptors(
                dest_num,
                dest_descriptor_range_starts,
                dest_descriptor_range_sizes,
                src_num,
                src_descriptor_range_starts,
                src_descriptor_range_sizes,
                descriptor_heaps_type
            );
        }
    }

    fn copy_descriptors_simple(
        &self,
        num_descriptors: u32,
        dest_descriptor_range_start: CpuDescriptorHandle,
        src_descriptor_range_start: CpuDescriptorHandle,
        descriptor_heaps_type: DescriptorHeapType,
    ) {
        unsafe {
            self.0.CopyDescriptorsSimple(
                num_descriptors,
                dest_descriptor_range_start.as_raw(),
                src_descriptor_range_start.as_raw(),
                descriptor_heaps_type.as_raw()
            );
        }
    }

    fn create_command_allocator<CA: CommandAllocatorInterface>(
        &self,
        r#type: CommandListType
    ) -> Result<CA, DxError> {
        unsafe {
            let res: CA::Raw = self.0.CreateCommandAllocator(r#type.as_raw()).map_err(DxError::from)?;

            Ok(CA::new(res))
        }
    }

    fn create_command_queue<CQ: CommandQueueInterface>(
        &self,
        desc: CommandQueueDesc,
    ) -> Result<CQ, DxError> {
        unsafe {
            let res: CQ::Raw = self.0.CreateCommandQueue(&desc.as_raw()).map_err(DxError::from)?;

            Ok(CQ::new(res))
        }
    }

    fn create_command_signature<CS: CommandSignatureInterface>(
        &self,
        desc: &CommandSignatureDesc<'_>,
        root_signature: Option<&impl RootSignatureInterface>,
    ) -> Result<CS, DxError> {
        unsafe {
            let desc = desc.as_raw();
            let mut res: Option<CS::Raw> = None;

            if let Some(root) = root_signature {
                self.0.CreateCommandSignature(&desc, root.as_raw_ref(), &mut res).map_err(DxError::from)?;
            } else {
                self.0.CreateCommandSignature(&desc, None, &mut res).map_err(DxError::from)?;
            };

            let res = res.unwrap();

            Ok(CS::new(res))
        }
    }

    fn create_fence<F: FenceInterface>(
        &self,
        initial_value: u64,
    ) -> Result<F, DxError> {
        let res: F::Raw  = unsafe {
            self.0.CreateFence(initial_value, D3D12_FENCE_FLAG_NONE).map_err(|_| DxError::Dummy)?
        };

        Ok(F::new(res))
    }

    fn create_descriptor_heap<H: DescriptorHeapInterface>(
        &self,
        desc: DescriptorHeapDesc,
    ) -> Result<H, DxError> {
        let res: H::Raw  = unsafe {
            self.0.CreateDescriptorHeap(&desc.as_raw()).map_err(|_| DxError::Dummy)?
        };

        Ok(H::new(res))
    }

    fn get_descriptor_handle_increment_size(&self, r#type: DescriptorHeapType) -> u32 {
        unsafe {
            self.0.GetDescriptorHandleIncrementSize(r#type.as_raw())
        }
    }

    fn create_render_target_view(&self, resource: &impl ResourceInterface, view_desc: Option<&RenderTargetViewDesc>, handle: CpuDescriptorHandle) {
        let desc = view_desc.map(|v| v.as_raw());
        let desc = desc.as_ref().map(|f| f as *const _);

        unsafe {
            self.0.CreateRenderTargetView(resource.as_raw_ref(), desc, handle.as_raw());
        }
    }

    fn create_command_list<CL: CommandListInterface>(
        &self,
        node_mask: u32,
        r#type: CommandListType,
        command_allocator: &impl CommandAllocatorInterface,
        pso: Option<&impl PipelineStateInterface>,
    ) -> Result<CL, DxError> {
        let res: CL::Raw = unsafe {
            if let Some(pso) = pso {
                self.0.CreateCommandList(
                    node_mask,
                    r#type.as_raw(),
                    command_allocator.as_raw_ref(),
                    pso.as_raw_ref()
                ).map_err(|_| DxError::Dummy)?
            } else {
                self.0.CreateCommandList(
                    node_mask,
                    r#type.as_raw(),
                    command_allocator.as_raw_ref(),
                    None
                ).map_err(|_| DxError::Dummy)?
            }

        };

        Ok(CL::new(res))
    }

    fn create_root_signature<RS: RootSignatureInterface>(&self, node_mask: u32, blob: &[u8]) -> Result<RS, DxError> {
        let res: RS::Raw = unsafe {
            self.0
                .CreateRootSignature(
                    node_mask,
                    blob,
                )
                .map_err(|_| DxError::Dummy)?
        };

        Ok(RS::new(res))
    }

    fn serialize_create_root_signature<RS: RootSignatureInterface>(
        &self,
        desc: &RootSignatureDesc<'_>,
        version: RootSignatureVersion,
        node_mask: u32,
    ) -> Result<RS, DxError> {
        let blob = RS::serialize(desc, version)?;
        unsafe {
        self.create_root_signature(node_mask, std::slice::from_raw_parts(
                blob.get_buffer_ptr() as _,
                blob.get_buffer_size(),
            ))
        }
    }

    fn create_graphics_pipeline<G: PipelineStateInterface>(
        &self,
        desc: &GraphicsPipelineDesc<'_>,
    ) -> Result<G, DxError> {
        let mut rtv_formats = [DXGI_FORMAT::default(); 8];

        for (i, format) in desc.rtv_formats.iter().enumerate() {
            rtv_formats[i] = format.as_raw();
        }

        let input_layouts = desc
            .input_layout
            .iter()
            .map(|il| il.as_raw())
            .collect::<SmallVec<[_; 8]>>();

        let so_entries: SmallVec<[_; 8]> = if let Some(ref so) = desc.stream_output {
            so.entries.iter().map(|e| e.as_raw()).collect()
        } else {
            smallvec::smallvec![]
        };

        let mut rtv_blend = [D3D12_RENDER_TARGET_BLEND_DESC::default(); 8];

        for (i, desc) in desc.blend_state.render_targets.iter().enumerate() {
            rtv_blend[i] = desc.as_raw();
        }

        let desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            pRootSignature: unsafe { std::mem::transmute_copy(desc.root_signature.as_raw_ref()) },
            VS: desc.vs.as_raw(),
            PS: desc.ps.map(|ps| ps.as_raw()).unwrap_or_default(),
            DS: desc.ds.map(|ds| ds.as_raw()).unwrap_or_default(),
            HS: desc.hs.map(|hs| hs.as_raw()).unwrap_or_default(),
            GS: desc.gs.map(|gs| gs.as_raw()).unwrap_or_default(),
            StreamOutput: desc
                .stream_output
                .as_ref()
                .map(|so| D3D12_STREAM_OUTPUT_DESC {
                    pSODeclaration: so_entries.as_ptr() as *const _,
                    NumEntries: so_entries.len() as u32,
                    pBufferStrides: so.buffer_strides.as_ptr() as *const _,
                    NumStrides: so.buffer_strides.len() as u32,
                    RasterizedStream: so.rasterized_stream,
                })
                .unwrap_or_default(),
            BlendState: D3D12_BLEND_DESC {
                AlphaToCoverageEnable: desc.blend_state.alpha_to_coverage_enable.into(),
                IndependentBlendEnable: desc.blend_state.independent_blend_enable.into(),
                RenderTarget: rtv_blend,
            },
            SampleMask: desc.sample_mask,
            RasterizerState: desc.rasterizer_state.as_raw(),
            DepthStencilState: desc
                .depth_stencil
                .as_ref()
                .map(|ds| ds.as_raw())
                .unwrap_or_default(),
            InputLayout: D3D12_INPUT_LAYOUT_DESC {
                pInputElementDescs: input_layouts.as_ptr() as *const _,
                NumElements: input_layouts.len() as u32,
            },
            IBStripCutValue: desc
                .ib_strip_cut_value
                .map(|ib| ib.as_raw())
                .unwrap_or_default(),
            PrimitiveTopologyType: desc.primitive_topology.as_raw(),
            NumRenderTargets: desc.rtv_formats.len() as u32,
            RTVFormats: rtv_formats,
            DSVFormat: desc.dsv_format.map(|f| f.as_raw()).unwrap_or_default(),
            SampleDesc: desc.sampler_desc.as_raw(),
            NodeMask: desc.node_mask,
            CachedPSO: desc
                .cached_pso
                .as_ref()
                .map(|pso| pso.as_raw())
                .unwrap_or_default(),
            Flags: D3D12_PIPELINE_STATE_FLAGS(desc.flags.bits()),
        };

        let res: G::Raw = unsafe {
            self.0
                .CreateGraphicsPipelineState(&desc)
                .map_err(|_| DxError::Dummy)?
        };

        Ok(G::new(res))
    }

    fn create_committed_resource<R: ResourceInterface>(
        &self,
        heap_properties: HeapProperties,
        heap_flags: HeapFlags,
        desc: ResourceDesc,
        init_state: ResourceState,
        optimized_clear_value: Option<ClearValue>,
    ) -> Result<R, DxError> {
        let clear_value = optimized_clear_value.as_ref().map(|c| c.as_raw());
        let clear_value = clear_value.as_ref().map(|c| c as *const _);

        let mut resource = None;
        unsafe {
            self.0.CreateCommittedResource(
                &heap_properties.as_raw(),
                heap_flags.as_raw(),
                &desc.as_raw(),
                init_state.as_raw(),
                clear_value,
                &mut resource,
            ).map_err(|_| DxError::Dummy)?;
        }
        let resource = resource.unwrap();

        Ok(R::new(resource))
    }
}
