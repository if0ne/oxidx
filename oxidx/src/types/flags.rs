use windows::Win32::Graphics::Direct3D12::*;

#[allow(unused_imports)]
use super::*;

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
    /// Specifies resources that are supported for a provided format.
    ///
    /// Empty flag - No resources are supported.
    ///
    /// For more information: [`D3D12_FORMAT_SUPPORT1 enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_format_support1)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct FormatSupport1: i32 {
        /// Buffer resources supported.
        const Buffer = D3D12_FORMAT_SUPPORT1_BUFFER.0;

        /// Vertex buffers supported.
        const IAVertexBuffer = D3D12_FORMAT_SUPPORT1_IA_VERTEX_BUFFER.0;

        /// Index buffers supported.
        const IAIndexBuffer = D3D12_FORMAT_SUPPORT1_IA_INDEX_BUFFER.0;

        /// Streaming output buffers supported.
        const SOBuffer = D3D12_FORMAT_SUPPORT1_SO_BUFFER.0;

        /// 1D texture resources supported.
        const Texture1D = D3D12_FORMAT_SUPPORT1_TEXTURE1D.0;

        /// 2D texture resources supported.
        const Texture2D = D3D12_FORMAT_SUPPORT1_TEXTURE2D.0;

        /// 3D texture resources supported.
        const Texture3D = D3D12_FORMAT_SUPPORT1_TEXTURE3D.0;

        /// Cube texture resources supported.
        const TextureCube = D3D12_FORMAT_SUPPORT1_TEXTURECUBE.0;

        /// The HLSL Load function for texture objects is supported.
        const ShaderLoad = D3D12_FORMAT_SUPPORT1_SHADER_LOAD.0;

        /// The HLSL Sample function for texture objects is supported.
        const ShaderSample = D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE.0;

        /// The HLSL SampleCmp and SampleCmpLevelZero functions for texture objects are supported.
        const ShaderSampleComparison = D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_COMPARISON.0;

        /// Mipmaps are supported.
        const Mip = D3D12_FORMAT_SUPPORT1_MIP.0;

        /// Render targets are supported.
        const RenderTarget = D3D12_FORMAT_SUPPORT1_RENDER_TARGET.0;

        /// Blend operations supported.
        const Blendable = D3D12_FORMAT_SUPPORT1_BLENDABLE.0;

        /// Depth stencils supported.
        const DepthStencil = D3D12_FORMAT_SUPPORT1_DEPTH_STENCIL.0;

        /// Multisample antialiasing (MSAA) resolve operations are supported.
        const MultiSampleResolve = D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RESOLVE.0;

        ///Format can be displayed on screen.
        const Display = D3D12_FORMAT_SUPPORT1_DISPLAY.0;

        /// Format can't be cast to another format.
        const CastWithinBitLayout = D3D12_FORMAT_SUPPORT1_CAST_WITHIN_BIT_LAYOUT.0;

        /// Format can be used as a multi-sampled render target.
        const MultiSampleRenderTarget = D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RENDERTARGET.0;

        /// Format can be used as a multi-sampled texture and read into a shader with the HLSL Load function.
        const MultiSampleLoad = D3D12_FORMAT_SUPPORT1_MULTISAMPLE_LOAD.0;

        /// Format can be used with the HLSL gather function. This value is available in DirectX 10.1 or higher.
        const ShaderGather = D3D12_FORMAT_SUPPORT1_SHADER_GATHER.0;

        /// Format supports casting when the resource is a back buffer.
        const BackbufferCast = D3D12_FORMAT_SUPPORT1_BACK_BUFFER_CAST.0;

        /// Format can be used for an unordered access view.
        const TypedUnorderedAccessView = D3D12_FORMAT_SUPPORT1_TYPED_UNORDERED_ACCESS_VIEW.0;

        /// Format can be used with the HLSL gather with comparison function.
        const ShaderGatherComparison = D3D12_FORMAT_SUPPORT1_SHADER_GATHER_COMPARISON.0;

        /// Format can be used with the decoder output.
        const DecoderOutput = D3D12_FORMAT_SUPPORT1_DECODER_OUTPUT.0;

        /// Format can be used with the video processor output.
        const VideoProcessorOutput = D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_OUTPUT.0;

        /// Format can be used with the video processor input.
        const VideoProcessorInput = D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_INPUT.0;

        /// Format can be used with the video encoder.
        const VideoEncoder = D3D12_FORMAT_SUPPORT1_VIDEO_ENCODER.0;
    }
}

bitflags::bitflags! {
    /// Specifies which unordered resource options are supported for a provided format.
    ///
    /// Empty flag - No unordered resource options are supported.
    ///
    /// For more information: [`D3D12_FORMAT_SUPPORT2 enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_format_support2)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct FormatSupport2: i32 {
        /// Format supports atomic add.
        const UavAtomicAdd = D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_ADD.0;

        /// Format supports atomic bitwise operations.
        const UavAtomicBitwiseOps = D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_BITWISE_OPS.0;

        /// Format supports atomic compare with store or exchange.
        const UavAtomicCompareStoreOrCompareExchange = D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE.0;

        /// Format supports atomic exchange.
        const UavAtomicExchange = D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_EXCHANGE.0;

        /// Format supports atomic min and max.
        const UavAtomicSignedMinOrMax = D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_SIGNED_MIN_OR_MAX.0;

        /// Format supports atomic unsigned min and max.
        const UavAtomicUnsignedMinOrMax = D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX.0;

        /// Format supports a typed load.
        const UavTypedLoad = D3D12_FORMAT_SUPPORT2_UAV_TYPED_LOAD.0;

        /// Format supports a typed store.
        const UavTypedStore = D3D12_FORMAT_SUPPORT2_UAV_TYPED_STORE.0;

        /// Format supports logic operations in blend state.
        const OutputMergerLogicOp = D3D12_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP.0;

        /// Format supports tiled resources.
        const Tiled = D3D12_FORMAT_SUPPORT2_TILED.0;

        /// Format supports multi-plane overlays.
        const MultiplaneOverlay = D3D12_FORMAT_SUPPORT2_MULTIPLANE_OVERLAY.0;

        /// TBD
        const SamplerFeedback = D3D12_FORMAT_SUPPORT2_SAMPLER_FEEDBACK.0;
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

        /// The heap is not allowed to contain resources with [`ResourceDimension::Texture1D`], [`ResourceDimension::Texture2D`], or
        /// [`ResourceDimension::Texture3D`] unless either [`ResourceFlags::AllowRenderTarget`] or [`ResourceFlags::AllowDepthStencil`] are present.
        const DenyNonRtDsTextures = D3D12_HEAP_FLAG_DENY_NON_RT_DS_TEXTURES.0;

        /// The heap supports MEM_WRITE_WATCH functionality, which causes the system to track the pages that are written to in the committed memory region.
        /// This flag can't be combined with the [`HeapType::Default`] or [`CpuPageProperty::Unknown`] flags.
        /// Applications are discouraged from using this flag themselves because it prevents tools from using this functionality.
        const AllowWriteWatch = D3D12_HEAP_FLAG_ALLOW_WRITE_WATCH.0;

        /// Ensures that atomic operations will be atomic on this heap's memory, according to components able to see the memory.
        const AllowSharedAtomics = D3D12_HEAP_FLAG_ALLOW_SHADER_ATOMICS.0;

        /// The heap is created in a non-resident state and must be made resident using [`DeviceInterface::make_resident`](crate::device::DeviceInterface::make_resident) or [`Device3Interface::enqueue_make_resident`](crate::device::Device3Interface::enqueue_make_resident).
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
    /// Specifies options for determining quality levels.
    ///
    /// Empty flag - No options are supported.
    ///
    /// For more information: [`D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_multisample_quality_level_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct MultisampleQualityLevelFlags: i32 {
        /// The number of quality levels can be determined for tiled resources.
        const TiledResource = D3D12_MULTISAMPLE_QUALITY_LEVELS_FLAG_TILED_RESOURCE.0;
    }
}

bitflags::bitflags! {
    /// Specifies options for determining quality levels.
    ///
    /// Empty flag - Indicates that protected resource sessions are not supported.
    ///
    /// For more information: [`D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS enumeration`](https://learn.microsoft.com/ru-ru/windows/win32/api/d3d12/ne-d3d12-d3d12_protected_resource_session_support_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct ProtectedResourceSessionSupportFlags: i32 {
        /// Indicates that protected resource sessions are supported.
        const Supported = D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAG_SUPPORTED.0;
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
