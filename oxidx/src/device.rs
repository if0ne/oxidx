use windows::{core::Interface, Win32::Graphics::Direct3D12::ID3D12Device};

use crate::{
    blob::BlobInterface, command_allocator::CommandAllocatorInterface,
    command_list::CommandListInterface, command_queue::CommandQueueInterface,
    command_signature::CommandSignatureInterface, create_type,
    descriptor_heap::DescriptorHeapInterface, error::DxError, heap::HeapInterface, impl_trait,
    pso::PipelineStateInterface, query_heap::QueryHeapInterface, resources::ResourceInterface,
    root_signature::RootSignatureInterface, sync::FenceInterface, types::*, FeatureObject,
    HasInterface,
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

    /// Creates both a resource and an implicit heap, such that the heap is big enough to contain the entire resource, and the resource is mapped to the heap.
    ///
    /// # Arguments
    /// * `heap_properties` - A reference to a [`HeapProperties`] structure that provides properties for the resource's heap.
    /// * `heap_flags` - Heap options, as a bitwise-OR'd combination of [`HeapFlags`] enumeration constants.
    /// * `desc` - A reference to a [`ResourceDesc`] structure that describes the resource.
    /// * `initial_state` - The initial state of the resource, as a bitwise-OR'd combination of [`ResourceState`] enumeration constants.
    /// * `optimized_clear_value` - Specifies a [`ClearValue`] structure that describes the default value for a clear color.
    ///
    /// For more information: [`ID3D12Device::CreateCommittedResource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcommittedresource)
    fn create_committed_resource<R: ResourceInterface>(
        &self,
        heap_properties: &HeapProperties,
        heap_flags: HeapFlags,
        desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> Result<R, DxError>;

    /// Creates a compute pipeline state object.
    ///
    /// # Arguments
    /// * `desc` - A reference to a [`ComputePipelineStateDesc`] structure that describes compute pipeline state.
    ///
    /// For more information: [`ID3D12Device::CreateComputePipelineState method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcomputepipelinestate)
    fn create_compute_pipeline_state<CPS: PipelineStateInterface>(
        &self,
        desc: &ComputePipelineStateDesc<'_>,
    ) -> Result<CPS, DxError>;

    /// Creates a constant-buffer view for accessing resource data.
    ///
    /// # Arguments
    /// * `desc` - A reference to a [`ConstantBufferViewDesc`] structure that describes the constant-buffer view.
    /// * `dest_descriptor` - Describes the CPU descriptor handle that represents the start of the heap that holds the constant-buffer view.
    ///
    /// For more information: [`ID3D12Device::CreateConstantBufferView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createconstantbufferview)
    fn create_constant_buffer_view(
        &self,
        desc: Option<&ConstantBufferViewDesc>,
        dest_descriptor: CpuDescriptorHandle,
    );

    /// Creates a depth-stencil view for accessing resource data.
    ///
    /// # Arguments
    /// * `resource` - A reference to the [`ResourceInterface`] object that represents the depth stencil.
    /// * `desc` - A reference to a [`ConstantBufferViewDesc`] structure that describes the constant-buffer view.
    /// * `dest_descriptor` - Describes the CPU descriptor handle that represents the start of the heap that holds the constant-buffer view.
    ///
    /// For more information: [`ID3D12Device::CreateDepthStencilView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createdepthstencilview)
    fn create_depth_stencil_view(
        &self,
        resource: Option<&impl ResourceInterface>,
        desc: Option<&DepthStencilViewDesc>,
        dest_descriptor: CpuDescriptorHandle,
    );

    /// Creates a descriptor heap object.
    ///
    /// # Arguments
    /// * `desc` - A reference to a [`DescriptorHeapDesc`] structure that describes the heap.
    ///
    /// For more information: [`ID3D12Device::CreateDescriptorHeap method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createdescriptorheap)
    fn create_descriptor_heap<H: DescriptorHeapInterface>(
        &self,
        desc: &DescriptorHeapDesc,
    ) -> Result<H, DxError>;

    /// Creates a fence object.
    ///
    /// # Arguments
    /// * `initial_value` - The initial value for the fence.
    /// * `flags` - A combination of [`FenceFlags`]-typed values that are combined by using a bitwise OR operation. The resulting value specifies options for the fence.
    ///
    /// For more information: [`ID3D12Device::CreateFence method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createfence)
    fn create_fence<F: FenceInterface>(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> Result<F, DxError>;

    /// Creates a graphics pipeline state object.
    ///
    /// # Arguments
    /// * `desc` - A reference to a [`GraphicsPipelineDesc`] structure that describes graphics pipeline state.
    ///
    /// For more information: [`ID3D12Device::CreateGraphicsPipelineState method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-creategraphicspipelinestate)
    fn create_graphics_pipeline<G: PipelineStateInterface>(
        &self,
        desc: &GraphicsPipelineDesc<'_>,
    ) -> Result<G, DxError>;

    /// Creates a heap that can be used with placed resources and reserved resources.
    ///
    /// # Arguments
    /// * `desc` - A reference to a constant [`HeapDesc`] structure that describes the heap.
    ///
    /// For more information: [`ID3D12Device::CreateHeap method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createheap)
    fn create_heap<H: HeapInterface>(&self, desc: &HeapDesc) -> Result<H, DxError>;

    /// Creates a resource that is placed in a specific heap. Placed resources are the lightest weight resource objects available, and are the fastest to create and destroy.
    ///
    /// # Arguments
    /// * `heap` - A reference to the [`HeapInterface`] interface that represents the heap in which the resource is placed.
    /// * `heap_offset` - The offset, in bytes, to the resource.
    /// * `desc` - A reference to a [`ResourceDesc`] structure that describes the resource.
    /// * `initial_state` - The initial state of the resource, as a bitwise-OR'd combination of [`ResourceStates`] enumeration constants.
    /// * `optimized_clear_value` - Specifies a [`ClearValue`] that describes the default value for a clear color.
    ///
    /// For more information: [`ID3D12Device::CreatePlacedResource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createplacedresource)
    fn create_placed_resource<R: ResourceInterface>(
        &self,
        heap: &impl HeapInterface,
        heap_offset: u64,
        desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> Result<R, DxError>;

    /// Describes the purpose of a query heap. A query heap contains an array of individual queries.
    ///
    /// # Arguments
    /// * `desc` - Specifies the query heap in a [`QueryHeapDesc`] structure.
    ///
    /// For more information: [`ID3D12Device::CreateQueryHeap method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createqueryheap)
    fn create_query_heap<Q: QueryHeapInterface>(&self, desc: &QueryHeapDesc) -> Result<Q, DxError>;

    /// Creates a render-target view for accessing resource data.
    ///
    /// # Arguments
    /// * `resource` - A reference to the [`ResourceInterface`] object that represents the render target.
    /// * `desc` - A reference to a [`RenderTargetViewDesc`] structure that describes the render-target view.
    /// * `dest_descriptor` - Describes the CPU descriptor handle that represents the destination where the newly-created render target view will reside.
    ///
    /// For more information: [`ID3D12Device::CreateRenderTargetView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createrendertargetview)
    fn create_render_target_view(
        &self,
        resource: Option<&impl ResourceInterface>,
        desc: Option<&RenderTargetViewDesc>,
        handle: CpuDescriptorHandle,
    );

    /// Creates a root signature layout.
    ///
    /// # Arguments
    /// * `node_mask` - For single GPU operation, set this to zero.
    ///   If there are multiple GPU nodes, set bits to identify the nodes (the device's physical adapters) to which the root signature is to apply.
    ///   Each bit in the mask corresponds to a single node.
    /// * `blob` - A reference to the source data for the serialized signature.
    ///
    /// For more information: [`ID3D12Device::CreateRootSignature method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createrootsignature)
    fn create_root_signature<RS: RootSignatureInterface>(
        &self,
        node_mask: u32,
        blob: &[u8],
    ) -> Result<RS, DxError>;

    /// Serializes and creates a root signature layout.
    ///
    /// # Arguments
    /// * `desc` - The description of the root signature, as a reference to a [`RootSignatureDesc`] structure.
    /// * `version` - A [`RootSignatureVersion`]-typed value that specifies the version of root signature.
    /// * `node_mask` - For single GPU operation, set this to zero.
    ///   If there are multiple GPU nodes, set bits to identify the nodes (the device's physical adapters) to which the root signature is to apply.
    ///   Each bit in the mask corresponds to a single node.
    fn serialize_and_create_root_signature<RS: RootSignatureInterface>(
        &self,
        desc: &RootSignatureDesc<'_>,
        version: RootSignatureVersion,
        node_mask: u32,
    ) -> Result<RS, DxError>;

    /// Create a sampler object that encapsulates sampling information for a texture.
    ///
    /// # Arguments
    /// * `desc` - A reference to a [`SamplerDesc`] structure that describes the sampler.
    /// * `dest_descriptor` - Describes the CPU descriptor handle that represents the start of the heap that holds the sampler.
    ///
    /// For more information: [`ID3D12Device::CreateRootSignature method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createrootsignature)
    fn create_sampler(&self, desc: &SamplerDesc, dest_descriptor: CpuDescriptorHandle);

    fn get_descriptor_handle_increment_size(&self, r#type: DescriptorHeapType) -> u32;
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

            let res = res.unwrap_unchecked();

            Ok(CS::new(res))
        }
    }

    fn create_committed_resource<R: ResourceInterface>(
        &self,
        heap_properties: &HeapProperties,
        heap_flags: HeapFlags,
        desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> Result<R, DxError> {
        unsafe {
            let clear_value = optimized_clear_value.as_ref().map(|c| c.as_raw());
            let clear_value = clear_value.as_ref().map(|c| c as *const _);

            let mut resource = None;

            self.0.CreateCommittedResource(
                &heap_properties.as_raw(),
                heap_flags.as_raw(),
                &desc.as_raw(),
                initial_state.as_raw(),
                clear_value,
                &mut resource,
            ).map_err(DxError::from)?;

            let resource = resource.unwrap_unchecked();

            Ok(R::new(resource))
        }
    }

    fn create_compute_pipeline_state<CPS: PipelineStateInterface>(
        &self,
        desc: &ComputePipelineStateDesc<'_>,
    ) -> Result<CPS, DxError> {
        unsafe {
            let desc = desc.as_raw();

            let res: CPS::Raw = self.0.CreateComputePipelineState(&desc).map_err(DxError::from)?;

            Ok(CPS::new(res))
        }
    }

    fn create_constant_buffer_view(
        &self,
        desc: Option<&ConstantBufferViewDesc>,
        dest_descriptor: CpuDescriptorHandle,
    ) {
        unsafe {
            let desc = desc.map(|desc| desc.as_raw());
            let desc = desc.as_ref().map(|c| c as *const _);

            let dest_descriptor = dest_descriptor.as_raw();

            self.0.CreateConstantBufferView(desc, dest_descriptor);
        }
    }

    fn create_depth_stencil_view(
        &self,
        resource: Option<&impl ResourceInterface>,
        desc: Option<&DepthStencilViewDesc>,
        dest_descriptor: CpuDescriptorHandle,
    ) {
        unsafe {
            let desc = desc.map(|desc| desc.as_raw());
            let desc = desc.as_ref().map(|c| c as *const _);

            let dest_descriptor = dest_descriptor.as_raw();

            if let Some(resource) = resource {
                self.0.CreateDepthStencilView(resource.as_raw_ref(), desc, dest_descriptor);
            } else {
                self.0.CreateDepthStencilView(None, desc, dest_descriptor);
            }
        }
    }

    fn create_descriptor_heap<H: DescriptorHeapInterface>(
        &self,
        desc: &DescriptorHeapDesc,
    ) -> Result<H, DxError> {
        unsafe {
            let desc = desc.as_raw();

            let res: H::Raw  = self.0.CreateDescriptorHeap(&desc).map_err(DxError::from)?;

            Ok(H::new(res))
        }
    }

    fn create_fence<F: FenceInterface>(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> Result<F, DxError> {
        unsafe {
            let res: F::Raw = self.0.CreateFence(initial_value, flags.as_raw()).map_err(DxError::from)?;

            Ok(F::new(res))
        }
    }

    fn create_graphics_pipeline<G: PipelineStateInterface>(
        &self,
        desc: &GraphicsPipelineDesc<'_>,
    ) -> Result<G, DxError> {
        unsafe {
            let desc = desc.as_raw();

            let res: G::Raw = self.0.CreateGraphicsPipelineState(&desc).map_err(DxError::from)?;

            Ok(G::new(res))
        }
    }

    fn create_heap<H: HeapInterface>(&self, desc: &HeapDesc) -> Result<H, DxError> {
        unsafe {
            let desc = desc.as_raw();

            let mut res = None;
            self.0.CreateHeap(&desc, &mut res).map_err(DxError::from)?;
            let res = res.unwrap_unchecked();

            Ok(H::new(res))
        }
    }

    fn create_placed_resource<R: ResourceInterface>(
        &self,
        heap: &impl HeapInterface,
        heap_offset: u64,
        desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> Result<R, DxError> {
        unsafe {
            let clear_value = optimized_clear_value.as_ref().map(|c| c.as_raw());
            let clear_value = clear_value.as_ref().map(|c| c as *const _);

            let mut resource = None;

            self.0.CreatePlacedResource(
                heap.as_raw_ref(),
                heap_offset,
                &desc.as_raw(),
                initial_state.as_raw(),
                clear_value,
                &mut resource,
            ).map_err(DxError::from)?;

            let resource = resource.unwrap_unchecked();

            Ok(R::new(resource))
        }
    }

    fn create_query_heap<Q: QueryHeapInterface>(
        &self,
        desc: &QueryHeapDesc,
    ) -> Result<Q, DxError> {
        unsafe {
            let desc = desc.as_raw();
            let mut res = None;
            self.0.CreateQueryHeap(&desc, &mut res).map_err(DxError::from)?;
            let res = res.unwrap_unchecked();
            Ok(Q::new(res))
        }
    }

    fn create_render_target_view(
        &self,
        resource: Option<&impl ResourceInterface>,
        desc: Option<&RenderTargetViewDesc>,
        handle: CpuDescriptorHandle,
    ) {
        unsafe {
            let desc = desc.map(|v| v.as_raw());
            let desc = desc.as_ref().map(|f| f as *const _);

            if let Some(resource) = resource {
                self.0.CreateRenderTargetView(resource.as_raw_ref(), desc, handle.as_raw());
            } else {
                self.0.CreateRenderTargetView(None, desc, handle.as_raw());
            }
        }
    }

    fn create_root_signature<RS: RootSignatureInterface>(
        &self,
        node_mask: u32,
        blob: &[u8],
    ) -> Result<RS, DxError> {
        unsafe {
            let res: RS::Raw = self.0.CreateRootSignature(node_mask, blob).map_err(DxError::from)?;

            Ok(RS::new(res))
        }
    }

    fn serialize_and_create_root_signature<RS: RootSignatureInterface>(
        &self,
        desc: &RootSignatureDesc<'_>,
        version: RootSignatureVersion,
        node_mask: u32,
    ) -> Result<RS, DxError> {
        unsafe {
            let blob = RS::serialize(desc, version)?;

            self.create_root_signature(
                node_mask,
                std::slice::from_raw_parts(
                    blob.get_buffer_ptr() as _,
                    blob.get_buffer_size(),
                )
            )
        }
    }

    fn create_sampler(&self, desc: &SamplerDesc, dest_descriptor: CpuDescriptorHandle) {
        unsafe {
            let desc = desc.as_raw();

            self.0.CreateSampler(&desc, dest_descriptor.as_raw());
        }
    }

    fn get_descriptor_handle_increment_size(&self, r#type: DescriptorHeapType) -> u32 {
        unsafe {
            self.0.GetDescriptorHandleIncrementSize(r#type.as_raw())
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
}
