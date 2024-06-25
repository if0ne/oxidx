use windows::Win32::Graphics::Direct3D12::*;

bitflags::bitflags! {
    /// Specifies flags to be used when creating a command queue.
    ///
    /// Empty flag - Indicates a default command queue.
    ///
    /// For more information: [`D3D12_COMMAND_QUEUE_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_command_queue_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct CommandQueueFlags: i32 {
        /// Indicates that the GPU timeout should be disabled for this command queue.
        const DisableGpuTimeout = D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT.0;
    }
}

bitflags::bitflags! {
    /// Specifies options for a heap.
    ///
    /// Empty flag - Indicates default usage of a heap.
    ///
    /// For more information: [`D3D12_DESCRIPTOR_HEAP_FLAG enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_descriptor_heap_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct DescriptorHeapFlags: i32 {
        /// The flag [`DescriptorHeapFlags::ShaderVisible`] can optionally be set on a descriptor heap to indicate it is be bound on a command list
        /// for reference by shaders. Descriptor heaps created without this flag allow applications the option to stage descriptors in CPU memory
        /// before copying them to a shader visible descriptor heap, as a convenience. But it is also fine for applications to directly create
        /// descriptors into shader visible descriptor heaps with no requirement to stage anything on the CPU.
        const ShaderVisible = D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE.0;
    }
}

bitflags::bitflags! {
    /// Specifies fence options.
    ///
    /// Empty flag - No options are specified.
    ///
    /// For more information: [`D3D12_FENCE_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_fence_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct FenceFlags: i32 {
        /// The fence is shared.
        const Shared = D3D12_FENCE_FLAG_SHARED.0;

        /// The fence is shared with another GPU adapter.
        const SharedCrossAdapter = D3D12_FENCE_FLAG_SHARED_CROSS_ADAPTER.0;

        /// The fence is of the non-monitored type. Non-monitored fences should only be used when the adapter doesn't support monitored fences,
        /// or when a fence is shared with an adapter that doesn't support monitored fences.
        const NonMonitored = D3D12_FENCE_FLAG_NON_MONITORED.0;
    }
}

bitflags::bitflags! {
    /// Describes the level of GPU-based validation to perform at runtime.
    ///
    /// Empty flag - Default behavior; resource states, descriptors, and descriptor tables are all validated.
    ///
    /// For more information: [`D3D12_GPU_BASED_VALIDATION_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/ne-d3d12sdklayers-d3d12_gpu_based_validation_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct GpuBasedValidationFlags: i32 {
        /// Indicates that the GPU timeout should be disabled for this command queue.
        const DisableStateTracking = D3D12_GPU_BASED_VALIDATION_FLAGS_DISABLE_STATE_TRACKING.0;
    }
}

bitflags::bitflags! {
    /// Specifies heap options, such as whether the heap can contain textures, and whether resources are shared across adapters.
    ///
    /// Empty flag - No options are specified.
    ///
    /// For more information: [`D3D12_HEAP_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_heap_flags)
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct HeapFlags: i32 {
        /// No options are specified.
        const Shared = D3D12_HEAP_FLAG_SHARED.0;

        /// The heap isn't allowed to contain buffers.
        const DenyBuffers = D3D12_HEAP_FLAG_DENY_BUFFERS.0;

        /// The heap is allowed to contain swap-chain surfaces.
        const AllowDisplay = D3D12_HEAP_FLAG_ALLOW_DISPLAY.0;

        /// The heap is allowed to share resources across adapters. A protected session cannot be mixed with resources that are shared across adapters.
        const SharedCrossAdapter = D3D12_HEAP_FLAG_SHARED_CROSS_ADAPTER.0;

        /// The heap is not allowed to store Render Target (RT) and/or Depth-Stencil (DS) textures.
        const DenyRtDsTextures = D3D12_HEAP_FLAG_DENY_RT_DS_TEXTURES.0;

        /// The heap is not allowed to contain resources with D3D12_RESOURCE_DIMENSION_TEXTURE1D, D3D12_RESOURCE_DIMENSION_TEXTURE2D, or
        /// D3D12_RESOURCE_DIMENSION_TEXTURE3D unless either D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET or D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL are present.
        const DenyNonRtDsTextures = D3D12_HEAP_FLAG_DENY_NON_RT_DS_TEXTURES.0;

        /// The heap supports MEM_WRITE_WATCH functionality, which causes the system to track the pages that are written to in the committed memory region.
        /// This flag can't be combined with the D3D12_HEAP_TYPE_DEFAULT or D3D12_CPU_PAGE_PROPERTY_UNKNOWN flags.
        /// Applications are discouraged from using this flag themselves because it prevents tools from using this functionality.
        const AllowWriteWatch = D3D12_HEAP_FLAG_ALLOW_WRITE_WATCH.0;

        /// Ensures that atomic operations will be atomic on this heap's memory, according to components able to see the memory.
        const AllowSharedAtomics = D3D12_HEAP_FLAG_ALLOW_SHADER_ATOMICS.0;

        /// The heap is created in a non-resident state and must be made resident using ID3D12Device::MakeResident or ID3D12Device3::EnqueueMakeResident.
        const CreateNotResident = D3D12_HEAP_FLAG_CREATE_NOT_RESIDENT.0;

        /// Allows the OS to not zero the heap created. By default, committed resources and heaps are almost always zeroed upon creation.
        /// This flag allows this to be elided in some scenarios. However, it doesn't guarantee it.
        /// For example, memory coming from other processes still needs to be zeroed for data protection and process isolation.
        /// This can lower the overhead of creating the heap.
        const CreateNotZeroed = D3D12_HEAP_FLAG_CREATE_NOT_ZEROED.0;

        /// TBD
        const ToolsUseManualWriteTracking = D3D12_HEAP_FLAG_TOOLS_USE_MANUAL_WRITE_TRACKING.0;

        /// The heap is allowed to store all types of buffers and/or textures.
        const AllowAllBuffersAndTextures = D3D12_HEAP_FLAG_ALLOW_ALL_BUFFERS_AND_TEXTURES.0;

        /// The heap is only allowed to store buffers.
        const AllowOnlyBuffers = D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS.0;

        /// The heap is only allowed to store non-RT, non-DS textures.
        const AllowOnlyNonRtDsTextures = D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES.0;

        /// The heap is only allowed to store RT and/or DS textures.
        const AllowOnlyRtDsTextures = D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES.0;
    }
}

bitflags::bitflags! {
    /// Specifies a range of tile mappings.
    ///
    /// Empty flag - No tile-mapping flags are specified.
    ///
    /// For more information: [`D3D12_TILE_RANGE_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_tile_range_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct TileRangeFlags: i32 {
        /// The tile range is NULL.
        const Null = D3D12_TILE_RANGE_FLAG_NULL.0;

        /// Skip the tile range.
        const Skip = D3D12_TILE_RANGE_FLAG_SKIP.0;

        /// Reuse a single tile in the tile range.
        const ReuseSingleTile = D3D12_TILE_RANGE_FLAG_REUSE_SINGLE_TILE.0;
    }
}
