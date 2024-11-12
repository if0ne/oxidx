use windows::Win32::Graphics::Direct3D12::*;

#[allow(unused_imports)]
use super::*;

bitflags::bitflags! {
    /// Identifies the type of DXGI adapter.
    ///
    /// Empty flag - Specifies no flags.
    ///
    /// For more information: [`DXGI_ADAPTER_FLAG enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ne-dxgi-dxgi_adapter_flag)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct AdapterFlags: i32 {
        /// Value always set to 0. This flag is reserved.
        const Remote = DXGI_ADAPTER_FLAG_REMOTE.0;

        /// Specifies a software adapter.
        const Sofware = DXGI_ADAPTER_FLAG_SOFTWARE.0;
    }
}

bitflags::bitflags! {
    /// Identifies how to view a buffer resource.
    ///
    /// Empty flag - Indicates a default view.
    ///
    /// For more information: [`D3D12_BUFFER_SRV_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_buffer_srv_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct BufferSrvFlags: i32 {
        /// View the buffer as raw.
        const Raw = D3D12_BUFFER_SRV_FLAG_RAW.0;
    }
}

bitflags::bitflags! {
    /// Identifies unordered-access view options for a buffer resource.
    ///
    /// Empty flag - Indicates a default view.
    ///
    /// For more information: [`D3D12_BUFFER_UAV_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_buffer_uav_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct BufferUavFlags: i32 {
        /// Resource contains raw, unstructured data.
        const Raw = D3D12_BUFFER_UAV_FLAG_RAW.0;
    }
}

bitflags::bitflags! {
    /// Describes the level of support for shader caching in the current graphics driver.
    ///
    /// Empty flag - Indicates that the driver does not support shader caching.
    ///
    /// For more information: [`D3D12_SHADER_CACHE_SUPPORT_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_shader_cache_support_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct CacheSupportFlags: i32 {
        /// Indicates that the driver supports the CachedPSO member of the [`GraphicsPipelineStateDesc`] and [`ComputePipelineStateDesc`] structures. This is always supported.
        const SinglePSO = D3D12_SHADER_CACHE_SUPPORT_SINGLE_PSO.0;

        /// Indicates that the driver supports the ID3D12PipelineLibrary interface, which provides application-controlled PSO grouping and caching.
        /// This is supported by drivers targetting the Windows 10 Anniversary Update.
        const Library = D3D12_SHADER_CACHE_SUPPORT_LIBRARY.0;

        /// Indicates that the driver supports an OS-managed shader cache that stores compiled shaders in memory during the current run of the application.
        const AutomaticInprocCache = D3D12_SHADER_CACHE_SUPPORT_AUTOMATIC_INPROC_CACHE.0;

        /// Indicates that the driver supports an OS-managed shader cache that stores compiled shaders on disk to accelerate future runs of the application.
        const AutomaticDiskCache = D3D12_SHADER_CACHE_SUPPORT_AUTOMATIC_DISK_CACHE.0;

        /// TBD
        const DriverManagedCache = D3D12_SHADER_CACHE_SUPPORT_DRIVER_MANAGED_CACHE.0;

        /// TBD
        const ShaderControlClear = D3D12_SHADER_CACHE_SUPPORT_SHADER_CONTROL_CLEAR.0;

        /// TBD
        const ShaderSessionDelete = D3D12_SHADER_CACHE_SUPPORT_SHADER_SESSION_DELETE.0;
    }
}

bitflags::bitflags! {
    /// TBD
    ///
    /// For more information: [`D3D12_MESSAGE_CALLBACK_FLAGS enumeration`](https://microsoft.github.io/DirectX-Specs/d3d/MessageCallback.html)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct CallbackFlags: i32 {
        /// TBD
        const IgnoreFilters = D3D12_MESSAGE_CALLBACK_IGNORE_FILTERS.0;
    }
}

bitflags::bitflags! {
    /// Specifies what to clear from the depth stencil view.
    ///
    /// For more information: [`D3D12_CLEAR_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_clear_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct ClearFlags: i32 {
        /// Indicates the depth buffer should be cleared.
        const Depth = D3D12_CLEAR_FLAG_DEPTH.0;

        /// Indicates the stencil buffer should be cleared.
        const Stencil = D3D12_CLEAR_FLAG_STENCIL.0;
    }
}

bitflags::bitflags! {
    /// Identifies which components of each pixel of a render target are writable during blending.
    ///
    /// For more information: [`D3D12_COLOR_WRITE_ENABLE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_color_write_enable)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct ColorWriteEnable: i32 {
        /// Allow data to be stored in the red component.
        const R = D3D12_COLOR_WRITE_ENABLE_RED.0;

        /// Allow data to be stored in the green component.
        const G = D3D12_COLOR_WRITE_ENABLE_GREEN.0;

        /// Allow data to be stored in the blue component.
        const B = D3D12_COLOR_WRITE_ENABLE_BLUE.0;

        /// Allow data to be stored in the alpha component.
        const A = D3D12_COLOR_WRITE_ENABLE_ALPHA.0;
    }
}

bitflags::bitflags! {
    /// Used to determine which kinds of command lists are capable of supporting various operations. For example, whether a command list supports immediate writes.
    ///
    /// Empty flag - Specifies that no command list supports the operation in question.
    ///
    /// For more information: [`D3D12_COMMAND_LIST_SUPPORT_FLAGS enumeration`](https://learn.microsoft.com/ru-ru/windows/win32/api/d3d12/ne-d3d12-d3d12_command_list_support_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct CommandListSupportFlags: i32 {
        /// Specifies that direct command lists can support the operation in question.
        const Direct = D3D12_COMMAND_LIST_SUPPORT_FLAG_DIRECT.0;

        /// Specifies that command list bundles can support the operation in question.
        const Bundle = D3D12_COMMAND_LIST_SUPPORT_FLAG_BUNDLE.0;

        /// Specifies that compute command lists can support the operation in question.
        const Compute = D3D12_COMMAND_LIST_SUPPORT_FLAG_COMPUTE.0;

        /// Specifies that copy command lists can support the operation in question.
        const Copy = D3D12_COMMAND_LIST_SUPPORT_FLAG_COPY.0;

        /// Specifies that video-decode command lists can support the operation in question.
        const VideoDecode = D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_DECODE.0;

        /// Specifies that video-processing command lists can support the operation is question.
        const VideoProcess = D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_PROCESS.0;

        /// Specifies that video-encode command lists can support the operation in question.
        const VideoEncode = D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_ENCODE.0;
    }
}

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
    /// Options for enumerating display modes.
    ///
    /// For more information: [`DXGI_ENUM_MODES enumeration`](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dxgi-enum-modes)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct EnumModeFlags: u32 {
        /// Include interlaced modes.
        const Interlaced = DXGI_ENUM_MODES_INTERLACED.0;

        /// Include stretched-scaling modes.
        const Scaling = DXGI_ENUM_MODES_SCALING.0;

        /// Include stereo modes.
        const Stereo = DXGI_ENUM_MODES_STEREO.0;

        /// Include stereo modes that are hidden because the user has disabled stereo.
        /// Control panel applications can use this option to show stereo capabilities that have been disabled as part of a user interface
        /// that enables and disables stereo.
        const DisabledStereo = DXGI_ENUM_MODES_DISABLED_STEREO.0;
    }
}

bitflags::bitflags! {
    /// Identifies the portion of a depth-stencil buffer for writing depth data.
    ///
    /// Empty flag - Turn off writes to the depth-stencil buffer.
    ///
    /// For more information: [`D3D12_DEPTH_WRITE_MASK enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_depth_write_mask)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct DepthWriteMask: i32 {
        /// Turn on writes to the depth-stencil buffer.
        const All = D3D12_DEPTH_WRITE_MASK_ALL.0;
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
    /// Specifies depth-stencil view options.
    ///
    /// Empty flag - Indicates a default view.
    ///
    /// For more information: [`D3D12_DSV_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_dsv_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct DsvFlags: i32 {
        /// Indicates that depth values are read only.
        const ReadOnlyDepth = D3D12_DSV_FLAG_READ_ONLY_DEPTH.0;

        /// Indicates that stencil values are read only.
        const ReadOnlyStencil = D3D12_DSV_FLAG_READ_ONLY_STENCIL.0;
    }
}

bitflags::bitflags! {
    /// Valid values include the `Debug` flag, and zero.
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct FactoryCreationFlags: u32 {
        /// Request DXGIDebug.dll; the DLL will be loaded if it is present on the system.
        const Debug = DXGI_CREATE_FACTORY_DEBUG.0;
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
    /// Flags for surface and resource creation options.
    ///
    /// For more information: [`DXGI_USAGE`](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dxgi-usage)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct FrameBufferUsage: u32 {
        /// The surface or resource is used as a back buffer. You don’t need to pass `BackBuffer` when you create a swap chain.
        const BackBuffer = DXGI_USAGE_BACK_BUFFER.0;

        /// This flag is for internal use only.
        const DiscardOnPresent = DXGI_USAGE_DISCARD_ON_PRESENT.0;

        /// Use the surface or resource for reading only.
        const ReadOnly = DXGI_USAGE_READ_ONLY.0;

        /// Use the surface or resource as an output render target.
        const RenderTargetOutput = DXGI_USAGE_RENDER_TARGET_OUTPUT.0;

        /// Use the surface or resource as an input to a shader.
        const ShaderInput = DXGI_USAGE_SHADER_INPUT.0;

        /// Share the surface or resource.
        const Shared = DXGI_USAGE_SHARED.0;

        /// Use the surface or resource for unordered access.
        const UnorderedAccess = DXGI_USAGE_UNORDERED_ACCESS.0;
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
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    /// Flags to control pipeline state.
    ///
    /// Empty flag - Indicates no flags.
    ///
    /// For more information: [`D3D12_PIPELINE_STATE_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_pipeline_state_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct PipelineStateFlags: i32 {
        /// Indicates that the pipeline state should be compiled with additional information to assist debugging.
        ///
        /// This can only be set on WARP devices.
        const ToolDebug = D3D12_PIPELINE_STATE_FLAG_TOOL_DEBUG.0;

        /// Indicates that the pipeline state can be dynamically changed after the pipeline is set by using RSSetDepthBias.
        const DynamicDepthBias = D3D12_PIPELINE_STATE_FLAG_DYNAMIC_DEPTH_BIAS.0;

        /// Indicates that the pipeline state can be dynamically changed after the pipeline is set by using IASetIndexBufferStripCutValue.
        const DynamicIndexBufferStripCut = D3D12_PIPELINE_STATE_FLAG_DYNAMIC_INDEX_BUFFER_STRIP_CUT.0;
    }
}

bitflags::bitflags! {
    /// The [`PresentFlags`] constants specify options for presenting frames to the output.
    ///
    /// Empty flag - Present a frame from each buffer (starting with the current buffer) to the output.
    ///
    /// For more information: [`DXGI_PRESENT`](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dxgi-present)
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct PresentFlags: u32 {
        /// Present a frame from the current buffer to the output.
        /// Use this flag so that the presentation can use vertical-blank synchronization instead of sequencing buffers in the chain in the usual manner.
        const DoNotSequence = DXGI_PRESENT_DO_NOT_SEQUENCE.0;

        /// Do not present the frame to the output.
        const Test = DXGI_PRESENT_TEST.0;

        /// Specifies that the runtime will discard outstanding queued presents.
        const Restart = DXGI_PRESENT_RESTART.0;

        /// Specifies that the runtime will fail the presentation with the error code if the calling thread is blocked.
        const DoNotWait = DXGI_PRESENT_DO_NOT_WAIT.0;

        /// Indicates that presentation content will be shown only on the particular output. The content will not be visible on other outputs.
        const RestrictToOutput = DXGI_PRESENT_RESTRICT_TO_OUTPUT.0;

        /// Indicates that if the stereo present must be reduced to mono, right-eye viewing is used rather than left-eye viewing.
        const StereoPreferRight = DXGI_PRESENT_STEREO_PREFER_RIGHT.0;

        /// Indicates that the presentation should use the left buffer as a mono buffer.
        const StereoTemporaryMono = DXGI_PRESENT_STEREO_TEMPORARY_MONO.0;

        /// This flag must be set by media apps that are currently using a custom present duration (custom refresh rate).
        const UseDuration = DXGI_PRESENT_USE_DURATION.0;

        /// Allowing tearing is a requirement of variable refresh rate displays.
        const AllowTearing = DXGI_PRESENT_ALLOW_TEARING.0;
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
    /// Flags for setting split resource barriers.
    ///
    /// Empty flag - No flags.
    ///
    /// For more information: [`D3D12_RESOURCE_BARRIER_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_resource_barrier_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct ResourceBarrierFlags: i32 {
        /// This starts a barrier transition in a new state, putting a resource in a temporary no-access condition.
        const BeginOnly = D3D12_RESOURCE_BARRIER_FLAG_BEGIN_ONLY.0;

        /// This barrier completes a transition, setting a new state and restoring active access to a resource.
        const EndOnly = D3D12_RESOURCE_BARRIER_FLAG_END_ONLY.0;
    }
}

bitflags::bitflags! {
    /// Defines constants that specify options for working with resources.
    ///
    /// Empty flag - No options are specified.
    ///
    /// For more information: [`D3D12_RESOURCE_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_resource_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct ResourceFlags: i32 {
        /// Allows a render target view to be created for the resource; and also enables the resource to transition into the state of [`ResourceStates::RenderTarget`].
        const AllowRenderTarget = D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET.0;

        /// Allows a depth stencil view to be created for the resource, as well as enables the resource to transition into the state of [`ResourceStates::DepthWrite`] and/or [`ResourceStates::DepthRead`].
        const AllowDepthStencil = D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL.0;

        /// Allows an unordered access view to be created for the resource, as well as enables the resource to transition into the state of [`ResourceStates::UnorderedAccess`].
        const AllowUnorderedAccess = D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS.0;

        /// Disallows a shader resource view from being created for the resource, as well as disables the resource from transitioning into the state of [`ResourceStates::NonPixelShaderResource`] or [`ResourceStates::PixelShaderResource`].
        const DenyShaderResource = D3D12_RESOURCE_FLAG_DENY_SHADER_RESOURCE.0;

        /// Allows the resource to be used for cross-adapter data, as well as those features enabled by [`ResourceFlags::AllowSimultaneousAccess`].
        const AllowCrossAdapter = D3D12_RESOURCE_FLAG_ALLOW_CROSS_ADAPTER.0;

        /// Allows a resource to be simultaneously accessed by multiple different queues, devices, or processes.
        const AllowSimultaneousAccess = D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS.0;

        /// Specfies that this resource may be used only as a decode reference frame. It may be written to or read only by the video decode operation.
        const VideoDecodeReferenceOnly = D3D12_RESOURCE_FLAG_VIDEO_DECODE_REFERENCE_ONLY.0;

        /// Specfies that this resource may be used only as an encode reference frame. It may be written to or read only by the video encode operation.
        const VideoEncodeReferenceOnly = D3D12_RESOURCE_FLAG_VIDEO_ENCODE_REFERENCE_ONLY.0;

        /// Requires the DirectX 12 Agility SDK 1.7 or later. Indicates that a buffer is to be used as a raytracing acceleration structure.
        const RaytracingAccelerationStructure = D3D12_RESOURCE_FLAG_RAYTRACING_ACCELERATION_STRUCTURE.0;
    }
}

bitflags::bitflags! {
    /// Defines constants that specify the state of a resource regarding how the resource is being used.
    ///
    /// For more information: [`D3D12_RESOURCE_STATES enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_resource_states)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct ResourceStates: i32 {
        /// Your application should transition to this state only for accessing a resource across different graphics engine types.
        const Common = D3D12_RESOURCE_STATE_COMMON.0;

        /// A subresource must be in this state when it is accessed by the GPU as a vertex buffer or constant buffer. This is a read-only state.
        const VertexAndConstantBuffer = D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER.0;

        /// A subresource must be in this state when it is accessed by the 3D pipeline as an index buffer. This is a read-only state.
        const IndexBuffer = D3D12_RESOURCE_STATE_INDEX_BUFFER.0;

        /// The resource is used as a render target.
        const RenderTarget = D3D12_RESOURCE_STATE_RENDER_TARGET.0;

        /// The resource is used for unordered access
        const UnorderedAccess = D3D12_RESOURCE_STATE_UNORDERED_ACCESS.0;

        /// This is a state that is mutually exclusive with other states.
        const DepthWrite = D3D12_RESOURCE_STATE_DEPTH_WRITE.0;

        /// This is a state that can be combined with other states.
        const DepthRead = D3D12_RESOURCE_STATE_DEPTH_READ.0;

        /// The resource is used with a shader other than the pixel shader.
        const NonPixelShaderResource = D3D12_RESOURCE_STATE_NON_PIXEL_SHADER_RESOURCE.0;

        /// The resource is used with a pixel shader.
        const PixelShaderResource = D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE.0;

        /// The resource is used with stream output.
        const StreamOut = D3D12_RESOURCE_STATE_STREAM_OUT.0;

        /// The resource is used as an indirect argument.
        const IndirectArgument = D3D12_RESOURCE_STATE_INDIRECT_ARGUMENT.0;

        /// The resource is used as the destination in a copy operation.
        const CopyDest = D3D12_RESOURCE_STATE_COPY_DEST.0;

        /// The resource is used as the source in a copy operation.
        const CopySource = D3D12_RESOURCE_STATE_COPY_SOURCE.0;

        /// The resource is used as the destination in a resolve operation.
        const ResolveDest = D3D12_RESOURCE_STATE_RESOLVE_DEST.0;

        /// The resource is used as the source in a resolve operation.
        const ResolveSource = D3D12_RESOURCE_STATE_RESOLVE_SOURCE.0;

        /// When a buffer is created with this as its initial state, it indicates that the resource is a raytracing acceleration structure.
        const RaytracingAccelerationStructure = D3D12_RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE.0;

        /// Starting with Windows 10, version 1903 (10.0; Build 18362), indicates that the resource is a screen-space shading-rate image for variable-rate shading (VRS).
        const ShadingRateSource = D3D12_RESOURCE_STATE_SHADING_RATE_SOURCE.0;

        /// This is a logically OR'd combination of other read-state bits. This is the required starting state for an upload heap.
        const GenericRead = D3D12_RESOURCE_STATE_GENERIC_READ.0;

        /// Equivalent to [`ResourceStates::NonPixelShaderResource`] | [`ResourceStates::PixelShaderResource`]
        const AllShaderResource = D3D12_RESOURCE_STATE_ALL_SHADER_RESOURCE.0;

        /// Synonymous with [`ResourceStates::Common`].
        const Present = D3D12_RESOURCE_STATE_PRESENT.0;

        /// The resource is used for Predication.
        const Predication = D3D12_RESOURCE_STATE_PREDICATION.0;

        /// The resource is used as a source in a decode operation.
        const VideoDecodeRead = D3D12_RESOURCE_STATE_VIDEO_DECODE_READ.0;

        /// The resource is used as a destination in the decode operation.
        const VideoDecodeWrite = D3D12_RESOURCE_STATE_VIDEO_DECODE_WRITE.0;

        /// The resource is used to read video data during video processing.
        const VideoProcessRead = D3D12_RESOURCE_STATE_VIDEO_PROCESS_READ.0;

        /// The resource is used to write video data during video processing.
        const VideoProcessWrite = D3D12_RESOURCE_STATE_VIDEO_PROCESS_WRITE.0;

        /// The resource is used as the source in an encode operation.
        const VideoEncodeRead = D3D12_RESOURCE_STATE_VIDEO_ENCODE_READ.0;

        /// This resource is used as the destination in an encode operation.
        const VideoEncodeWrite = D3D12_RESOURCE_STATE_VIDEO_ENCODE_WRITE.0;
    }
}

bitflags::bitflags! {
    /// Specifies options for root signature layout.
    ///
    /// Empty flag - Indicates default behavior.
    ///
    /// For more information: [`D3D12_ROOT_SIGNATURE_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_root_signature_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct RootSignatureFlags: i32 {
        /// The app is opting in to using the Input Assembler (requiring an input layout that defines a set of vertex buffer bindings).
        /// Omitting this flag can result in one root argument space being saved on some hardware.
        /// Omit this flag if the Input Assembler is not required, though the optimization is minor.
        const AllowInputAssemblerInputLayout = D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT.0;

        /// Denies the vertex shader access to the root signature.
        const DenyVertexShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_VERTEX_SHADER_ROOT_ACCESS.0;

        /// Denies the hull shader access to the root signature.
        const DenyHullShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS.0;

        /// Denies the domain shader access to the root signature.
        const DenyDomainShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS.0;

        /// Denies the geometry shader access to the root signature.
        const DenyGeometryShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS.0;

        /// Denies the pixel shader access to the root signature.
        const DenyPixelShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_PIXEL_SHADER_ROOT_ACCESS.0;

        /// The app is opting in to using Stream Output. Omitting this flag can result in one root argument space being saved on some hardware. Omit this flag if Stream Output is not required, though the optimization is minor.
        const AllowStreamOutput = D3D12_ROOT_SIGNATURE_FLAG_ALLOW_STREAM_OUTPUT.0;

        /// The root signature is to be used with raytracing shaders to define resource bindings sourced from shader records in shader tables.
        /// This flag cannot be combined with any other root signature flags, which are all related to the graphics pipeline.
        /// The absence of the flag means the root signature can be used with graphics or compute, where the compute version is also shared with raytracing’s global root signature.
        const Local = D3D12_ROOT_SIGNATURE_FLAG_LOCAL_ROOT_SIGNATURE.0;

        /// Denies the amplification shader access to the root signature.
        const DenyAmplificationShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_AMPLIFICATION_SHADER_ROOT_ACCESS.0;

        /// Denies the mesh shader access to the root signature.
        const DenyMeshShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_MESH_SHADER_ROOT_ACCESS.0;

        /// The shaders are allowed to index the CBV/SRV/UAV descriptor heap directly, using the ResourceDescriptorHeap built-in variable.
        const CbvSrvUavHeapDirectlyIndexed = D3D12_ROOT_SIGNATURE_FLAG_CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED.0;

        /// The shaders are allowed to index the sampler descriptor heap directly, using the SamplerDescriptorHeap built-in variable.
        const SamplerHeapDirectlyIndexed = D3D12_ROOT_SIGNATURE_FLAG_SAMPLER_HEAP_DIRECTLY_INDEXED.0;
    }
}

bitflags::bitflags! {
    /// Specifies how to copy a tile.
    ///
    /// Empty flag - No tile-copy flags are specified.
    ///
    /// For more information: [`D3D12_TILE_COPY_FLAGS enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_tile_copy_flags)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct TileCopyFlags: i32 {
        /// Indicates that the GPU isn't currently referencing any of the portions of destination memory being written.
        const NoHazard = D3D12_TILE_COPY_FLAG_NO_HAZARD.0;

        /// Indicates that the [`IGraphicsCommandList::copy_tiles`](crate::command_list::IGraphicsCommandList) operation involves copying a linear buffer to a swizzled tiled resource.
        const LinearBufferToSwizzledTiledResource = D3D12_TILE_COPY_FLAG_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE.0;

        /// Indicates that the [`IGraphicsCommandList::copy_tiles`](crate::command_list::IGraphicsCommandList) operation involves copying a swizzled tiled resource to a linear buffer. This means to copy tile data from the tile region, reading tiles sequentially (in x,y,z order if the region is a box),
        const SwizzledTiledResourceToLinearBuffer = D3D12_TILE_COPY_FLAG_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER.0;
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

bitflags::bitflags! {
    /// Bitflags representing shader requirements in Direct3D.
    ///
    /// For more information, refer to the Direct3D documentation.
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct ShaderRequirements: u32 {
        /// Shader requires that the graphics driver and hardware support the double data type.
        const RequiresDoubles = D3D_SHADER_REQUIRES_DOUBLES.0;

        /// Shader requires an early depth stencil.
        const RequiresEarlyDepthStencil = D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL.0;

        /// Shader requires unordered access views (UAVs) at every pipeline stage.
        const RequiresUAVsAtEveryStage = D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE.0;

        /// Shader requires 64 UAVs.
        const Requires64UAVs = D3D_SHADER_REQUIRES_64_UAVS.0;

        /// Shader requires the graphics driver and hardware to support minimum precision.
        const RequiresMinimumPrecision = D3D_SHADER_REQUIRES_MINIMUM_PRECISION.0;

        /// Shader requires that the graphics driver and hardware support extended doubles instructions.
        const Requires11_1DoubleExtensions = D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS.0;

        /// Shader requires that the graphics driver and hardware support the msad4 intrinsic function in shaders.
        const Requires11_1ShaderExtensions = D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS.0;

        /// Shader requires that the graphics driver and hardware support Direct3D 9 shadow support.
        const RequiresLevel9ComparisonFiltering = D3D_SHADER_REQUIRES_LEVEL_9_COMPARISON_FILTERING.0;

        /// Shader requires that the graphics driver and hardware support tiled resources.
        const RequiresTiledResources = D3D_SHADER_REQUIRES_TILED_RESOURCES.0;

        /// Shader requires a reference value for depth stencil tests.
        const RequiresStencilRef = D3D_SHADER_REQUIRES_STENCIL_REF.0;

        /// Shader requires that the graphics driver and hardware support inner coverage.
        const RequiresInnerCoverage = D3D_SHADER_REQUIRES_INNER_COVERAGE.0;

        /// Shader requires that the graphics driver and hardware support the loading of additional formats for typed UAVs.
        const RequiresTypedUAVLoadAdditionalFormats = D3D_SHADER_REQUIRES_TYPED_UAV_LOAD_ADDITIONAL_FORMATS.0;

        /// Shader requires that the graphics driver and hardware support rasterizer ordered views (ROVs).
        const RequiresROVs = D3D_SHADER_REQUIRES_ROVS.0;

        /// Shader requires that the graphics driver and hardware support viewport and render target array index values from any shader-feeding rasterizer.
        const RequiresViewportAndRTArrayIndexFromAnyShaderFeedingRasterizer = D3D_SHADER_REQUIRES_VIEWPORT_AND_RT_ARRAY_INDEX_FROM_ANY_SHADER_FEEDING_RASTERIZER.0;

        /// Shader requires that the graphics driver and hardware support wave ops.
        const RequiresWaveOps = D3D_SHADER_REQUIRES_WAVE_OPS.0;

        /// Shader requires that the graphics driver and hardware support 64-bit integer ops.
        const RequiresInt64Ops = D3D_SHADER_REQUIRES_INT64_OPS.0;

        /// Shader requires that the graphics driver and hardware support view instancing using SV_ViewID.
        const RequiresViewId = D3D_SHADER_REQUIRES_VIEW_ID.0;

        /// Shader requires that the graphics driver and hardware support barycentrics using SV_Barycentrics.
        const RequiresBarycentrics = D3D_SHADER_REQUIRES_BARYCENTRICS.0;

        /// Shader requires that the graphics driver and hardware support native 16-bit ops.
        const RequiresNative16BitOps = D3D_SHADER_REQUIRES_NATIVE_16BIT_OPS.0;

        /// Shader requires that the graphics driver and hardware support the Variable Shading Rate (VRS) feature.
        const RequiresShadingRate = D3D_SHADER_REQUIRES_SHADING_RATE.0;

        /// Shader requires that the graphics driver and hardware support DXR tier 1.1.
        const RequiresRaytracingTier1_1 = D3D_SHADER_REQUIRES_RAYTRACING_TIER_1_1.0;

        /// Shader requires that the graphics driver and hardware support Sampler Feedback.
        const RequiresSamplerFeedback = D3D_SHADER_REQUIRES_SAMPLER_FEEDBACK.0;

        /// Shader requires that the graphics driver and hardware support int64 atomics on typed resources.
        const RequiresAtomicInt64OnTypedResource = D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_TYPED_RESOURCE.0;

        /// Shader requires that the graphics driver and hardware support int64 atomics on groupshared memory.
        const RequiresAtomicInt64OnGroupShared = D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_GROUP_SHARED.0;

        /// Shader requires that the graphics driver and hardware support derivatives in mesh and amplification shaders.
        const RequiresDerivativesInMeshAndAmplificationShaders = D3D_SHADER_REQUIRES_DERIVATIVES_IN_MESH_AND_AMPLIFICATION_SHADERS.0;

        /// Shader requires that the graphics driver and hardware support Dynamic Resources and the ResourceDescriptorHeap.
        const RequiresResourceDescriptorHeapIndexing = D3D_SHADER_REQUIRES_RESOURCE_DESCRIPTOR_HEAP_INDEXING.0;

        /// Shader requires that the graphics driver and hardware support Dynamic Resources and the SamplerDescriptorHeap.
        const RequiresSamplerDescriptorHeapIndexing = D3D_SHADER_REQUIRES_SAMPLER_DESCRIPTOR_HEAP_INDEXING.0;

        /// Shader requires that the graphics driver and hardware support Wave MMA.
        const RequiresWaveMMA = D3D_SHADER_REQUIRES_WAVE_MMA.0;

        /// Shader requires that the graphics driver and hardware support int64 atomics on descriptor heap resources.
        const RequiresAtomicInt64OnDescriptorHeapResource = D3D_SHADER_REQUIRES_ATOMIC_INT64_ON_DESCRIPTOR_HEAP_RESOURCE.0;
    }
}

bitflags::bitflags! {
    /// Options for swap-chain behavior.
    ///
    /// For more information: [`DXGI_SWAP_CHAIN_FLAG enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ne-dxgi-dxgi_swap_chain_flag)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct SwapchainFlags: i32 {
        /// Set this flag to turn off automatic image rotation; that is, do not perform a rotation when transferring the contents of the front buffer to the monitor.
        const NonPrerotated = DXGI_SWAP_CHAIN_FLAG_NONPREROTATED.0;

        /// Set this flag to enable an application to switch modes.
        const AllowModeSwitch = DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0;

        /// Set this flag to enable an application to render using GDI on a swap chain or a surface.
        const GdiCompatible = DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE.0;

        /// Set this flag to indicate that the swap chain might contain protected content;
        /// therefore, the operating system supports the creation of the swap chain only when driver and hardware protection is used.
        /// If the driver and hardware do not support content protection, the call to create a resource for the swap chain fails.
        const RestrictContent = DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT.0;

        /// Set this flag to indicate that shared resources that are created within the swap chain must be protected by using the driver’s mechanism for restricting access to shared surfaces.
        const RestrictSharedResourceDriver = DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER.0;

        /// Set this flag to restrict presented content to the local displays.
        const DisplayOnly = DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY.0;

        /// Set this flag to create a waitable object you can use to ensure rendering does not begin while a frame is still being presented.
        const FrameLatencyWaitableObject = DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT.0;

        /// Set this flag to create a swap chain in the foreground layer for multi-plane rendering.
        const ForegroundLayer = DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER.0;

        /// Set this flag to create a swap chain for full-screen video.
        const FullscreenVideo = DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO.0;

        /// Set this flag to create a swap chain for YUV video.
        const YuvVideo = DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO.0;

        /// Indicates that the swap chain should be created such that all underlying resources can be protected by the hardware.
        const Protected = DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED.0;

        /// Tearing support is a requirement to enable displays that support variable refresh rates to function properly when the application presents a swap chain tied to a full screen borderless window.
        const AllowTearing = DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING.0;

        /// TBD
        const RestrictedToAllHolographicDisplays = DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS.0;
    }
}

bitflags::bitflags! {
    /// TBD
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct WindowAssociationFlags: u32 {
        /// revent DXGI from monitoring an applications message queue; this makes DXGI unable to respond to mode changes.
        const NoWindowChanges = DXGI_MWA_NO_WINDOW_CHANGES.0;

        /// Prevent DXGI from responding to an alt-enter sequence.
        const NoAltEnter = DXGI_MWA_NO_ALT_ENTER.0;

        /// Prevent DXGI from responding to a print-screen key.
        const NoPrintScreen = DXGI_MWA_NO_PRINT_SCREEN.0;
    }
}
