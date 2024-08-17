use std::{ffi::CStr, marker::PhantomData, mem::ManuallyDrop, ops::Range};

use compact_str::CompactString;
use smallvec::SmallVec;
use windows::{
    core::PCSTR,
    Win32::Foundation::{CloseHandle, HANDLE, LUID},
};

use crate::{
    blob::Blob, error::DxError, resources::Resource, root_signature::RootSignature, HasInterface,
};

use super::*;

/// Describes an adapter (or video card) using DXGI 1.1.
///
/// For more information: [`DXGI_ADAPTER_DESC1 structure`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_adapter_desc1)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct AdapterDesc1(pub(crate) DXGI_ADAPTER_DESC1);

impl AdapterDesc1 {
    #[inline]
    pub fn description(&self) -> CompactString {
        CompactString::from_utf16_lossy(self.0.Description)
    }

    #[inline]
    pub fn vendor_id(&self) -> u32 {
        self.0.VendorId
    }

    #[inline]
    pub fn sub_sys_id(&self) -> u32 {
        self.0.SubSysId
    }

    #[inline]
    pub fn revision(&self) -> u32 {
        self.0.Revision
    }

    #[inline]
    pub fn dedicated_video_memory(&self) -> usize {
        self.0.DedicatedVideoMemory
    }

    #[inline]
    pub fn dedicated_system_memory(&self) -> usize {
        self.0.DedicatedSystemMemory
    }

    #[inline]
    pub fn shared_system_memory(&self) -> usize {
        self.0.DedicatedSystemMemory
    }

    #[inline]
    pub fn adapter_luid(&self) -> Luid {
        Luid(self.0.AdapterLuid)
    }

    #[inline]
    pub fn flags(&self) -> AdapterFlags {
        AdapterFlags::from_bits_retain(self.0.Flags as i32)
    }
}

/// Describes the blend state.
///
/// For more information: [`D3D12_BLEND_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_blend_desc)
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct BlendDesc(pub(crate) D3D12_BLEND_DESC);

impl BlendDesc {
    #[inline]
    pub fn with_render_targets(
        mut self,
        render_targets: impl IntoIterator<Item = RenderTargetBlendDesc>,
    ) -> Self {
        let mut rts = [D3D12_RENDER_TARGET_BLEND_DESC::default(); 8];

        for (i, desc) in render_targets.into_iter().take(8).enumerate() {
            rts[i] = desc.0;
        }

        self.0.RenderTarget = rts;
        self
    }

    #[inline]
    pub fn enable_alpha_to_coverage(mut self) -> Self {
        self.0.AlphaToCoverageEnable = true.into();
        self
    }

    #[inline]
    pub fn enable_independent_blend(mut self) -> Self {
        self.0.IndependentBlendEnable = true.into();
        self
    }
}

/// Describes a 3D box.
///
/// For more information: [`D3D12_BOX structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_box)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct Box(pub(crate) D3D12_BOX);

impl Box {
    #[inline]
    pub fn with_left(mut self, val: u32) -> Self {
        self.0.left = val;
        self
    }

    #[inline]
    pub fn with_top(mut self, val: u32) -> Self {
        self.0.top = val;
        self
    }

    #[inline]
    pub fn with_front(mut self, val: u32) -> Self {
        self.0.front = val;
        self
    }

    #[inline]
    pub fn with_right(mut self, val: u32) -> Self {
        self.0.right = val;
        self
    }

    #[inline]
    pub fn with_bottom(mut self, val: u32) -> Self {
        self.0.bottom = val;
        self
    }

    #[inline]
    pub fn with_back(mut self, val: u32) -> Self {
        self.0.back = val;
        self
    }
}

/// Describes a command queue.
///
/// For more information: [`D3D12_COMMAND_QUEUE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_command_queue_desc)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct CommandQueueDesc(pub(crate) D3D12_COMMAND_QUEUE_DESC);

impl CommandQueueDesc {
    #[inline]
    pub fn direct() -> Self {
        Self(D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            ..Default::default()
        })
    }

    #[inline]
    pub fn compute() -> Self {
        Self(D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_COMPUTE,
            ..Default::default()
        })
    }

    #[inline]
    pub fn copy() -> Self {
        Self(D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_COPY,
            ..Default::default()
        })
    }

    #[inline]
    pub fn video_decode() -> Self {
        Self(D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE,
            ..Default::default()
        })
    }

    #[inline]
    pub fn video_process() -> Self {
        Self(D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS,
            ..Default::default()
        })
    }

    #[inline]
    pub fn video_encode() -> Self {
        Self(D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE,
            ..Default::default()
        })
    }

    #[inline]
    pub fn with_priority(mut self, priority: CommandQueuePriority) -> Self {
        self.0.Priority = priority.as_raw();
        self
    }

    #[inline]
    pub fn with_flags(mut self, flags: CommandQueueFlags) -> Self {
        self.0.Flags = flags.as_raw();
        self
    }

    #[inline]
    pub fn with_node_mask(mut self, node_mask: u32) -> Self {
        self.0.NodeMask = node_mask;
        self
    }

    #[inline]
    pub fn r#type(&self) -> CommandListType {
        self.0.Type.into()
    }

    #[inline]
    pub fn priority(&self) -> CommandQueuePriority {
        self.0.Priority.into()
    }

    #[inline]
    pub fn flags(&self) -> CommandQueueFlags {
        self.0.Flags.into()
    }

    #[inline]
    pub fn node_mask(&self) -> u32 {
        self.0.NodeMask
    }
}

/// Describes the arguments (parameters) of a command signature.
///
/// For more information: [`D3D12_COMMAND_SIGNATURE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_command_signature_desc)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct CommandSignatureDesc<'a>(pub(crate) D3D12_COMMAND_SIGNATURE_DESC, PhantomData<&'a ()>);

impl<'a> CommandSignatureDesc<'a> {
    #[inline]
    pub fn with_byte_stride(mut self, byte_stride: u32) -> Self {
        self.0.ByteStride = byte_stride;
        self
    }

    #[inline]
    pub fn with_indirect_arguments(
        mut self,
        indirect_arguments: &'a [IndirectArgumentDesc],
    ) -> Self {
        self.0.NumArgumentDescs = indirect_arguments.len() as u32;
        self.0.pArgumentDescs = indirect_arguments.as_ptr() as *const _;
        self
    }

    #[inline]
    pub fn with_node_mask(mut self, node_mask: u32) -> Self {
        self.0.NodeMask = node_mask;
        self
    }
}

/// Describes a compute pipeline state object.
///
/// For more information: [`D3D12_COMPUTE_PIPELINE_STATE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_compute_pipeline_state_desc)
#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ComputePipelineStateDesc<'a>(
    pub(crate) D3D12_COMPUTE_PIPELINE_STATE_DESC,
    PhantomData<&'a ()>,
);

impl<'a> ComputePipelineStateDesc<'a> {
    #[inline]
    pub fn new(cs: &'a Blob) -> Self {
        Self(
            D3D12_COMPUTE_PIPELINE_STATE_DESC {
                CS: cs.as_shader_bytecode(),
                ..Default::default()
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn with_root_signature(mut self, root_signature: &'a RootSignature) -> Self {
        unsafe {
            self.0.pRootSignature = std::mem::transmute_copy(root_signature.as_raw());
            self
        }
    }

    #[inline]
    pub fn with_cache(mut self, cache: &'a Blob) -> Self {
        self.0.CachedPSO = cache.as_cached_pipeline_state();
        self
    }

    #[inline]
    pub fn with_flags(mut self, flags: PipelineStateFlags) -> Self {
        self.0.Flags = flags.as_raw();
        self
    }
}

/// Describes a constant buffer to view.
///
/// For more information: [`D3D12_CONSTANT_BUFFER_VIEW_DESC structure `](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_constant_buffer_view_desc)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct ConstantBufferViewDesc(pub(crate) D3D12_CONSTANT_BUFFER_VIEW_DESC);

impl ConstantBufferViewDesc {
    #[inline]
    pub fn new(buffer_location: u64, size: u32) -> Self {
        Self(D3D12_CONSTANT_BUFFER_VIEW_DESC {
            BufferLocation: buffer_location,
            SizeInBytes: size,
        })
    }
}

/// Type that represent return values of [`IDevice::get_copyable_footprints`](crate::device::IDevice::get_copyable_footprints)
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct CopyableFootprints {
    /// An array (of length NumSubresources) of [`PlacedSubresourceFootprint`] structures, to be filled with the description and placement of each subresource.
    pub layouts: SmallVec<[PlacedSubresourceFootprint; 8]>,

    /// An array (of length NumSubresources) of integer variables, to be filled with the number of rows for each subresource.
    pub num_rows: SmallVec<[u32; 8]>,

    /// An array (of length NumSubresources) of integer variables, each entry to be filled with the unpadded size in bytes of a row, of each subresource.
    pub row_sizes: SmallVec<[u64; 8]>,

    /// The total size, in bytes.
    pub total_bytes: u64,
}

/// Describes a CPU descriptor handle.
///
/// For more information: [`D3D12_CPU_DESCRIPTOR_HANDLE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_cpu_descriptor_handle)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct CpuDescriptorHandle(pub(crate) D3D12_CPU_DESCRIPTOR_HANDLE);

impl CpuDescriptorHandle {
    /// Returns a new handle with offset relative to the current handle.
    #[inline]
    pub fn offset(&self, offset: usize) -> Self {
        Self(D3D12_CPU_DESCRIPTOR_HANDLE {
            ptr: self.0.ptr + offset,
        })
    }
}

/// Describes a vertex element in a vertex buffer in an output slot.
///
/// For more information: [`D3D12_SO_DECLARATION_ENTRY structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_so_declaration_entry)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct DeclarationEntry(pub(crate) D3D12_SO_DECLARATION_ENTRY);

impl DeclarationEntry {
    #[inline]
    pub fn new(
        (semantic_name, semantic_index): (&'static CStr, u32),
        stream: u32,
        components: Range<u8>,
        output_slot: u8,
    ) -> Self {
        let semantic_name = PCSTR::from_raw(semantic_name.as_ref().as_ptr() as *const _);

        Self(D3D12_SO_DECLARATION_ENTRY {
            Stream: stream,
            SemanticName: semantic_name,
            SemanticIndex: semantic_index,
            StartComponent: components.start,
            ComponentCount: components.count() as u8,
            OutputSlot: output_slot,
        })
    }
}

/// Describes depth-stencil state.
///
/// For more information: [`D3D12_DEPTH_STENCIL_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_depth_stencil_desc)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct DepthStencilDesc(pub(crate) D3D12_DEPTH_STENCIL_DESC);

impl DepthStencilDesc {
    #[inline]
    pub fn enable_depth(mut self, depth_func: ComparisonFunc) -> Self {
        self.0.DepthEnable = true.into();
        self.0.DepthFunc = depth_func.as_raw();
        self
    }

    #[inline]
    pub fn with_depth_write_mask(mut self, mask: DepthWriteMask) -> Self {
        self.0.DepthWriteMask = mask.as_raw();
        self
    }

    #[inline]
    pub fn enable_stencil(mut self, stencil_read_mask: u8, stencil_write_mask: u8) -> Self {
        self.0.StencilEnable = true.into();
        self.0.StencilReadMask = stencil_read_mask;
        self.0.StencilWriteMask = stencil_write_mask;
        self
    }

    #[inline]
    pub fn with_front_face(mut self, front_face: DepthStencilOpDesc) -> Self {
        self.0.FrontFace = front_face.0;
        self
    }

    #[inline]
    pub fn with_back_face(mut self, back_face: DepthStencilOpDesc) -> Self {
        self.0.BackFace = back_face.0;
        self
    }
}

/// Describes stencil operations that can be performed based on the results of stencil test.
///
/// For more information: [`D3D12_DEPTH_STENCILOP_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_depth_stencilop_desc)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct DepthStencilOpDesc(pub(crate) D3D12_DEPTH_STENCILOP_DESC);

impl DepthStencilOpDesc {
    #[inline]
    pub fn with_stencil_fail_op(mut self, stencil_fail_op: StencilOp) -> Self {
        self.0.StencilFailOp = stencil_fail_op.as_raw();
        self
    }

    #[inline]
    pub fn with_stencil_depth_fail_op(mut self, stencil_depth_fail_op: StencilOp) -> Self {
        self.0.StencilDepthFailOp = stencil_depth_fail_op.as_raw();
        self
    }

    #[inline]
    pub fn with_stencil_pass_op(mut self, stencil_pass_op: StencilOp) -> Self {
        self.0.StencilDepthFailOp = stencil_pass_op.as_raw();
        self
    }

    #[inline]
    pub fn with_stencil_func(mut self, stencil_func: ComparisonFunc) -> Self {
        self.0.StencilFunc = stencil_func.as_raw();
        self
    }
}

/// Describes the subresources of a texture that are accessible from a depth-stencil view.
///
/// For more information: [`D3D12_DEPTH_STENCIL_VIEW_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_depth_stencil_view_desc)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DepthStencilViewDesc(pub(crate) D3D12_DEPTH_STENCIL_VIEW_DESC);

impl DepthStencilViewDesc {
    #[inline]
    pub fn texture_1d(format: Format, mip_slice: u32) -> Self {
        Self(D3D12_DEPTH_STENCIL_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_DSV_DIMENSION_TEXTURE1D,
            Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                Texture1D: D3D12_TEX1D_DSV {
                    MipSlice: mip_slice,
                },
            },
            Flags: D3D12_DSV_FLAG_NONE,
        })
    }

    #[inline]
    pub fn texture_1d_array(format: Format, mip_slice: u32, array: Range<u32>) -> Self {
        Self(D3D12_DEPTH_STENCIL_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_DSV_DIMENSION_TEXTURE1DARRAY,
            Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                Texture1DArray: D3D12_TEX1D_ARRAY_DSV {
                    MipSlice: mip_slice,
                    FirstArraySlice: array.start,
                    ArraySize: array.count() as u32,
                },
            },
            Flags: D3D12_DSV_FLAG_NONE,
        })
    }

    #[inline]
    pub fn texture_2d(format: Format, mip_slice: u32) -> Self {
        Self(D3D12_DEPTH_STENCIL_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_DSV_DIMENSION_TEXTURE2D,
            Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_DSV {
                    MipSlice: mip_slice,
                },
            },
            Flags: D3D12_DSV_FLAG_NONE,
        })
    }

    #[inline]
    pub fn texture_2d_array(format: Format, mip_slice: u32, array: Range<u32>) -> Self {
        Self(D3D12_DEPTH_STENCIL_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_DSV_DIMENSION_TEXTURE2DARRAY,
            Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                Texture2DArray: D3D12_TEX2D_ARRAY_DSV {
                    MipSlice: mip_slice,
                    FirstArraySlice: array.start,
                    ArraySize: array.count() as u32,
                },
            },
            Flags: D3D12_DSV_FLAG_NONE,
        })
    }

    #[inline]
    pub fn texture_2d_ms(format: Format) -> Self {
        Self(D3D12_DEPTH_STENCIL_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_DSV_DIMENSION_TEXTURE2DMS,
            Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                Texture2DMS: D3D12_TEX2DMS_DSV::default(),
            },
            Flags: D3D12_DSV_FLAG_NONE,
        })
    }

    #[inline]
    pub fn texture_2d_ms_array(format: Format, array: Range<u32>) -> Self {
        Self(D3D12_DEPTH_STENCIL_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY,
            Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                Texture2DMSArray: D3D12_TEX2DMS_ARRAY_DSV {
                    FirstArraySlice: array.start,
                    ArraySize: array.count() as u32,
                },
            },
            Flags: D3D12_DSV_FLAG_NONE,
        })
    }
}

/// Describes the descriptor heap.
///
/// For more information: [`D3D12_DESCRIPTOR_HEAP_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_descriptor_heap_desc)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct DescriptorHeapDesc(pub(crate) D3D12_DESCRIPTOR_HEAP_DESC);

impl DescriptorHeapDesc {
    #[inline]
    pub fn rtv(num: u32) -> Self {
        Self(D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            NumDescriptors: num,
            ..Default::default()
        })
    }

    #[inline]
    pub fn dsv(num: u32) -> Self {
        Self(D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
            NumDescriptors: num,
            ..Default::default()
        })
    }

    #[inline]
    pub fn cbr_srv_uav(num: u32) -> Self {
        Self(D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            NumDescriptors: num,
            ..Default::default()
        })
    }

    #[inline]
    pub fn sampler(num: u32) -> Self {
        Self(D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
            NumDescriptors: num,
            ..Default::default()
        })
    }

    #[inline]
    pub fn with_flags(mut self, flags: DescriptorHeapFlags) -> Self {
        self.0.Flags = flags.as_raw();
        self
    }

    #[inline]
    pub fn with_node_mask(mut self, node_mask: u32) -> Self {
        self.0.NodeMask = node_mask;
        self
    }

    #[inline]
    pub fn r#type(&self) -> DescriptorHeapType {
        self.0.Type.into()
    }

    #[inline]
    pub fn num_descriptors(&self) -> u32 {
        self.0.NumDescriptors
    }

    #[inline]
    pub fn flags(&self) -> DescriptorHeapFlags {
        self.0.Flags.into()
    }
}

/// Describes a descriptor range.
///
/// For more information: [`D3D12_DESCRIPTOR_RANGE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_descriptor_range)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct DescriptorRange(pub(crate) D3D12_DESCRIPTOR_RANGE);

impl DescriptorRange {
    #[inline]
    pub fn cbv(num: u32) -> Self {
        Self(D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_CBV,
            NumDescriptors: num,
            ..Default::default()
        })
    }

    #[inline]
    pub fn srv(num: u32) -> Self {
        Self(D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
            NumDescriptors: num,
            ..Default::default()
        })
    }

    #[inline]
    pub fn sampler(num: u32) -> Self {
        Self(D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER,
            NumDescriptors: num,
            ..Default::default()
        })
    }

    #[inline]
    pub fn uav(num: u32) -> Self {
        Self(D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_UAV,
            NumDescriptors: num,
            ..Default::default()
        })
    }

    #[inline]
    pub fn with_base_shader_register(mut self, base_shader_register: u32) -> Self {
        self.0.BaseShaderRegister = base_shader_register;
        self
    }

    #[inline]
    pub fn with_register_space(mut self, register_space: u32) -> Self {
        self.0.RegisterSpace = register_space;
        self
    }

    #[inline]
    pub fn with_offset_in_descriptors_from_table_start(
        mut self,
        offset_in_descriptors_from_table_start: u32,
    ) -> Self {
        self.0.OffsetInDescriptorsFromTableStart = offset_in_descriptors_from_table_start;
        self
    }
}

/// Describes details for the discard-resource operation.
///
/// For more information: [`D3D12_DISCARD_REGION structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_discard_region)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct DiscardRegion<'a>(pub(crate) D3D12_DISCARD_REGION, PhantomData<&'a ()>);

impl<'a> DiscardRegion<'a> {
    #[inline]
    pub fn new(rects: &'a [Rect], subresource: Range<u32>) -> Self {
        Self(
            D3D12_DISCARD_REGION {
                NumRects: rects.len() as u32,
                pRects: rects.as_ptr() as *const _,
                FirstSubresource: subresource.start,
                NumSubresources: subresource.count() as u32,
            },
            Default::default(),
        )
    }
}

/// Describes a GPU descriptor handle.
///
/// For more information: [`D3D12_GPU_DESCRIPTOR_HANDLE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_gpu_descriptor_handle)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct GpuDescriptorHandle(pub(crate) D3D12_GPU_DESCRIPTOR_HANDLE);

impl GpuDescriptorHandle {
    /// Returns a new handle with offset relative to the current handle.
    #[inline]
    pub fn offset(&self, offset: u64) -> Self {
        Self(D3D12_GPU_DESCRIPTOR_HANDLE {
            ptr: self.0.ptr + offset,
        })
    }
}

/// Describes a graphics pipeline state object.
///
/// For more information: [`D3D12_GRAPHICS_PIPELINE_STATE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_graphics_pipeline_state_desc)
#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct GraphicsPipelineDesc<'a>(
    pub(crate) D3D12_GRAPHICS_PIPELINE_STATE_DESC,
    PhantomData<&'a ()>,
);

impl<'a> GraphicsPipelineDesc<'a> {
    #[inline]
    pub fn new(vs: &'a Blob) -> Self {
        Self(
            D3D12_GRAPHICS_PIPELINE_STATE_DESC {
                VS: vs.as_shader_bytecode(),
                ..Default::default()
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn with_root_signature(mut self, root_signature: &'a RootSignature) -> Self {
        unsafe {
            self.0.pRootSignature = std::mem::transmute_copy(root_signature.as_raw());
            self
        }
    }

    #[inline]
    pub fn with_ps(mut self, ps: &'a Blob) -> Self {
        self.0.PS = ps.as_shader_bytecode();
        self
    }

    #[inline]
    pub fn with_ds(mut self, ds: &'a Blob) -> Self {
        self.0.DS = ds.as_shader_bytecode();
        self
    }

    #[inline]
    pub fn with_hs(mut self, hs: &'a Blob) -> Self {
        self.0.HS = hs.as_shader_bytecode();
        self
    }

    #[inline]
    pub fn with_gs(mut self, gs: &'a Blob) -> Self {
        self.0.GS = gs.as_shader_bytecode();
        self
    }

    #[inline]
    pub fn with_stream_output(mut self, stream_output: StreamOutputDesc<'a>) -> Self {
        self.0.StreamOutput = stream_output.0;
        self
    }

    #[inline]
    pub fn with_blend_desc(mut self, blend_desc: BlendDesc, sample_mask: u32) -> Self {
        self.0.BlendState = blend_desc.0;
        self.0.SampleMask = sample_mask;
        self
    }

    #[inline]
    pub fn with_rasterizer_state(mut self, rasterizer_state: RasterizerDesc) -> Self {
        self.0.RasterizerState = rasterizer_state.0;
        self
    }

    #[inline]
    pub fn with_depth_stencil(mut self, depth_stencil: DepthStencilDesc, format: Format) -> Self {
        self.0.DepthStencilState = depth_stencil.0;
        self.0.DSVFormat = format.as_raw();
        self
    }

    #[inline]
    pub fn with_input_layout(mut self, input_layout: &'a [InputElementDesc]) -> Self {
        self.0.InputLayout = D3D12_INPUT_LAYOUT_DESC {
            pInputElementDescs: input_layout.as_ptr() as *const _,
            NumElements: input_layout.len() as u32,
        };
        self
    }

    #[inline]
    pub fn with_ib_strip_cut_value(mut self, ib_strip_cut_value: IndexBufferStripCutValue) -> Self {
        self.0.IBStripCutValue = ib_strip_cut_value.as_raw();
        self
    }

    #[inline]
    pub fn with_primitive_topology(
        mut self,
        primitive_topology: PipelinePrimitiveTopology,
    ) -> Self {
        self.0.PrimitiveTopologyType = primitive_topology.as_raw();
        self
    }

    #[inline]
    pub fn with_render_targets(mut self, render_targets: impl IntoIterator<Item = Format>) -> Self {
        let mut rts = [DXGI_FORMAT_UNKNOWN; 8];
        let mut count = 0;

        for (i, desc) in render_targets.into_iter().take(8).enumerate() {
            rts[i] = desc.as_raw();
            count += 1;
        }

        self.0.RTVFormats = rts;
        self.0.NumRenderTargets = count;
        self
    }

    #[inline]
    pub fn with_sampler_desc(mut self, sampler_desc: SamplerDesc) -> Self {
        self.0.SampleDesc = sampler_desc.0;
        self
    }

    #[inline]
    pub fn with_node_mask(mut self, node_mask: u32) -> Self {
        self.0.NodeMask = node_mask;
        self
    }

    #[inline]
    pub fn with_cache(mut self, cache: &'a Blob) -> Self {
        self.0.CachedPSO = cache.as_cached_pipeline_state();
        self
    }

    #[inline]
    pub fn with_flags(mut self, flags: PipelineStateFlags) -> Self {
        self.0.Flags = flags.as_raw();
        self
    }
}

/// Describes a heap.
///
/// For more information: [`D3D12_HEAP_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_heap_desc)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct HeapDesc {
    /// The size, in bytes, of the heap. To avoid wasting memory, applications should pass size values which are multiples of the effective Alignment;
    /// but non-aligned size is also supported, for convenience.
    /// To find out how large a heap must be to support textures with undefined layouts and adapter-specific sizes, call [`crate::device::IDevice::get_resource_allocation_info`](crate::device::IDevice::get_resource_allocation_info)
    pub size: u64,

    /// A [`HeapProperties`] structure that describes the heap properties.
    pub props: HeapProperties,

    /// The alignment value for the heap.
    pub alignment: HeapAlignment,

    /// A combination of [`HeapFlags`]-typed values that are combined by using a bitwise-OR operation.
    /// The resulting value identifies heap options. When creating heaps to support adapters with resource heap tier 1, an application must choose some flags.
    pub flags: HeapFlags,
}

/// Describes heap properties.
///
/// For more information: [`D3D12_HEAP_PROPERTIES structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_heap_properties)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct HeapProperties {
    /// A [`HeapType`]-typed value that specifies the type of heap.
    pub r#type: HeapType,

    /// A [`CpuPageProperty`]-typed value that specifies the CPU-page properties for the heap.
    pub cpu_page_propery: CpuPageProperty,

    /// A [`MemoryPool`]-typed value that specifies the memory pool for the heap.
    pub memory_pool_preference: MemoryPool,

    /// For multi-adapter operation, this indicates the node where the resource should be created.
    ///
    /// Exactly one bit of this UINT must be set.
    ///
    /// Passing zero is equivalent to passing one, in order to simplify the usage of single-GPU adapters.
    pub creation_node_mask: u32,

    /// For multi-adapter operation, this indicates the set of nodes where the resource is visible.
    ///
    /// VisibleNodeMask must have the same bit set that is set in CreationNodeMask. VisibleNodeMask can also have additional bits set for cross-node resources, but doing so can potentially reduce performance for resource accesses, so you should do so only when needed.
    ///
    /// Passing zero is equivalent to passing one, in order to simplify the usage of single-GPU adapters.
    pub visible_node_mask: u32,
}

/// Describes the index buffer to view.
///
/// For more information: [`D3D12_INDEX_BUFFER_VIEW structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_index_buffer_view)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct IndexBufferView {
    /// The GPU virtual address of the index buffer.
    pub buffer_location: GpuVirtualAddress,

    /// The size in bytes of the index buffer.
    pub size_in_bytes: u32,

    /// A [`Format`]-typed value for the index-buffer format.
    pub format: Format,
}

/// Specifies the type of the indirect parameter.
///
/// For more information: [`D3D12_INDIRECT_ARGUMENT_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_indirect_argument_desc)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct IndirectArgumentDesc(pub(crate) D3D12_INDIRECT_ARGUMENT_DESC);

impl IndirectArgumentDesc {
    #[inline]
    pub fn draw() -> Self {
        Self(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_DRAW,
            Anonymous: Default::default(),
        })
    }

    #[inline]
    pub fn draw_indexed() -> Self {
        Self(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_DRAW_INDEXED,
            Anonymous: Default::default(),
        })
    }

    #[inline]
    pub fn dispatch() -> Self {
        Self(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH,
            Anonymous: Default::default(),
        })
    }

    #[inline]
    pub fn vertex_buffer_view(slot: u32) -> Self {
        Self(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW,
            Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
                VertexBuffer: D3D12_INDIRECT_ARGUMENT_DESC_0_4 { Slot: slot },
            },
        })
    }

    #[inline]
    pub fn index_buffer_view() -> Self {
        Self(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_INDEX_BUFFER_VIEW,
            Anonymous: Default::default(),
        })
    }

    #[inline]
    pub fn constant(
        root_parameter_index: u32,
        dest_offset_in32_bit_values: u32,
        num32_bit_values_to_set: u32,
    ) -> Self {
        Self(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT,
            Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
                Constant: D3D12_INDIRECT_ARGUMENT_DESC_0_1 {
                    RootParameterIndex: root_parameter_index,
                    DestOffsetIn32BitValues: dest_offset_in32_bit_values,
                    Num32BitValuesToSet: num32_bit_values_to_set,
                },
            },
        })
    }

    #[inline]
    pub fn constant_buffer_view(root_parameter_index: u32) -> Self {
        Self(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT_BUFFER_VIEW,
            Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
                ConstantBufferView: D3D12_INDIRECT_ARGUMENT_DESC_0_0 {
                    RootParameterIndex: root_parameter_index,
                },
            },
        })
    }

    #[inline]
    pub fn shader_resource_view(root_parameter_index: u32) -> Self {
        Self(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_SHADER_RESOURCE_VIEW,
            Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
                ShaderResourceView: D3D12_INDIRECT_ARGUMENT_DESC_0_2 {
                    RootParameterIndex: root_parameter_index,
                },
            },
        })
    }

    #[inline]
    pub fn unordered_access_view(root_parameter_index: u32) -> Self {
        Self(D3D12_INDIRECT_ARGUMENT_DESC {
            Type: D3D12_INDIRECT_ARGUMENT_TYPE_UNORDERED_ACCESS_VIEW,
            Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
                UnorderedAccessView: D3D12_INDIRECT_ARGUMENT_DESC_0_3 {
                    RootParameterIndex: root_parameter_index,
                },
            },
        })
    }
}

/// Describes a single element for the input-assembler stage of the graphics pipeline.
///
/// For more information: [`D3D12_INPUT_ELEMENT_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_input_element_desc)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct InputElementDesc(pub(crate) D3D12_INPUT_ELEMENT_DESC);

impl InputElementDesc {
    #[inline]
    pub fn per_vertex(
        (semantic_name, semantic_index): (&'static CStr, u32),
        format: Format,
        offset: u32,
        input_slot: u32,
    ) -> Self {
        let semantic_name = PCSTR::from_raw(semantic_name.as_ref().as_ptr() as *const _);

        Self(D3D12_INPUT_ELEMENT_DESC {
            SemanticName: semantic_name,
            SemanticIndex: semantic_index,
            Format: format.as_raw(),
            InputSlot: input_slot,
            AlignedByteOffset: offset,
            InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        })
    }

    #[inline]
    pub fn per_instance(
        (semantic_name, semantic_index): (&'static CStr, u32),
        format: Format,
        offset: u32,
        input_slot: u32,
        instance_data_step_rate: u32,
    ) -> Self {
        let semantic_name = PCSTR::from_raw(semantic_name.as_ref().as_ptr() as *const _);

        Self(D3D12_INPUT_ELEMENT_DESC {
            SemanticName: semantic_name,
            SemanticIndex: semantic_index,
            Format: format.as_raw(),
            InputSlot: input_slot,
            AlignedByteOffset: offset,
            InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA,
            InstanceDataStepRate: instance_data_step_rate,
        })
    }
}

/// The LUID structure is an opaque structure that specifies an identifier that is guaranteed to be unique on the local machine.
///
/// For more information: [`LUID structure`](https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-luid)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Luid(pub(crate) LUID);

/// Describes the tile structure of a tiled resource with mipmaps.
///
/// For more information: [`D3D12_PACKED_MIP_INFO structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_packed_mip_info)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct PackedMipDesc {
    /// The number of standard mipmaps in the tiled resource.
    pub num_standard_mips: u8,

    /// The number of packed mipmaps in the tiled resource.
    pub num_packed_mips: u8,

    /// The number of tiles for the packed mipmaps in the tiled resource.
    pub num_tiles_for_packed_mips: u32,

    /// The offset of the first packed tile for the resource in the overall range of tiles.
    pub start_tile_index_in_overall_resource: u32,
}

/// Describes the footprint of a placed subresource, including the offset and the [`SubresourceFootprint`].
///
/// For more information: [`D3D12_PLACED_SUBRESOURCE_FOOTPRINT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_placed_subresource_footprint)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct PlacedSubresourceFootprint {
    /// The offset of the subresource within the parent resource, in bytes. The offset between the start of the parent resource and this subresource.
    pub offset: u64,

    /// The format, width, height, depth, and row-pitch of the subresource, as a [`SubresourceFootprint`] structure.
    pub footprint: SubresourceFootprint,
}

/// Describes the purpose of a query heap. A query heap contains an array of individual queries.
///
/// For more information: [`D3D12_QUERY_HEAP_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_query_heap_desc)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct QueryHeapDesc {
    /// Specifies one member of [`QueryHeapType`].
    pub r#type: QueryHeapType,

    /// Specifies the number of queries the heap should contain.
    pub count: u32,

    /// For single GPU operation, set this to zero.
    /// If there are multiple GPU nodes, set a bit to identify the node (the device's physical adapter) to which the query heap applies.
    /// Each bit in the mask corresponds to a single node. Only 1 bit must be set.
    pub node_mask: u32,
}

/// Describes rasterizer state.
///
/// For more information: [`D3D12_RASTERIZER_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_rasterizer_desc)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RasterizerDesc {
    /// A [`FillMode`]-typed value that specifies the fill mode to use when rendering.
    pub fill_mode: FillMode,

    /// A [`CullMode`]-typed value that specifies that triangles facing the specified direction are not drawn.
    pub cull_mode: CullMode,

    /// Determines if a triangle is front- or back-facing.
    /// If this member is TRUE, a triangle will be considered front-facing if its vertices are counter-clockwise on the render target and considered back-facing
    /// if they are clockwise. If this parameter is FALSE, the opposite is true.
    pub front_counter_clockwise: bool,

    /// Depth value added to a given pixel. For info about depth bias.
    pub depth_bias: i32,

    /// Maximum depth bias of a pixel. For info about depth bias.
    pub depth_bias_clamp: f32,

    /// Scalar on a given pixel's slope.
    pub slope_scaled_depth_bias: f32,

    /// Specifies whether to enable clipping based on distance.
    pub depth_clip_enable: bool,

    /// Specifies whether to use the quadrilateral or alpha line anti-aliasing algorithm on multisample antialiasing (MSAA) render targets.
    /// Set to TRUE to use the quadrilateral line anti-aliasing algorithm and to FALSE to use the alpha line anti-aliasing algorithm.
    pub multisample_enable: bool,

    /// Specifies whether to enable line antialiasing; only applies if doing line drawing and MultisampleEnable is FALSE.
    pub antialiased_line_enable: bool,

    /// The sample count that is forced while UAV rendering or rasterizing. Valid values are 0, 1, 4, 8, and optionally 16. 0 indicates that the sample count is not forced.
    pub forced_sample_count: u32,

    /// A [`ConservativeRaster``]-typed value that identifies whether conservative rasterization is on or off.
    pub conservative_raster: ConservativeRaster,
}

/// Represents a rational number.
///
/// For more information: [`DXGI_RATIONAL structure`](https://learn.microsoft.com/en-us/windows/win32/api/dxgicommon/ns-dxgicommon-dxgi_rational)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Rational {
    /// An unsigned integer value representing the top of the rational number.
    pub numerator: u32,

    /// An unsigned integer value representing the bottom of the rational number.
    pub denominator: u32,
}

/// The RECT structure defines a rectangle by the coordinates of its upper-left and lower-right corners.
///
/// For more information: [`RECT structure`](https://learn.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Rect {
    /// Specifies the x-coordinate of the upper-left corner of the rectangle.
    pub left: i32,

    /// Specifies the y-coordinate of the upper-left corner of the rectangle.
    pub top: i32,

    /// Specifies the x-coordinate of the lower-right corner of the rectangle.
    pub right: i32,

    /// Specifies the y-coordinate of the lower-right corner of the rectangle.
    pub bottom: i32,
}

impl Rect {
    /// Create rect with left and top equal to 0.
    #[inline]
    pub fn with_size(size: impl Into<(i32, i32)>) -> Self {
        let (width, height) = size.into();

        Self {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        }
    }
}

/// Describes the blend state for a render target.
///
/// For more information: [`D3D12_RENDER_TARGET_BLEND_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_render_target_blend_desc)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct RenderTargetBlendDesc(pub(crate) D3D12_RENDER_TARGET_BLEND_DESC);

impl RenderTargetBlendDesc {
    /// Specifies blending.
    /// * `src_blend` - A [`Blend`]-typed value that specifies the operation to perform on the RGB value that the pixel shader outputs. The BlendOp member defines how to combine the src_blend and dst_blend operations.
    /// * `dst_blend` -  A [`Blend`]-typed value that specifies the operation to perform on the current RGB value in the render target. The BlendOp member defines how to combine the src_blend and dst_blend operations.
    /// * `blend_op` - A [`BlendOp]-typed value that defines how to combine the src_blend and dst_blend operations.
    /// * `mask` -  A combination of [`ColorWriteEnable`]-typed values that are combined by using a bitwise OR operation. The resulting value specifies a write mask.
    #[inline]
    pub fn blend(
        src_blend: Blend,
        dst_blend: Blend,
        blend_op: BlendOp,
        mask: ColorWriteEnable,
    ) -> Self {
        Self(D3D12_RENDER_TARGET_BLEND_DESC {
            BlendEnable: true.into(),
            SrcBlend: src_blend.as_raw(),
            DestBlend: dst_blend.as_raw(),
            BlendOp: blend_op.as_raw(),
            RenderTargetWriteMask: mask.bits() as u8,
            ..Default::default()
        })
    }

    /// Specifies blending with alpha.
    /// * `src_blend` - A [`Blend`]-typed value that specifies the operation to perform on the RGB value that the pixel shader outputs. The BlendOp member defines how to combine the src_blend and dst_blend operations.
    /// * `dst_blend` -  A [`Blend`]-typed value that specifies the operation to perform on the current RGB value in the render target. The BlendOp member defines how to combine the src_blend and dst_blend operations.
    /// * `blend_op` - A [`BlendOp]-typed value that defines how to combine the src_blend and dst_blend operations.
    /// * `src_blend_alpha` -A [`Blend`]-typed value that specifies the operation to perform on the alpha value that the pixel shader outputs.
    ///   Blend options that end in _COLOR are not allowed. The BlendOpAlpha member defines how to combine the src_blend_alpha and dst_blend_alpha operations.
    /// * `dst_blend_alpha` -  A [`Blend`]-typed value that specifies the operation to perform on the current alpha value in the render target.
    ///   Blend options that end in _COLOR are not allowed. The BlendOpAlpha member defines how to combine the src_blend_alpha and dst_blend_alpha operations.
    /// * `blend_op_alpha` - A [`BlendOp`]-typed value that defines how to combine the SrcBlendAlpha and DestBlendAlpha operations.
    /// * `mask` -  A combination of [`ColorWriteEnable`]-typed values that are combined by using a bitwise OR operation. The resulting value specifies a write mask.
    #[inline]
    pub fn blend_with_alpha(
        src_blend: Blend,
        dst_blend: Blend,
        blend_op: BlendOp,
        src_blend_alpha: Blend,
        dst_blend_alpha: Blend,
        blend_op_alpha: BlendOp,
        mask: ColorWriteEnable,
    ) -> Self {
        Self(D3D12_RENDER_TARGET_BLEND_DESC {
            BlendEnable: true.into(),
            SrcBlend: src_blend.as_raw(),
            DestBlend: dst_blend.as_raw(),
            BlendOp: blend_op.as_raw(),
            SrcBlendAlpha: src_blend_alpha.as_raw(),
            DestBlendAlpha: dst_blend_alpha.as_raw(),
            BlendOpAlpha: blend_op_alpha.as_raw(),
            RenderTargetWriteMask: mask.bits() as u8,
            ..Default::default()
        })
    }

    /// Specifies logic operation.
    /// * `logic_op` - A [`LogicOp`]-typed value that specifies the logical operation to configure for the render target.
    /// * `mask` -  A combination of [`ColorWriteEnable`]-typed values that are combined by using a bitwise OR operation. The resulting value specifies a write mask.
    #[inline]
    pub fn logic(logic_op: LogicOp, mask: ColorWriteEnable) -> Self {
        Self(D3D12_RENDER_TARGET_BLEND_DESC {
            LogicOpEnable: true.into(),
            LogicOp: logic_op.as_raw(),
            RenderTargetWriteMask: mask.bits() as u8,
            ..Default::default()
        })
    }
}

/// Describes the subresources from a resource that are accessible by using a render-target view.
///
/// For more information: [`D3D12_RENDER_TARGET_VIEW_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_render_target_view_desc)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct RenderTargetViewDesc(pub(crate) D3D12_RENDER_TARGET_VIEW_DESC);

impl RenderTargetViewDesc {
    #[inline]
    pub fn buffer(format: Format, elements_range: Range<u64>) -> Self {
        Self(D3D12_RENDER_TARGET_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_RTV_DIMENSION_BUFFER,
            Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Buffer: D3D12_BUFFER_RTV {
                    FirstElement: elements_range.start,
                    NumElements: elements_range.count() as u32,
                },
            },
        })
    }

    #[inline]
    pub fn texture_1d(format: Format, mip_slice: u32) -> Self {
        Self(D3D12_RENDER_TARGET_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_RTV_DIMENSION_TEXTURE1D,
            Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture1D: D3D12_TEX1D_RTV {
                    MipSlice: mip_slice,
                },
            },
        })
    }

    #[inline]
    pub fn texture_2d(format: Format, mip_slice: u32, plane_slice: u32) -> Self {
        Self(D3D12_RENDER_TARGET_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2D,
            Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_RTV {
                    MipSlice: mip_slice,
                    PlaneSlice: plane_slice,
                },
            },
        })
    }

    #[inline]
    pub fn texture_3d(format: Format, mip_slice: u32, w_slices: Range<u32>) -> Self {
        Self(D3D12_RENDER_TARGET_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_RTV_DIMENSION_TEXTURE3D,
            Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture3D: D3D12_TEX3D_RTV {
                    MipSlice: mip_slice,
                    FirstWSlice: w_slices.start,
                    WSize: w_slices.count() as u32,
                },
            },
        })
    }

    #[inline]
    pub fn texture_1d_array(format: Format, mip_slice: u32, array: Range<u32>) -> Self {
        Self(D3D12_RENDER_TARGET_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_RTV_DIMENSION_TEXTURE1DARRAY,
            Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture1DArray: D3D12_TEX1D_ARRAY_RTV {
                    MipSlice: mip_slice,
                    FirstArraySlice: array.start,
                    ArraySize: array.count() as u32,
                },
            },
        })
    }

    #[inline]
    pub fn texture_2d_array(
        format: Format,
        mip_slice: u32,
        plane_slice: u32,
        array: Range<u32>,
    ) -> Self {
        Self(D3D12_RENDER_TARGET_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2DARRAY,
            Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2DArray: D3D12_TEX2D_ARRAY_RTV {
                    MipSlice: mip_slice,
                    PlaneSlice: plane_slice,
                    FirstArraySlice: array.start,
                    ArraySize: array.count() as u32,
                },
            },
        })
    }

    #[inline]
    pub fn texture_2d_ms(format: Format) -> Self {
        Self(D3D12_RENDER_TARGET_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2DMS,
            Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2DMS: Default::default(),
            },
        })
    }

    #[inline]
    pub fn texture_2d_ms_array(format: Format, array: Range<u32>) -> Self {
        Self(D3D12_RENDER_TARGET_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY,
            Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2DMSArray: D3D12_TEX2DMS_ARRAY_RTV {
                    FirstArraySlice: array.start,
                    ArraySize: array.count() as u32,
                },
            },
        })
    }
}

/// Describes parameters needed to allocate resources.
///
/// For more information: [`D3D12_RESOURCE_ALLOCATION_INFO structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_resource_allocation_info)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct ResourceAllocationInfo(pub(crate) D3D12_RESOURCE_ALLOCATION_INFO);

impl ResourceAllocationInfo {
    #[inline]
    pub fn size(&self) -> u64 {
        self.0.SizeInBytes
    }

    #[inline]
    pub fn alignment(&self) -> u64 {
        self.0.Alignment
    }
}

/// Describes a resource barrier (transition in resource use).
///
/// For more information: [`D3D12_RESOURCE_BARRIER structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_resource_barrier)
#[derive(Clone)]
#[repr(transparent)]
pub struct ResourceBarrier<'a>(pub(crate) D3D12_RESOURCE_BARRIER, PhantomData<&'a ()>);

impl<'a> ResourceBarrier<'a> {
    #[inline]
    pub fn transition(
        resource: &'a Resource,
        subresource: u32,
        before: ResourceStates,
        after: ResourceStates,
    ) -> Self {
        Self(
            D3D12_RESOURCE_BARRIER {
                Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
                Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                        pResource: unsafe { std::mem::transmute_copy(resource.as_raw()) },
                        Subresource: subresource,
                        StateBefore: before.as_raw(),
                        StateAfter: after.as_raw(),
                    }),
                },
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn aliasing(before: &'a Resource, after: &'a Resource) -> Self {
        Self(
            D3D12_RESOURCE_BARRIER {
                Type: D3D12_RESOURCE_BARRIER_TYPE_ALIASING,
                Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    Aliasing: ManuallyDrop::new(D3D12_RESOURCE_ALIASING_BARRIER {
                        pResourceBefore: unsafe { std::mem::transmute_copy(before.as_raw()) },
                        pResourceAfter: unsafe { std::mem::transmute_copy(after.as_raw()) },
                    }),
                },
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn uav(resource: &'a Resource) -> Self {
        Self(
            D3D12_RESOURCE_BARRIER {
                Type: D3D12_RESOURCE_BARRIER_TYPE_UAV,
                Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    UAV: ManuallyDrop::new(D3D12_RESOURCE_UAV_BARRIER {
                        pResource: unsafe { std::mem::transmute_copy(resource.as_raw_ref()) },
                    }),
                },
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn with_flags(mut self, flags: ResourceBarrierFlags) -> Self {
        self.0.Flags = flags.as_raw();
        self
    }
}

/// Describes a resource, such as a texture. This structure is used extensively.
///
/// For more information: [`D3D12_RESOURCE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_resource_desc)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct ResourceDesc {
    /// One member of [`ResourceDimension`], specifying the dimensions of the resource.
    pub dimension: ResourceDimension,

    /// Specifies the alignment.
    pub alignment: HeapAlignment,

    /// Specifies the width of the resource.
    pub width: u64,

    /// Specifies the height of the resource.
    pub height: u32,

    /// Specifies the depth of the resource, if it is 3D, or the array size if it is an array of 1D or 2D resources.
    pub depth_or_array_size: u16,

    /// Specifies the number of MIP levels.
    pub mip_levels: u16,

    /// Specifies a [`SampleDesc`] structure.
    pub sample_desc: SampleDesc,

    /// Specifies one member of [`Format`].
    pub format: Format,

    /// Specifies one member of [`TextureLayout`].
    pub layout: TextureLayout,

    /// Bitwise-OR'd flags, as [`ResourceFlags`] enumeration constants.
    pub flags: ResourceFlags,
}

/// Describes the slot of a root signature version 1.0.
///
/// For more information: [`D3D12_ROOT_PARAMETER structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_root_parameter)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RootParameter<'a> {
    /// A [`RootParameterType`]-typed value that specifies the type of root signature slot. This member determines which type to use in the union below.
    pub r#type: RootParameterType<'a>,

    /// A [`ShaderVisibility`]-typed value that specifies the shaders that can access the contents of the root signature slot.
    pub visibility: ShaderVisibility,
}

/// Describes the layout of a root signature version 1.0.
///
/// For more information: [`D3D12_ROOT_SIGNATURE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_root_signature_desc)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RootSignatureDesc<'a> {
    /// An array of [`RootParameter`] structures for the slots in the root signature.
    pub parameters: &'a [RootParameter<'a>],

    /// Pointer to one or more [`StaticSamplerDesc`] structures.
    pub samplers: &'a [StaticSamplerDesc],

    /// A combination of [`RootSignatureFlags`]-typed values that are combined by using a bitwise OR operation. The resulting value specifies options for the root signature layout.
    pub flags: RootSignatureFlags,
}

/// Describes multi-sampling parameters for a resource.
///
/// For more information: [`DXGI_SAMPLE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/dxgicommon/ns-dxgicommon-dxgi_sample_desc)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct SampleDesc {
    /// The number of multisamples per pixel.
    pub count: u32,

    /// The image quality level. The higher the quality, the lower the performance.
    pub quality: u32,
}

/// Describes a sampler state.
///
/// For more information: [`D3D12_SAMPLER_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_sampler_desc)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SamplerDesc {
    /// A [`Filter`]-typed value that specifies the filtering method to use when sampling a texture.
    pub filter: Filter,

    /// Specifies the [`AddressMode`] mode to use for resolving a `u` texture coordinate that is outside the 0 to 1 range.
    pub address_u: AddressMode,

    /// Specifies the [`AddressMode`] mode to use for resolving a `v` texture coordinate that is outside the 0 to 1 range.
    pub address_v: AddressMode,

    /// Specifies the [`AddressMode`] mode to use for resolving a `w` texture coordinate that is outside the 0 to 1 range.
    pub address_w: AddressMode,

    /// Offset from the calculated mipmap level. For example, if Direct3D calculates that a texture should be sampled at mipmap level 3 and MipLODBias is 2, then the texture will be sampled at mipmap level 5.
    pub mip_lod_bias: f32,

    /// Clamping value used if [`Filter::Anisotropic`] or [`Filter::ComparisonAnisotropic`] is specified as the filter. Valid values are between 1 and 16.
    pub max_anisotropy: u32,

    /// A function that compares sampled data against existing sampled data. The function options are listed in [`ComparisonFunc`].
    pub comparison_func: ComparisonFunc,

    /// RGBA border color to use if [`AddressMode::Border`] is specified for AddressU, AddressV, or AddressW. Range must be between 0.0 and 1.0 inclusive.
    pub border_color: [f32; 4],

    /// Lower end of the mipmap range to clamp access to, where 0 is the largest and most detailed mipmap level and any level higher than that is less detailed.
    pub min_lod: f32,

    /// Upper end of the mipmap range to clamp access to, where 0 is the largest and most detailed mipmap level and any level higher than that is less detailed. This value must be greater than or equal to MinLOD. To have no upper limit on LOD set this to a large value such as D3D12_FLOAT32_MAX.
    pub max_lod: f32,
}

/// Describes a shader-resource view (SRV).
///
/// For more information: [`D3D12_SHADER_RESOURCE_VIEW_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_shader_resource_view_desc)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShaderResourceViewDesc {
    /// A [`Format`]-typed value that specifies the viewing format.
    pub format: Format,

    /// A [`SrvDimension`]-typed value that specifies the resource type of the view. This type is the same as the resource type of the underlying resource.
    pub dimension: SrvDimension,
}

/// A handle to the object of event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SharedHandle(pub(crate) HANDLE);

impl SharedHandle {
    pub fn close(self) -> Result<(), DxError> {
        unsafe { CloseHandle(self.0).map_err(DxError::from) }
    }
}

/// Describes a static sampler.
///
/// For more information: [`D3D12_STATIC_SAMPLER_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_static_sampler_desc)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StaticSamplerDesc {
    /// The filtering method to use when sampling a texture, as a [`Filter`] enumeration constant.
    pub filter: Filter,

    /// Specifies the [`AddressMode`] mode to use for resolving a `u` texture coordinate that is outside the 0 to 1 range.
    pub address_u: AddressMode,

    /// Specifies the [`AddressMode`] mode to use for resolving a `v` texture coordinate that is outside the 0 to 1 range.
    pub address_v: AddressMode,

    /// Specifies the [`AddressMode`] mode to use for resolving a `w` texture coordinate that is outside the 0 to 1 range.
    pub address_w: AddressMode,

    /// Offset from the calculated mipmap level. For example, if Direct3D calculates that a texture should be sampled at mipmap level 3 and MipLODBias is 2, then the texture will be sampled at mipmap level 5.
    pub mip_lod_bias: f32,

    /// Clamping value used if [`Filter::Anisotropic`] or [`Filter::ComparisonAnisotropic`] is specified as the filter. Valid values are between 1 and 16.
    pub max_anisotropy: u32,

    /// A function that compares sampled data against existing sampled data. The function options are listed in [`ComparisonFunc`].
    pub comparison_func: ComparisonFunc,

    /// One member of [`BorderColor`], the border color to use if [`AddressMode::Border`] is specified for AddressU, AddressV, or AddressW. Range must be between 0.0 and 1.0 inclusive.
    pub border_color: BorderColor,

    /// Lower end of the mipmap range to clamp access to, where 0 is the largest and most detailed mipmap level and any level higher than that is less detailed.
    pub min_lod: f32,

    /// Upper end of the mipmap range to clamp access to, where 0 is the largest and most detailed mipmap level and any level higher than that is less detailed. This value must be greater than or equal to MinLOD. To have no upper limit on LOD set this to a large value such as D3D12_FLOAT32_MAX.
    pub max_lod: f32,

    /// The ShaderRegister and RegisterSpace parameters correspond to the binding syntax of HLSL.
    pub shader_register: u32,

    /// See the description for ShaderRegister. Register space is optional; the default register space is 0.
    pub register_space: u32,

    /// Specifies the visibility of the sampler to the pipeline shaders, one member of [`ShaderVisibility`].
    pub visibility: ShaderVisibility,
}

/// Describes a stream output buffer.
///
/// For more information: [`D3D12_STREAM_OUTPUT_BUFFER_VIEW structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_stream_output_buffer_view)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct StreamOutputBufferView {
    /// A u64 that points to the stream output buffer. If `size_in_bytes` is 0, this member isn't used and can be any value.
    pub buffer_location: GpuVirtualAddress,

    /// The size of the stream output buffer in bytes.
    pub size_in_bytes: u64,

    /// The location of the value of how much data has been filled into the buffer, as a u64.
    /// This member can't be NULL; a filled size location must be supplied (which the hardware will increment as data is output). If `size_in_bytes` is 0, this member isn't used and can be any value.
    pub buffer_filled_size_location: u64,
}

/// Describes a streaming output buffer.
///
/// For more information: [`D3D12_STREAM_OUTPUT_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_stream_output_desc)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct StreamOutputDesc<'a>(pub(crate) D3D12_STREAM_OUTPUT_DESC, PhantomData<&'a ()>);

impl<'a> StreamOutputDesc<'a> {
    #[inline]
    pub fn new(entries: &'a [DeclarationEntry]) -> Self {
        Self(
            D3D12_STREAM_OUTPUT_DESC {
                pSODeclaration: entries.as_ptr() as *const _,
                NumEntries: entries.len() as u32,
                ..Default::default()
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn with_buffer_strides(mut self, buffer_strides: &'a [u32]) -> Self {
        self.0.pBufferStrides = buffer_strides.as_ptr();
        self.0.NumStrides = buffer_strides.len() as u32;
        self
    }

    #[inline]
    pub fn with_rasterized_stream(mut self, rasterized_stream: u32) -> Self {
        self.0.RasterizedStream = rasterized_stream;
        self
    }
}

/// Describes the format, width, height, depth, and row-pitch of the subresource into the parent resource.
///
/// For more information: [`D3D12_SUBRESOURCE_FOOTPRINT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_subresource_footprint)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct SubresourceFootprint {
    /// A [`Format`]-typed value that specifies the viewing format.
    pub format: Format,

    /// The width of the subresource.
    pub width: u32,

    /// The height of the subresource.
    pub height: u32,

    /// The depth of the subresource.
    pub depth: u32,

    /// The row pitch, or width, or physical size, in bytes, of the subresource data.
    /// This must be a multiple of [`TEXTURE_DATA_PITCH_ALIGNMENT`], and must be greater than or equal to the size of the data within a row.
    pub row_pitch: u32,
}

/// Describes a tiled subresource volume.
///
/// For more information: [`D3D12_SUBRESOURCE_TILING structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_subresource_tiling)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct SubresourceTiling {
    /// The width in tiles of the subresource.
    pub width_in_tiles: u32,

    /// The height in tiles of the subresource.
    pub height_in_tiles: u16,

    /// The depth in tiles of the subresource.
    pub depth_in_tiles: u16,

    /// The index of the tile in the overall tiled subresource to start with.
    pub start_tile_index_in_overall_resource: u32,
}

/// Describes a swap chain.
///
/// For more information: [`DXGI_SWAP_CHAIN_DESC1 structure`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/ns-dxgi1_2-dxgi_swap_chain_desc1)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct SwapchainDesc1 {
    /// A value that describes the resolution width.
    pub width: u32,

    /// A value that describes the resolution height.
    pub height: u32,

    /// A [`Format`] structure that describes the display format.
    pub format: Format,

    /// Specifies whether the full-screen display mode or the swap-chain back buffer is stereo.
    pub stereo: bool,

    /// A [`SampleDesc`] structure that describes multi-sampling parameters. This member is valid only with bit-block transfer (bitblt) model swap chains.
    pub sample_desc: SampleDesc,

    /// A [`FrameBufferUsage`]-typed value that describes the surface usage and CPU access options for the back buffer. The back buffer can be used for shader input or render-target output.
    pub usage: FrameBufferUsage,

    /// A value that describes the number of buffers in the swap chain. When you create a full-screen swap chain, you typically include the front buffer in this value.
    pub buffer_count: u32,

    /// A [`Scaling`]-typed value that identifies resize behavior if the size of the back buffer is not equal to the target output.
    pub scaling: Scaling,

    /// A [`SwapEffect`]-typed value that describes the presentation model that is used by the swap chain and options for handling the contents of the presentation buffer after presenting a surface.
    pub swap_effect: SwapEffect,

    /// A [`AlphaMode`]-typed value that identifies the transparency behavior of the swap-chain back buffer.
    pub alpha_mode: AlphaMode,

    /// A combination of [`SwapchainFlags`]-typed values that are combined by using a bitwise OR operation. The resulting value specifies options for swap-chain behavior.
    pub flags: SwapchainFlags,
}

/// Describes a swap chain.
///
/// For more information: [`DXGI_SWAP_CHAIN_FULLSCREEN_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/ns-dxgi1_2-dxgi_swap_chain_fullscreen_desc)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct SwapchainFullscreenDesc {
    /// A [`Rational`] structure that describes the refresh rate in hertz.
    pub rational: Rational,

    /// A member of the [`ScanlineOrdering`] enumerated type that describes the scan-line drawing mode.
    pub scanline_ordering: ScanlineOrdering,

    /// A member of the [`ScalingMode`] enumerated type that describes the scaling mode.
    pub scaling: ScalingMode,

    /// A Boolean value that specifies whether the swap chain is in windowed mode.
    pub windowed: bool,
}

/// Describes a portion of a texture for the purpose of texture copies.
///
/// For more information: [`D3D12_TEXTURE_COPY_LOCATION structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_texture_copy_location)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextureCopyLocation<'a> {
    /// Specifies the resource which will be used for the copy operation.
    pub resource: &'a Resource,

    /// Specifies which type of resource location this is: a subresource of a texture, or a description of a texture layout which can be applied to a buffer.
    pub r#type: TextureCopyType,
}

/// Describes the size of a tiled region.
///
/// For more information: [`D3D12_TILE_REGION_SIZE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_tile_region_size)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct TileRegionSize {
    /// The number of tiles in the tiled region.
    pub num_tiles: u32,

    /// Specifies whether the runtime uses the **width**, **height**, and **depth** members to define the region.
    pub use_box: bool,

    /// The width of the tiled region, in tiles. Used for buffer and 1D, 2D, and 3D textures.
    pub width: u32,

    /// The height of the tiled region, in tiles. Used for 2D and 3D textures.
    pub height: u16,

    /// The depth of the tiled region, in tiles. Used for 3D textures or arrays.
    /// For arrays, used for advancing in depth jumps to next slice of same mipmap size, which isn't contiguous in the subresource counting space
    /// if there are multiple mipmaps.
    pub depth: u16,
}

/// Describes the shape of a tile by specifying its dimensions.
///
/// For more information: [`D3D12_TILE_SHAPE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_tile_shape)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct TileShape {
    /// The width in texels of the tile.
    pub width_in_texels: u32,

    /// The height in texels of the tile.
    pub height_in_texels: u32,

    /// The depth in texels of the tile.
    pub depth_in_texels: u32,
}

/// Describes the coordinates of a tiled resource.
///
/// For more information: [`D3D12_TILED_RESOURCE_COORDINATE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_tiled_resource_coordinate)
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct TiledResourceCoordinate {
    /// The x-coordinate of the tiled resource.
    pub x: u32,

    /// The y-coordinate of the tiled resource.
    pub y: u32,

    /// The z-coordinate of the tiled resource.
    pub z: u32,

    /// The index of the subresource for the tiled resource.
    pub subresource: u32,
}

/// Describes the subresources from a resource that are accessible by using an unordered-access view.
///
/// For more information: [`D3D12_UNORDERED_ACCESS_VIEW_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_unordered_access_view_desc)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct UnorderedAccessViewDesc {
    /// A [`Format`]-typed value that specifies the viewing format.
    pub format: Format,

    /// A [`UavDimension`]-typed value that specifies the resource type of the view. This type specifies how the resource will be accessed.
    pub dimension: UavDimension,
}

/// Describes a vertex buffer view.
///
/// For more information: [`D3D12_VERTEX_BUFFER_VIEW structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_vertex_buffer_view)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct VertexBufferView {
    /// Specifies a u64 that identifies the address of the buffer.
    pub buffer_location: GpuVirtualAddress,

    /// Specifies the size in bytes of the buffer.
    pub stride_in_bytes: u32,

    /// Specifies the size in bytes of each vertex entry.
    pub size_in_bytes: u32,
}

/// Describes the dimensions of a viewport.
///
/// For more information: [`D3D12_VIEWPORT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_viewport)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Viewport {
    /// X position of the left hand side of the viewport.
    pub x: f32,

    /// Y position of the top of the viewport.
    pub y: f32,

    /// Width of the viewport.
    pub width: f32,

    /// Height of the viewport.
    pub height: f32,

    /// Minimum depth of the viewport. Ranges between 0 and 1.
    pub min_depth: f32,

    /// Maximum depth of the viewport. Ranges between 0 and 1.
    pub max_depth: f32,
}

impl Viewport {
    /// Creates a viewport with a minimum depth of 0 and a maximum depth of 1.
    #[inline]
    pub fn with_position_and_size(
        position: impl Into<(f32, f32)>,
        size: impl Into<(f32, f32)>,
    ) -> Self {
        let (width, height) = size.into();
        let (x, y) = position.into();

        Self {
            x,
            y,
            width,
            height,
            min_depth: MIN_DEPTH,
            max_depth: MAX_DEPTH,
        }
    }

    /// Creates a viewport with a minimum depth of 0 and a maximum depth of 1 and with position in (0, 0).
    #[inline]
    pub fn with_size(size: impl Into<(f32, f32)>) -> Self {
        Self::with_position_and_size((0.0, 0.0), size)
    }
}
