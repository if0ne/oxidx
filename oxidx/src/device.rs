use std::{ffi::CStr, ops::Range};

use windows::{
    core::{Interface, PCWSTR},
    Win32::Graphics::Direct3D12::ID3D12Device,
};

use crate::{
    blob::IBlob,
    command_allocator::ICommandAllocator,
    command_list::ICommandList,
    command_queue::ICommandQueue,
    command_signature::ICommandSignature,
    create_type,
    descriptor_heap::IDescriptorHeap,
    device_child::{DeviceChild, IDeviceChild},
    dx::IRootSignatureExt,
    error::DxError,
    heap::IHeap,
    impl_trait,
    pageable::Pageable,
    pso::IPipelineState,
    query_heap::IQueryHeap,
    resources::IResource,
    root_signature::IRootSignature,
    sync::IFence,
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
pub trait IDevice: HasInterface<Raw: Interface> {
    /// Gets information about the features that are supported by the current graphics driver.
    ///
    /// For more information: [`ID3D12Device::CheckFeatureSupport method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-checkfeaturesupport)
    fn check_feature_support<F: FeatureObject>(
        &self,
        feature_input: F::UserInput<'_>,
    ) -> Result<F::Output, DxError>;

    /// Copies descriptors from a source to a destination.
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
    /// For more information: [`ID3D12Device::CreateCommandAllocator method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcommandallocator)
    fn create_command_allocator<CA: ICommandAllocator>(
        &self,
        r#type: CommandListType,
    ) -> Result<CA, DxError>;

    /// Creates a command list.
    ///
    /// For more information: [`ID3D12Device::CreateCommandList method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcommandlist)
    fn create_command_list<CL: ICommandList>(
        &self,
        node_mask: u32,
        r#type: CommandListType,
        command_allocator: &impl ICommandAllocator,
        pso: Option<&impl IPipelineState>,
    ) -> Result<CL, DxError>;

    /// Creates a command queue.
    ///
    /// For more information: [`ID3D12Device::CreateCommandQueue method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcommandqueue)
    fn create_command_queue<CQ: ICommandQueue>(
        &self,
        desc: &CommandQueueDesc,
    ) -> Result<CQ, DxError>;

    /// Creates a command queue.
    ///
    /// For more information: [`ID3D12Device::CreateCommandSignature method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcommandsignature)
    fn create_command_signature<CS: ICommandSignature>(
        &self,
        desc: &CommandSignatureDesc<'_>,
        root_signature: Option<&impl IRootSignature>,
    ) -> Result<CS, DxError>;

    /// Creates both a resource and an implicit heap, such that the heap is big enough to contain the entire resource, and the resource is mapped to the heap.
    ///
    /// For more information: [`ID3D12Device::CreateCommittedResource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcommittedresource)
    fn create_committed_resource<R: IResource>(
        &self,
        heap_properties: &HeapProperties,
        heap_flags: HeapFlags,
        desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> Result<R, DxError>;

    /// Creates a compute pipeline state object.
    ///
    /// For more information: [`ID3D12Device::CreateComputePipelineState method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createcomputepipelinestate)
    fn create_compute_pipeline_state<CPS: IPipelineState>(
        &self,
        desc: &ComputePipelineStateDesc<'_>,
    ) -> Result<CPS, DxError>;

    /// Creates a constant-buffer view for accessing resource data.
    ///
    /// For more information: [`ID3D12Device::CreateConstantBufferView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createconstantbufferview)
    fn create_constant_buffer_view(
        &self,
        desc: Option<&ConstantBufferViewDesc>,
        dest_descriptor: CpuDescriptorHandle,
    );

    /// Creates a depth-stencil view for accessing resource data.
    ///
    /// For more information: [`ID3D12Device::CreateDepthStencilView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createdepthstencilview)
    fn create_depth_stencil_view(
        &self,
        resource: Option<&impl IResource>,
        desc: Option<&DepthStencilViewDesc>,
        dest_descriptor: CpuDescriptorHandle,
    );

    /// Creates a descriptor heap object.
    ///
    /// For more information: [`ID3D12Device::CreateDescriptorHeap method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createdescriptorheap)
    fn create_descriptor_heap<H: IDescriptorHeap>(
        &self,
        desc: &DescriptorHeapDesc,
    ) -> Result<H, DxError>;

    /// Creates a fence object.
    ///
    /// For more information: [`ID3D12Device::CreateFence method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createfence)
    fn create_fence<F: IFence>(&self, initial_value: u64, flags: FenceFlags) -> Result<F, DxError>;

    /// Creates a graphics pipeline state object.
    ///
    /// For more information: [`ID3D12Device::CreateGraphicsPipelineState method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-creategraphicspipelinestate)
    fn create_graphics_pipeline<G: IPipelineState>(
        &self,
        desc: &GraphicsPipelineDesc<'_>,
    ) -> Result<G, DxError>;

    /// Creates a heap that can be used with placed resources and reserved resources.
    ///
    /// For more information: [`ID3D12Device::CreateHeap method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createheap)
    fn create_heap<H: IHeap>(&self, desc: &HeapDesc) -> Result<H, DxError>;

    /// Creates a resource that is placed in a specific heap. Placed resources are the lightest weight resource objects available, and are the fastest to create and destroy.
    ///
    /// For more information: [`ID3D12Device::CreatePlacedResource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createplacedresource)
    fn create_placed_resource<R: IResource>(
        &self,
        heap: &impl IHeap,
        heap_offset: u64,
        desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> Result<R, DxError>;

    /// Describes the purpose of a query heap. A query heap contains an array of individual queries.
    ///
    /// For more information: [`ID3D12Device::CreateQueryHeap method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createqueryheap)
    fn create_query_heap<Q: IQueryHeap>(&self, desc: &QueryHeapDesc) -> Result<Q, DxError>;

    /// Creates a render-target view for accessing resource data.
    ///
    /// For more information: [`ID3D12Device::CreateRenderTargetView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createrendertargetview)
    fn create_render_target_view(
        &self,
        resource: Option<&impl IResource>,
        desc: Option<&RenderTargetViewDesc>,
        handle: CpuDescriptorHandle,
    );

    /// Creates a resource that is reserved, and not yet mapped to any pages in a heap.
    ///
    /// For more information: [`ID3D12Device::CreateReservedResource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createreservedresource)
    fn create_reserved_resource<R: IResource>(
        &self,
        desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> Result<R, DxError>;

    /// Creates a root signature layout.
    ///
    /// For more information: [`ID3D12Device::CreateRootSignature method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createrootsignature)
    fn create_root_signature<RS: IRootSignature>(
        &self,
        node_mask: u32,
        blob: &[u8],
    ) -> Result<RS, DxError>;

    /// Serializes and creates a root signature layout.
    fn serialize_and_create_root_signature<RS: IRootSignature + IRootSignatureExt>(
        &self,
        desc: &RootSignatureDesc<'_>,
        version: RootSignatureVersion,
        node_mask: u32,
    ) -> Result<RS, DxError>;

    /// Create a sampler object that encapsulates sampling information for a texture.
    ///
    /// For more information: [`ID3D12Device::CreateRootSignature method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createrootsignature)
    fn create_sampler(&self, desc: &SamplerDesc, dest_descriptor: CpuDescriptorHandle);

    /// Creates a shader-resource view for accessing data in a resource.
    ///
    /// For more information: [`ID3D12Device::CreateShaderResourceView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createshaderresourceview)
    fn create_shader_resource_view(
        &self,
        resource: Option<&impl IResource>,
        desc: Option<&ShaderResourceViewDesc>,
        handle: CpuDescriptorHandle,
    );

    /// Creates a shared handle to a heap, resource, or fence object.
    ///
    /// For more information: [`ID3D12Device::CreateSharedHandle method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createsharedhandle)
    fn create_shared_handle(
        &self,
        shareable: &DeviceChild,
        name: Option<&CStr>,
    ) -> Result<SharedHandle, DxError>;

    /// Creates a shader-resource view for accessing data in a resource.
    ///
    /// For more information: [`ID3D12Device::CreateUnorderedAccessView method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-createunorderedaccessview)
    fn create_unordered_access_view(
        &self,
        resource: Option<&impl IResource>,
        counter_resource: Option<&impl IResource>,
        desc: Option<&UnorderedAccessViewDesc>,
        handle: CpuDescriptorHandle,
    );

    /// Enables the page-out of data, which precludes GPU access of that data.
    ///
    /// For more information: [`ID3D12Device::Evict method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-evict)
    fn evict(&self, objects: &[Option<Pageable>]) -> Result<(), DxError>;

    /// Gets a locally unique identifier for the current device (adapter).
    ///
    /// For more information: [`ID3D12Device::GetAdapterLuid method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-getadapterluid)
    fn get_adapter_luid(&self) -> Luid;

    /// Gets a resource layout that can be copied. Helps the app fill-in [`PlacedSubresourceFootprint`] and [`SubresourceFootprint`] when suballocating space in upload heaps.
    ///
    /// For more information: [`ID3D12Device::GetCopyableFootprints method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-getcopyablefootprints)
    fn get_copyable_footprints(
        &self,
        resource_desc: &ResourceDesc,
        subresources: Range<u32>,
        base_offset: u64,
        layouts: &mut [PlacedSubresourceFootprint],
        num_rows: &mut [u32],
        row_sizes: &mut [u32],
    ) -> u64;

    /// Gets a resource layout that can be copied. Helps the app fill-in [`PlacedSubresourceFootprint`] and [`SubresourceFootprint`] when suballocating space in upload heaps.
    ///
    /// For more information: [`ID3D12Device::GetCustomHeapProperties method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-getcustomheapproperties(uint_d3d12_heap_types))
    fn get_custom_heap_properties(&self, node_mask: u32, r#type: HeapType) -> HeapProperties;

    /// Gets the size of the handle increment for the given type of descriptor heap. This value is typically used to increment a handle into a descriptor array by the correct amount.
    ///
    /// For more information: [`ID3D12Device::GetDescriptorHandleIncrementSize method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-getdescriptorhandleincrementsize)
    fn get_descriptor_handle_increment_size(&self, r#type: DescriptorHeapType) -> u32;

    /// Gets the reason that the device was removed, or [`Result::Ok`] if the device isn't removed.
    /// To be called back when a device is removed, consider using [`IFence::set_event_on_completion`] with a value of [`u64::MAX`].
    /// That's because device removal causes all fences to be signaled to that value (which also implies completing all events waited on, because they'll all be less than [`u64::MAX`]).
    ///
    /// For more information: [`ID3D12Device::GetDeviceRemovedReason method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-getdeviceremovedreason)
    fn get_device_removed_reason(&self) -> Result<(), DxError>;

    /// Reports the number of physical adapters (nodes) that are associated with this device.
    ///
    /// For more information: [`ID3D12Device::GetNodeCount method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-getnodecount)
    fn get_node_count(&self) -> u32;

    /// Gets the size and alignment of memory required for a collection of resources on this adapter.
    ///
    /// For more information: [`ID3D12Device::GetResourceAllocationInfo method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-getresourceallocationinfo(uint_uint_constd3d12_resource_desc))
    fn get_resource_allocation_info(
        &self,
        visible_mask: u32,
        resource_desc: &[ResourceDesc],
    ) -> ResourceAllocationInfo;

    /// Gets info about how a tiled resource is broken into tiles.
    ///
    /// For more information: [`ID3D12Device::GetResourceTiling method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-getresourcetiling)
    fn get_resource_tiling(
        &self,
        resource: &impl IResource,
        first_subresource_tiling_to_get: u32,
        num_tiles_for_entire_resource: Option<&mut [u32]>,
        packed_mip_desc: Option<&mut [PackedMipDesc]>,
        standard_tile_shape_for_non_packed_mips: Option<&mut [TileShape]>,
        num_subresource_tilings: Option<&mut [u32]>,
    ) -> SubresourceTiling;

    /// Makes objects resident for the device.
    ///
    /// For more information: [`ID3D12Device::MakeResident method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-makeresident)
    fn make_resident(&self, objects: &[&Pageable]) -> Result<(), DxError>;

    /// Opens a handle for shared resources, shared heaps, and shared fences, by using [`SharedHandle`].
    ///
    /// For more information: [`ID3D12Device::OpenSharedHandle method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-opensharedhandle)
    fn open_shared_handle<D: IDeviceChild>(&self, handle: SharedHandle) -> Result<D, DxError>;

    /// Opens a handle for shared resources, shared heaps, and shared fences, by using Name.
    ///
    /// For more information: [`ID3D12Device::OpenSharedHandleByName method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-opensharedhandlebyname)
    fn open_shared_handle_by_name(&self, name: &CStr) -> Result<SharedHandle, DxError>;

    /// A development-time aid for certain types of profiling and experimental prototyping.
    ///
    /// For more information: [`ID3D12Device::SetStablePowerState method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12device-setstablepowerstate)
    fn set_stable_power_state(&self, enable: bool) -> Result<(), DxError>;
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
    impl IDevice =>
    Device;

    fn check_feature_support<F: FeatureObject>(&self, feature_input: F::UserInput<'_>) -> Result<F::Output, DxError> {
        unsafe {
            let raw = F::into_input(feature_input);
            let mut raw = F::from_input(raw);

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
                dest_descriptor_range_start.0,
                src_descriptor_range_start.0,
                descriptor_heaps_type.as_raw()
            );
        }
    }

    fn create_command_allocator<CA: ICommandAllocator>(
        &self,
        r#type: CommandListType
    ) -> Result<CA, DxError> {
        unsafe {
            let res: CA::Raw = self.0.CreateCommandAllocator(r#type.as_raw()).map_err(DxError::from)?;

            Ok(CA::new(res))
        }
    }

    fn create_command_queue<CQ: ICommandQueue>(
        &self,
        desc: &CommandQueueDesc,
    ) -> Result<CQ, DxError> {
        unsafe {
            let res: CQ::Raw = self.0.CreateCommandQueue(&desc.0).map_err(DxError::from)?;

            Ok(CQ::new(res))
        }
    }

    fn create_command_signature<CS: ICommandSignature>(
        &self,
        desc: &CommandSignatureDesc<'_>,
        root_signature: Option<&impl IRootSignature>,
    ) -> Result<CS, DxError> {
        unsafe {
            let mut res: Option<CS::Raw> = None;

            if let Some(root_signature) = root_signature {
                self.0.CreateCommandSignature(
                    &desc.0,
                    root_signature.as_raw_ref(),
                    &mut res
                ).map_err(DxError::from)?;
            } else {
                self.0.CreateCommandSignature(
                    &desc.0,
                    None,
                    &mut res
                ).map_err(DxError::from)?;
            }

            let res = res.unwrap_unchecked();

            Ok(CS::new(res))
        }
    }

    fn create_committed_resource<R: IResource>(
        &self,
        heap_properties: &HeapProperties,
        heap_flags: HeapFlags,
        desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> Result<R, DxError> {
        unsafe {
            let clear_value = optimized_clear_value.as_ref().map(|c| &c.0 as *const _);

            let mut resource = None;

            self.0.CreateCommittedResource(
                &heap_properties.0,
                heap_flags.as_raw(),
                &desc.0,
                initial_state.as_raw(),
                clear_value,
                &mut resource,
            ).map_err(DxError::from)?;

            let resource = resource.unwrap_unchecked();

            Ok(R::new(resource))
        }
    }

    fn create_command_list<CL: ICommandList>(
        &self,
        node_mask: u32,
        r#type: CommandListType,
        command_allocator: &impl ICommandAllocator,
        pso: Option<&impl IPipelineState>,
    ) -> Result<CL, DxError> {
        unsafe {
            let res: CL::Raw = if let Some(pso) = pso {
                self.0.CreateCommandList(
                    node_mask,
                    r#type.as_raw(),
                    command_allocator.as_raw_ref(),
                    pso.as_raw_ref()
                ).map_err(DxError::from)?
            } else {
                self.0.CreateCommandList(
                    node_mask,
                    r#type.as_raw(),
                    command_allocator.as_raw_ref(),
                    None
                ).map_err(DxError::from)?
            };

            Ok(CL::new(res))
        }
    }

    fn create_compute_pipeline_state<CPS: IPipelineState>(
        &self,
        desc: &ComputePipelineStateDesc<'_>,
    ) -> Result<CPS, DxError> {
        unsafe {
            let res: CPS::Raw = self.0.CreateComputePipelineState(&desc.0).map_err(DxError::from)?;

            Ok(CPS::new(res))
        }
    }

    fn create_constant_buffer_view(
        &self,
        desc: Option<&ConstantBufferViewDesc>,
        dest_descriptor: CpuDescriptorHandle,
    ) {
        unsafe {
            let desc = desc.map(|c| &c.0 as *const _);

            let dest_descriptor = dest_descriptor.0;

            self.0.CreateConstantBufferView(desc, dest_descriptor);
        }
    }

    fn create_depth_stencil_view(
        &self,
        resource: Option<&impl IResource>,
        desc: Option<&DepthStencilViewDesc>,
        dest_descriptor: CpuDescriptorHandle,
    ) {
        unsafe {
            let desc = desc.map(|c| &c.0 as *const _);

            let dest_descriptor = dest_descriptor.0;

            if let Some(resource) = resource {
                self.0.CreateDepthStencilView(
                    resource.as_raw_ref(),
                    desc,
                    dest_descriptor
                );
            } else {
                self.0.CreateDepthStencilView(
                    None,
                    desc,
                    dest_descriptor
                );
            }
        }
    }

    fn create_descriptor_heap<H: IDescriptorHeap>(
        &self,
        desc: &DescriptorHeapDesc,
    ) -> Result<H, DxError> {
        unsafe {
            let res: H::Raw  = self.0.CreateDescriptorHeap(&desc.0).map_err(DxError::from)?;

            Ok(H::new(res))
        }
    }

    fn create_fence<F: IFence>(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> Result<F, DxError> {
        unsafe {
            let res: F::Raw = self.0.CreateFence(initial_value, flags.as_raw()).map_err(DxError::from)?;

            Ok(F::new(res))
        }
    }

    fn create_graphics_pipeline<G: IPipelineState>(
        &self,
        desc: &GraphicsPipelineDesc<'_>,
    ) -> Result<G, DxError> {
        unsafe {
            let res: G::Raw = self.0.CreateGraphicsPipelineState(&desc.0).map_err(DxError::from)?;

            Ok(G::new(res))
        }
    }

    fn create_heap<H: IHeap>(&self, desc: &HeapDesc) -> Result<H, DxError> {
        unsafe {
            let mut res = None;
            self.0.CreateHeap(&desc.0, &mut res).map_err(DxError::from)?;
            let res = res.unwrap_unchecked();

            Ok(H::new(res))
        }
    }

    fn create_placed_resource<R: IResource>(
        &self,
        heap: &impl IHeap,
        heap_offset: u64,
        desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> Result<R, DxError> {
        unsafe {
            let clear_value = optimized_clear_value.as_ref().map(|c| &c.0 as *const _);

            let mut resource = None;

            self.0.CreatePlacedResource(
                heap.as_raw_ref(),
                heap_offset,
                &desc.0,
                initial_state.as_raw(),
                clear_value,
                &mut resource,
            ).map_err(DxError::from)?;

            let resource = resource.unwrap_unchecked();

            Ok(R::new(resource))
        }
    }

    fn create_query_heap<Q: IQueryHeap>(
        &self,
        desc: &QueryHeapDesc,
    ) -> Result<Q, DxError> {
        unsafe {
            let mut res = None;
            self.0.CreateQueryHeap(&desc.0, &mut res).map_err(DxError::from)?;
            let res = res.unwrap_unchecked();
            Ok(Q::new(res))
        }
    }

    fn create_render_target_view(
        &self,
        resource: Option<&impl IResource>,
        desc: Option<&RenderTargetViewDesc>,
        handle: CpuDescriptorHandle,
    ) {
        unsafe {
            let desc = desc.map(|f| &f.0 as *const _);

            if let Some(resource) = resource {
                self.0.CreateRenderTargetView(
                    resource.as_raw_ref(),
                    desc,
                    handle.0
                );
            } else {
                self.0.CreateRenderTargetView(
                    None,
                    desc,
                    handle.0
                );
            }
        }
    }

    fn create_reserved_resource<R: IResource>(
        &self,
        desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> Result<R, DxError> {
        unsafe {
            let clear_value = optimized_clear_value.as_ref().map(|c| &c.0 as *const _);

            let mut resource = None;

            self.0.CreateReservedResource(
                &desc.0,
                initial_state.as_raw(),
                clear_value,
                &mut resource,
            ).map_err(DxError::from)?;

            let resource = resource.unwrap_unchecked();

            Ok(R::new(resource))
        }
    }

    fn create_root_signature<RS: IRootSignature>(
        &self,
        node_mask: u32,
        blob: &[u8],
    ) -> Result<RS, DxError> {
        unsafe {
            let res: RS::Raw = self.0.CreateRootSignature(node_mask, blob).map_err(DxError::from)?;

            Ok(RS::new(res))
        }
    }

    fn serialize_and_create_root_signature<RS: IRootSignature + IRootSignatureExt>(
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
                    blob.get_buffer_ptr().as_ptr() as _,
                    blob.get_buffer_size(),
                )
            )
        }
    }

    fn create_sampler(&self, desc: &SamplerDesc, dest_descriptor: CpuDescriptorHandle) {
        unsafe {
            self.0.CreateSampler(&desc.0, dest_descriptor.0);
        }
    }

    fn create_shader_resource_view(
        &self,
        resource: Option<&impl IResource>,
        desc: Option<&ShaderResourceViewDesc>,
        handle: CpuDescriptorHandle,
    ) {
        unsafe {
            let desc = desc.as_ref().map(|f| &f.0 as *const _);

            if let Some(resource) = resource {
                self.0.CreateShaderResourceView(
                    resource.as_raw_ref(),
                    desc,
                    handle.0
                );
            } else {
                self.0.CreateShaderResourceView(
                    None,
                    desc,
                    handle.0
                );
            }

        }
    }

    fn create_shared_handle(
        &self,
        shareable: &DeviceChild,
        name: Option<&CStr>,
    ) -> Result<SharedHandle, DxError> {
        unsafe {
            let name = PCWSTR::from_raw(
                name.map(|name| name.as_ptr())
                    .unwrap_or(std::ptr::null())
                    as *const _
            );
            let handle = self.0.CreateSharedHandle(
                shareable.as_raw_ref(),
                None,
                0x10000000,
                name
            ).map_err(DxError::from)?;

            Ok(SharedHandle(handle))
        }
    }

    fn create_unordered_access_view(
        &self,
        resource: Option<&impl IResource>,
        counter_resource: Option<&impl IResource>,
        desc: Option<&UnorderedAccessViewDesc>,
        handle: CpuDescriptorHandle,
    ) {
        unsafe {
            let desc = desc.as_ref().map(|f| &f.0 as *const _);

            match (resource, counter_resource) {
                (Some(r), Some(c)) => {
                    self.0.CreateUnorderedAccessView(
                        r.as_raw_ref(),
                        c.as_raw_ref(),
                        desc,
                        handle.0
                    );
                },
                (Some(r), None) => {
                    self.0.CreateUnorderedAccessView(
                        r.as_raw_ref(),
                        None,
                        desc,
                        handle.0
                    );
                },
                (None, Some(c)) => {
                    self.0.CreateUnorderedAccessView(
                        None,
                        c.as_raw_ref(),
                        desc,
                        handle.0
                    );
                },
                (None, None) => {
                    self.0.CreateUnorderedAccessView(
                        None,
                        None,
                        desc,
                        handle.0
                    );
                }
            }
        }
    }

    fn evict(&self, objects: &[Option<Pageable>]) -> Result<(), DxError> {
        unsafe {
            self.0.Evict(std::mem::transmute::<&_, &_>(objects)).map_err(DxError::from)
        }
    }

    fn get_adapter_luid(&self) -> Luid {
        unsafe {
            Luid(self.0.GetAdapterLuid())
        }
    }

    fn get_copyable_footprints(
        &self,
        resource_desc: &ResourceDesc,
        subresources: Range<u32>,
        base_offset: u64,
        layouts: &mut [PlacedSubresourceFootprint],
        num_rows: &mut [u32],
        row_sizes: &mut [u32],
    ) -> u64 {
        unsafe {
            let mut total_bytes = 0;

            self.0.GetCopyableFootprints(
                &resource_desc.0,
                subresources.start,
                subresources.count() as u32,
                base_offset,
                Some(layouts.as_mut_ptr() as *mut _),
                Some(num_rows.as_mut_ptr() as *mut _),
                Some(row_sizes.as_mut_ptr() as *mut _),
                Some(&mut total_bytes)
            );

            total_bytes
        }
    }

    fn get_custom_heap_properties(
        &self,
        node_mask: u32,
        r#type: HeapType,
    ) -> HeapProperties {
        unsafe {
            HeapProperties(self.0.GetCustomHeapProperties(node_mask, r#type.as_raw()))
        }
    }

    fn get_descriptor_handle_increment_size(&self, r#type: DescriptorHeapType) -> u32 {
        unsafe {
            self.0.GetDescriptorHandleIncrementSize(r#type.as_raw())
        }
    }

    fn get_device_removed_reason(&self) -> Result<(), DxError> {
        unsafe {
            self.0.GetDeviceRemovedReason().map_err(DxError::from)
        }
    }

    fn get_node_count(&self) -> u32 {
        unsafe {
            self.0.GetNodeCount()
        }
    }

    fn get_resource_allocation_info(
        &self,
        visible_mask: u32,
        resource_desc: &[ResourceDesc],
    ) -> ResourceAllocationInfo {
        unsafe {
            let resource_desc = std::slice::from_raw_parts(resource_desc.as_ptr() as *const _, resource_desc.len());

            ResourceAllocationInfo(self.0.GetResourceAllocationInfo(visible_mask, resource_desc))
        }
    }

    fn get_resource_tiling(
        &self,
        resource: &impl IResource,
        first_subresource_tiling_to_get: u32,
        num_tiles_for_entire_resource: Option<&mut [u32]>,
        packed_mip_desc: Option<&mut [PackedMipDesc]>,
        standard_tile_shape_for_non_packed_mips: Option<&mut [TileShape]>,
        num_subresource_tilings: Option<&mut [u32]>
    ) -> SubresourceTiling {
        unsafe {
            let mut res = Default::default();
            self.0.GetResourceTiling(
                resource.as_raw_ref(),
                num_tiles_for_entire_resource.map(|v| v.as_mut_ptr()),
                packed_mip_desc.map(|v| v.as_mut_ptr() as *mut _),
                standard_tile_shape_for_non_packed_mips.map(|v| v.as_mut_ptr() as *mut _),
                num_subresource_tilings.map(|v| v.as_mut_ptr()),
                first_subresource_tiling_to_get,
                &mut res
            );

            SubresourceTiling(res)
        }
    }

    fn make_resident(
        &self,
        objects: &[&Pageable],
    ) -> Result<(), DxError> {
        unsafe {
            let objects = std::slice::from_raw_parts(objects.as_ptr() as *const _, objects.len());

            self.0.MakeResident(objects).map_err(DxError::from)
        }
    }

    fn open_shared_handle<D: IDeviceChild>(
        &self,
        handle: SharedHandle,
    ) -> Result<D, DxError> {
        unsafe {
            let mut res = None;
            self.0.OpenSharedHandle(handle.0, &mut res)?;
            let res = res.unwrap();

            Ok(D::new(res))
        }
    }

    fn open_shared_handle_by_name(&self, name: &CStr) -> Result<SharedHandle, DxError> {
        unsafe {
            let name = PCWSTR::from_raw(
                name.as_ptr()
                    as *const _
            );
            let handle = self.0.OpenSharedHandleByName(
                name,
                0x10000000,
            ).map_err(DxError::from)?;

            Ok(SharedHandle(handle))
        }
    }

    fn set_stable_power_state(&self, enable: bool) -> Result<(), DxError> {
        unsafe {
            self.0.SetStablePowerState(enable).map_err(DxError::from)
        }
    }
}
