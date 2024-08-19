use std::{ffi::CStr, marker::PhantomData, mem::ManuallyDrop, ops::Range};

use compact_str::CompactString;
use smallvec::SmallVec;
use windows::{
    core::PCSTR,
    Win32::Foundation::{CloseHandle, HANDLE, LUID, RECT},
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

/// Describes a value used to optimize clear operations for a particular resource.
///
/// For more information: [`D3D12_CLEAR_VALUE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_clear_value)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ClearValue(pub(crate) D3D12_CLEAR_VALUE);

impl ClearValue {
    #[inline]
    pub fn color(format: Format, value: impl Into<[f32; 4]>) -> Self {
        Self(D3D12_CLEAR_VALUE {
            Format: format.as_raw(),
            Anonymous: D3D12_CLEAR_VALUE_0 {
                Color: value.into(),
            },
        })
    }

    #[inline]
    pub fn depth(format: Format, depth: f32, stencil: u8) -> Self {
        Self(D3D12_CLEAR_VALUE {
            Format: format.as_raw(),
            Anonymous: D3D12_CLEAR_VALUE_0 {
                DepthStencil: D3D12_DEPTH_STENCIL_VALUE {
                    Depth: depth,
                    Stencil: stencil,
                },
            },
        })
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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
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
    pub fn with_sample_desc(mut self, sample_desc: SampleDesc) -> Self {
        self.0.SampleDesc = sample_desc.0;
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct HeapDesc(pub(crate) D3D12_HEAP_DESC);

impl HeapDesc {
    #[inline]
    pub fn new(size: u64, props: HeapProperties) -> Self {
        Self(D3D12_HEAP_DESC {
            SizeInBytes: size,
            Properties: props.0,
            ..Default::default()
        })
    }

    #[inline]
    pub fn with_alignment(mut self, alignment: HeapAlignment) -> Self {
        self.0.Alignment = alignment.as_raw();
        self
    }

    #[inline]
    pub fn with_flags(mut self, flags: HeapFlags) -> Self {
        self.0.Flags = flags.as_raw();
        self
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.0.SizeInBytes as usize
    }

    #[inline]
    pub fn properties(&self) -> &HeapProperties {
        unsafe {
            std::mem::transmute(&self.0.Properties)
        }
    }

    #[inline]
    pub fn alignement(&self) -> HeapAlignment {
        self.0.Alignment.into()
    }

    #[inline]
    pub fn flags(&self) -> HeapFlags {
        self.0.Flags.into()
    }
}

/// Describes heap properties.
///
/// For more information: [`D3D12_HEAP_PROPERTIES structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_heap_properties)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct HeapProperties(pub(crate) D3D12_HEAP_PROPERTIES);

impl HeapProperties {
    #[inline]
    pub fn new(
        r#type: HeapType,
        cpu_page_property: CpuPageProperty,
        memory_pool_preference: MemoryPool,
    ) -> Self {
        Self(D3D12_HEAP_PROPERTIES {
            Type: r#type.as_raw(),
            CPUPageProperty: cpu_page_property.as_raw(),
            MemoryPoolPreference: memory_pool_preference.as_raw(),
            ..Default::default()
        })
    }

    #[inline]
    pub fn upload() -> Self {
        Self(D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            ..Default::default()
        })
    }

    #[inline]
    pub fn readback() -> Self {
        Self(D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_READBACK,
            ..Default::default()
        })
    }

    #[inline]
    pub fn custom() -> Self {
        Self(D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_CUSTOM,
            ..Default::default()
        })
    }

    #[inline]
    pub fn gpu_upload() -> Self {
        Self(D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_GPU_UPLOAD,
            ..Default::default()
        })
    }

    #[inline]
    pub fn with_cpu_page_property(mut self, cpu_page_property: CpuPageProperty) -> Self {
        self.0.CPUPageProperty = cpu_page_property.as_raw();
        self
    }

    #[inline]
    pub fn with_memory_pool_preference(mut self, memory_pool_preference: MemoryPool) -> Self {
        self.0.MemoryPoolPreference = memory_pool_preference.as_raw();
        self
    }

    #[inline]
    pub fn with_creation_node_mask(mut self, mask: u32) -> Self {
        self.0.CreationNodeMask = mask;
        self
    }

    #[inline]
    pub fn with_visible_node_mask(mut self, mask: u32) -> Self {
        self.0.VisibleNodeMask = mask;
        self
    }

    #[inline]
    pub fn r#type(&self) -> HeapType {
        self.0.Type.into()
    }

    #[inline]
    pub fn cpu_page_property(&self) -> CpuPageProperty {
        self.0.CPUPageProperty.into()
    }

    #[inline]
    pub fn memory_pool_preference(&self) -> MemoryPool {
        self.0.MemoryPoolPreference.into()
    }

    #[inline]
    pub fn visible_node_mask(&self) -> u32 {
        self.0.VisibleNodeMask.into()
    }

    #[inline]
    pub fn creation_node_mask(&self) -> u32 {
        self.0.CreationNodeMask.into()
    }
}

impl Default for HeapProperties {
    fn default() -> Self {
        Self(D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_DEFAULT,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        })
    }
}

/// Describes the index buffer to view.
///
/// For more information: [`D3D12_INDEX_BUFFER_VIEW structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_index_buffer_view)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct IndexBufferView(pub(crate) D3D12_INDEX_BUFFER_VIEW);

impl IndexBufferView {
    #[inline]
    pub fn new(buffer_location: GpuVirtualAddress, size: u32, format: Format) -> Self {
        Self(D3D12_INDEX_BUFFER_VIEW {
            BufferLocation: buffer_location,
            SizeInBytes: size,
            Format: format.as_raw(),
        })
    }
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
#[repr(transparent)]
pub struct Luid(pub(crate) LUID);

impl Luid {
    #[inline]
    pub fn high_part(&self) -> i32 {
        self.0.HighPart
    }

    #[inline]
    pub fn low_part(&self) -> u32 {
        self.0.LowPart
    }
}

/// Describes the tile structure of a tiled resource with mipmaps.
///
/// For more information: [`D3D12_PACKED_MIP_INFO structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_packed_mip_info)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct PackedMipDesc(pub(crate) D3D12_PACKED_MIP_INFO);

impl PackedMipDesc {
    #[inline]
    pub fn num_standard_mips(&self) -> u8 {
        self.0.NumStandardMips
    }

    #[inline]
    pub fn num_packed_mips(&self) -> u8 {
        self.0.NumPackedMips
    }

    #[inline]
    pub fn num_tiles_for_packed_mips(&self) -> u32 {
        self.0.NumTilesForPackedMips
    }

    #[inline]
    pub fn start_tile_index_in_overall_resource(&self) -> u32 {
        self.0.StartTileIndexInOverallResource
    }
}

/// Describes the footprint of a placed subresource, including the offset and the [`SubresourceFootprint`].
///
/// For more information: [`D3D12_PLACED_SUBRESOURCE_FOOTPRINT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_placed_subresource_footprint)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PlacedSubresourceFootprint(pub(crate) D3D12_PLACED_SUBRESOURCE_FOOTPRINT);

impl PlacedSubresourceFootprint {
    #[inline]
    pub fn new(offset: u64, footprint: SubresourceFootprint) -> Self {
        Self(D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
            Offset: offset,
            Footprint: footprint.0,
        })
    }

    #[inline]
    pub fn offset(&self) -> u64 {
        self.0.Offset
    }

    #[inline]
    pub fn footprint(&self) -> &SubresourceFootprint {
        unsafe {
            std::mem::transmute(&self.0.Footprint)
        }
    }
}

/// Describes the purpose of a query heap. A query heap contains an array of individual queries.
///
/// For more information: [`D3D12_QUERY_HEAP_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_query_heap_desc)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct QueryHeapDesc(pub(crate) D3D12_QUERY_HEAP_DESC);

impl QueryHeapDesc {
    #[inline]
    pub fn occlusion(count: u32) -> Self {
        Self(D3D12_QUERY_HEAP_DESC {
            Type: D3D12_QUERY_HEAP_TYPE_OCCLUSION,
            Count: count,
            NodeMask: 0,
        })
    }

    #[inline]
    pub fn timestamp(count: u32) -> Self {
        Self(D3D12_QUERY_HEAP_DESC {
            Type: D3D12_QUERY_HEAP_TYPE_TIMESTAMP,
            Count: count,
            NodeMask: 0,
        })
    }

    #[inline]
    pub fn pipeline_statistics(count: u32) -> Self {
        Self(D3D12_QUERY_HEAP_DESC {
            Type: D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS,
            Count: count,
            NodeMask: 0,
        })
    }

    #[inline]
    pub fn so_statistics(count: u32) -> Self {
        Self(D3D12_QUERY_HEAP_DESC {
            Type: D3D12_QUERY_HEAP_TYPE_SO_STATISTICS,
            Count: count,
            NodeMask: 0,
        })
    }

    #[inline]
    pub fn video_decode_statistics(count: u32) -> Self {
        Self(D3D12_QUERY_HEAP_DESC {
            Type: D3D12_QUERY_HEAP_TYPE_VIDEO_DECODE_STATISTICS,
            Count: count,
            NodeMask: 0,
        })
    }

    #[inline]
    pub fn copy_queue_timestamp(count: u32) -> Self {
        Self(D3D12_QUERY_HEAP_DESC {
            Type: D3D12_QUERY_HEAP_TYPE_COPY_QUEUE_TIMESTAMP,
            Count: count,
            NodeMask: 0,
        })
    }

    #[inline]
    pub fn pipeline_statistics1(count: u32) -> Self {
        Self(D3D12_QUERY_HEAP_DESC {
            Type: D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS1,
            Count: count,
            NodeMask: 0,
        })
    }

    #[inline]
    pub fn with_node_mask(mut self, node_mask: u32) -> Self {
        self.0.NodeMask = node_mask;
        self
    }
}

/// Describes rasterizer state.
///
/// For more information: [`D3D12_RASTERIZER_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_rasterizer_desc)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct RasterizerDesc(pub(crate) D3D12_RASTERIZER_DESC);

impl RasterizerDesc {
    #[inline]
    pub fn with_fill_mode(mut self, fill_mode: FillMode) -> Self {
        self.0.FillMode = fill_mode.as_raw();
        self
    }

    #[inline]
    pub fn with_cull_mode(mut self, cull_mode: CullMode) -> Self {
        self.0.CullMode = cull_mode.as_raw();
        self
    }

    #[inline]
    pub fn enable_front_facing(mut self) -> Self {
        self.0.FrontCounterClockwise = true.into();
        self
    }

    #[inline]
    pub fn with_depth_bias(mut self, depth_bias: i32) -> Self {
        self.0.DepthBias = depth_bias;
        self
    }

    #[inline]
    pub fn with_depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
        self.0.DepthBiasClamp = depth_bias_clamp;
        self
    }

    #[inline]
    pub fn with_slope_scaled_depth_bias(mut self, slope_scaled_depth_bias: f32) -> Self {
        self.0.SlopeScaledDepthBias = slope_scaled_depth_bias;
        self
    }

    #[inline]
    pub fn enable_multisample(mut self) -> Self {
        self.0.MultisampleEnable = true.into();
        self
    }

    #[inline]
    pub fn enable_antialiased_line(mut self) -> Self {
        self.0.AntialiasedLineEnable = true.into();
        self
    }

    #[inline]
    pub fn with_forced_sample_count(mut self, forced_sample_count: u32) -> Self {
        self.0.ForcedSampleCount = forced_sample_count;
        self
    }

    #[inline]
    pub fn with_conservative_raster(mut self, conservative_raster: ConservativeRaster) -> Self {
        self.0.ConservativeRaster = conservative_raster.as_raw();
        self
    }
}

/// Represents a rational number.
///
/// For more information: [`DXGI_RATIONAL structure`](https://learn.microsoft.com/en-us/windows/win32/api/dxgicommon/ns-dxgicommon-dxgi_rational)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Rational(pub(crate) DXGI_RATIONAL);

impl Rational {
    #[inline]
    pub fn new(numerator: u32, denominator: u32) -> Self {
        Self(DXGI_RATIONAL {
            Numerator: numerator,
            Denominator: denominator,
        })
    }
}

/// The RECT structure defines a rectangle by the coordinates of its upper-left and lower-right corners.
///
/// For more information: [`RECT structure`](https://learn.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Rect(pub(crate) RECT);

impl Rect {
    /// Create rect with left and top equal to 0.
    #[inline]
    pub fn with_size(mut self, (width, height): (i32, i32)) -> Self {
        self.0.bottom = height;
        self.0.right = width;
        self
    }

    #[inline]
    pub fn with_left(mut self, left: i32) -> Self {
        self.0.left = left;
        self
    }

    #[inline]
    pub fn with_top(mut self, top: i32) -> Self {
        self.0.top = top;
        self
    }
}

/// Describes the blend state for a render target.
///
/// For more information: [`D3D12_RENDER_TARGET_BLEND_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_render_target_blend_desc)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct RenderTargetBlendDesc(pub(crate) D3D12_RENDER_TARGET_BLEND_DESC);

impl RenderTargetBlendDesc {
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ResourceDesc(pub(crate) D3D12_RESOURCE_DESC);

impl ResourceDesc {
    #[inline]
    pub fn buffer(size: u64) -> Self {
        Self(D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Width: size,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Alignment: HeapAlignment::Default.as_raw(),
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            ..Default::default()
        })
    }

    #[inline]
    pub fn texture_1d(width: u64) -> Self {
        Self(D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE1D,
            Width: width,
            Height: 1,
            DepthOrArraySize: 1,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            ..Default::default()
        })
    }

    #[inline]
    pub fn texture_2d(width: u64, height: u32) -> Self {
        Self(D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            Width: width,
            Height: height,
            DepthOrArraySize: 1,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            ..Default::default()
        })
    }

    #[inline]
    pub fn texture_3d(width: u64, height: u32, depth: u16) -> Self {
        Self(D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            Width: width,
            Height: height,
            DepthOrArraySize: depth,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            ..Default::default()
        })
    }

    #[inline]
    pub fn with_alignment(mut self, alignment: HeapAlignment) -> Self {
        self.0.Alignment = alignment.as_raw();
        self
    }

    #[inline]
    pub fn with_array_size(mut self, size: u16) -> Self {
        self.0.DepthOrArraySize = size;
        self
    }

    #[inline]
    pub fn with_format(mut self, format: Format) -> Self {
        self.0.Format = format.as_raw();
        self
    }

    #[inline]
    pub fn with_mip_levels(mut self, mip_levels: u16) -> Self {
        self.0.MipLevels = mip_levels;
        self
    }

    #[inline]
    pub fn with_sample_desc(mut self, sample_desc: SampleDesc) -> Self {
        self.0.SampleDesc = sample_desc.0;
        self
    }

    #[inline]
    pub fn with_layout(mut self, layout: TextureLayout) -> Self {
        self.0.Layout = layout.as_raw();
        self
    }

    #[inline]
    pub fn with_flags(mut self, flags: ResourceFlags) -> Self {
        self.0.Flags = flags.as_raw();
        self
    }

    #[inline]
    pub fn dimension(&self) -> ResourceDimension {
        self.0.Dimension.into()
    }

    #[inline]
    pub fn width(&self) -> u64 {
        self.0.Width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.0.Height
    }

    #[inline]
    pub fn depth_or_array_size(&self) -> u16 {
        self.0.DepthOrArraySize
    }

    #[inline]
    pub fn alignment(&self) -> HeapAlignment {
        self.0.Alignment.into()
    }

    #[inline]
    pub fn format(&self) -> Format {
        self.0.Format.into()
    }

    #[inline]
    pub fn mip_levels(&self) -> u16 {
        self.0.MipLevels
    }

    #[inline]
    pub fn sample_desc(&self) -> SampleDesc {
        SampleDesc(self.0.SampleDesc)
    }

    #[inline]
    pub fn layout(&self) -> TextureLayout {
        self.0.Layout.into()
    }

    #[inline]
    pub fn flags(&self) -> ResourceFlags {
        self.0.Flags.into()
    }
}

/// Describes the slot of a root signature version 1.0.
///
/// For more information: [`D3D12_ROOT_PARAMETER structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_root_parameter)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct RootParameter<'a>(pub(crate) D3D12_ROOT_PARAMETER, PhantomData<&'a ()>);

impl<'a> RootParameter<'a> {
    #[inline]
    pub fn descriptor_table(ranges: &'a [DescriptorRange]) -> Self {
        Self(
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE {
                        NumDescriptorRanges: ranges.len() as u32,
                        pDescriptorRanges: ranges.as_ptr() as *const _,
                    },
                },
                ..Default::default()
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn constant_32bit(
        shader_register: u32,
        register_space: u32,
        num_32bit_values: u32,
    ) -> Self {
        Self(
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    Constants: D3D12_ROOT_CONSTANTS {
                        ShaderRegister: shader_register,
                        RegisterSpace: register_space,
                        Num32BitValues: num_32bit_values,
                    },
                },
                ..Default::default()
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn cbv(shader_register: u32, register_space: u32) -> Self {
        Self(
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_CBV,
                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    Descriptor: D3D12_ROOT_DESCRIPTOR {
                        ShaderRegister: shader_register,
                        RegisterSpace: register_space,
                    },
                },
                ..Default::default()
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn srv(shader_register: u32, register_space: u32) -> Self {
        Self(
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_SRV,
                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    Descriptor: D3D12_ROOT_DESCRIPTOR {
                        ShaderRegister: shader_register,
                        RegisterSpace: register_space,
                    },
                },
                ..Default::default()
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn uav(shader_register: u32, register_space: u32) -> Self {
        Self(
            D3D12_ROOT_PARAMETER {
                ParameterType: D3D12_ROOT_PARAMETER_TYPE_UAV,
                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    Descriptor: D3D12_ROOT_DESCRIPTOR {
                        ShaderRegister: shader_register,
                        RegisterSpace: register_space,
                    },
                },
                ..Default::default()
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn with_visibility(mut self, visibility: ShaderVisibility) -> Self {
        self.0.ShaderVisibility = visibility.as_raw();
        self
    }
}

/// Describes the layout of a root signature version 1.0.
///
/// For more information: [`D3D12_ROOT_SIGNATURE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_root_signature_desc)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct RootSignatureDesc<'a>(pub(crate) D3D12_ROOT_SIGNATURE_DESC, PhantomData<&'a ()>);

impl<'a> RootSignatureDesc<'a> {
    #[inline]
    pub fn with_parameters<'b>(mut self, parameters: &'a [RootParameter<'b>]) -> Self
    where
        'a: 'b,
    {
        self.0.NumParameters = parameters.len() as u32;
        self.0.pParameters = parameters.as_ptr() as *const _;
        self
    }

    #[inline]
    pub fn with_sampler<'b>(mut self, samplers: &'a [StaticSamplerDesc]) -> Self
    where
        'a: 'b,
    {
        self.0.NumStaticSamplers = samplers.len() as u32;
        self.0.pStaticSamplers = samplers.as_ptr() as *const _;
        self
    }

    #[inline]
    pub fn with_flags(mut self, flags: RootSignatureFlags) -> Self {
        self.0.Flags = flags.as_raw();
        self
    }
}

/// Describes multi-sampling parameters for a resource.
///
/// For more information: [`DXGI_SAMPLE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/dxgicommon/ns-dxgicommon-dxgi_sample_desc)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct SampleDesc(pub(crate) DXGI_SAMPLE_DESC);

impl SampleDesc {
    #[inline]
    pub fn new(count: u32, quality: u32) -> Self {
        Self(DXGI_SAMPLE_DESC {
            Count: count,
            Quality: quality,
        })
    }
}

impl Default for SampleDesc {
    fn default() -> Self {
        Self::new(1, 0)
    }
}

/// Describes a sampler state.
///
/// For more information: [`D3D12_SAMPLER_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_sampler_desc)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct SamplerDesc(pub(crate) D3D12_SAMPLER_DESC);

impl SamplerDesc {
    #[inline]
    pub fn point() -> Self {
        Self::default().with_filter(Filter::Point)
    }

    #[inline]
    pub fn linear() -> Self {
        Self::default().with_filter(Filter::Linear)
    }

    #[inline]
    pub fn anisotropic() -> Self {
        Self::default().with_filter(Filter::Anisotropic)
    }

    #[inline]
    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.0.Filter = filter.as_raw();
        self
    }

    #[inline]
    pub fn with_address_u(mut self, address: AddressMode) -> Self {
        self.0.AddressU = address.as_raw();
        self
    }

    #[inline]
    pub fn with_address_v(mut self, address: AddressMode) -> Self {
        self.0.AddressV = address.as_raw();
        self
    }

    #[inline]
    pub fn with_address_w(mut self, address: AddressMode) -> Self {
        self.0.AddressW = address.as_raw();
        self
    }

    #[inline]
    pub fn with_mip_lod_bias(mut self, mip_lod_bias: f32) -> Self {
        self.0.MipLODBias = mip_lod_bias;
        self
    }

    #[inline]
    pub fn with_max_anisotropy(mut self, max_anisotropy: u32) -> Self {
        self.0.MaxAnisotropy = max_anisotropy;
        self
    }

    #[inline]
    pub fn with_comparison_func(mut self, comparison_func: ComparisonFunc) -> Self {
        self.0.ComparisonFunc = comparison_func.as_raw();
        self
    }

    #[inline]
    pub fn with_border_color(mut self, border_color: impl Into<[f32; 4]>) -> Self {
        self.0.BorderColor = border_color.into();
        self
    }

    #[inline]
    pub fn with_lod(mut self, lod: Range<f32>) -> Self {
        self.0.MinLOD = lod.start;
        self.0.MaxLOD = lod.end;
        self
    }
}

/// Describes a shader-resource view (SRV).
///
/// For more information: [`D3D12_SHADER_RESOURCE_VIEW_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_shader_resource_view_desc)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ShaderResourceViewDesc(pub(crate) D3D12_SHADER_RESOURCE_VIEW_DESC);

impl ShaderResourceViewDesc {
    #[inline]
    pub fn buffer(
        format: Format,
        elements: Range<u64>,
        structure_byte_stride: u32,
        flags: BufferSrvFlags,
    ) -> Self {
        Self(D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_SRV_DIMENSION_BUFFER,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Buffer: D3D12_BUFFER_SRV {
                    FirstElement: elements.start,
                    NumElements: elements.count() as u32,
                    StructureByteStride: structure_byte_stride,
                    Flags: flags.as_raw(),
                },
            },
            Shader4ComponentMapping: 0x7,
        })
    }

    #[inline]
    pub fn texture_1d(
        format: Format,
        most_detailed_mip: u32,
        mip_levels: u32,
        resource_min_lod_clamp: f32,
    ) -> Self {
        Self(D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_SRV_DIMENSION_TEXTURE1D,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture1D: D3D12_TEX1D_SRV {
                    MostDetailedMip: most_detailed_mip,
                    MipLevels: mip_levels,
                    ResourceMinLODClamp: resource_min_lod_clamp,
                },
            },
            Shader4ComponentMapping: 0x7,
        })
    }

    #[inline]
    pub fn texture_2d(
        format: Format,
        most_detailed_mip: u32,
        mip_levels: u32,
        resource_min_lod_clamp: f32,
        plane_slice: u32,
    ) -> Self {
        Self(D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_SRV_DIMENSION_TEXTURE2D,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_SRV {
                    MostDetailedMip: most_detailed_mip,
                    MipLevels: mip_levels,
                    ResourceMinLODClamp: resource_min_lod_clamp,
                    PlaneSlice: plane_slice,
                },
            },
            Shader4ComponentMapping: 0x7,
        })
    }

    #[inline]
    pub fn texture_3d(
        format: Format,
        most_detailed_mip: u32,
        mip_levels: u32,
        resource_min_lod_clamp: f32,
    ) -> Self {
        Self(D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_SRV_DIMENSION_TEXTURE3D,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture3D: D3D12_TEX3D_SRV {
                    MostDetailedMip: most_detailed_mip,
                    MipLevels: mip_levels,
                    ResourceMinLODClamp: resource_min_lod_clamp,
                },
            },
            Shader4ComponentMapping: 0x7,
        })
    }

    #[inline]
    pub fn texture_1d_array(
        format: Format,
        most_detailed_mip: u32,
        mip_levels: u32,
        resource_min_lod_clamp: f32,
        array: Range<u32>,
    ) -> Self {
        Self(D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_SRV_DIMENSION_TEXTURE1DARRAY,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture1DArray: D3D12_TEX1D_ARRAY_SRV {
                    MostDetailedMip: most_detailed_mip,
                    MipLevels: mip_levels,
                    ResourceMinLODClamp: resource_min_lod_clamp,
                    FirstArraySlice: array.start,
                    ArraySize: array.count() as u32,
                },
            },
            Shader4ComponentMapping: 0x7,
        })
    }

    #[inline]
    pub fn texture_2d_array(
        format: Format,
        most_detailed_mip: u32,
        mip_levels: u32,
        resource_min_lod_clamp: f32,
        plane_slice: u32,
        array: Range<u32>,
    ) -> Self {
        Self(D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_SRV_DIMENSION_TEXTURE2DARRAY,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2DArray: D3D12_TEX2D_ARRAY_SRV {
                    MostDetailedMip: most_detailed_mip,
                    MipLevels: mip_levels,
                    ResourceMinLODClamp: resource_min_lod_clamp,
                    PlaneSlice: plane_slice,
                    FirstArraySlice: array.start,
                    ArraySize: array.count() as u32,
                },
            },
            Shader4ComponentMapping: 0x7,
        })
    }

    #[inline]
    pub fn texture_2d_ms(format: Format) -> Self {
        Self(D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_SRV_DIMENSION_TEXTURE2DMS,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2DMS: D3D12_TEX2DMS_SRV::default(),
            },
            Shader4ComponentMapping: 0x7,
        })
    }

    #[inline]
    pub fn texture_2d_ms_array(format: Format, array: Range<u32>) -> Self {
        Self(D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2DMSArray: D3D12_TEX2DMS_ARRAY_SRV {
                    FirstArraySlice: array.start,
                    ArraySize: array.count() as u32,
                },
            },
            Shader4ComponentMapping: 0x7,
        })
    }

    #[inline]
    pub fn texture_cube(
        format: Format,
        most_detailed_mip: u32,
        mip_levels: u32,
        resource_min_lod_clamp: f32,
    ) -> Self {
        Self(D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_SRV_DIMENSION_TEXTURECUBE,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                TextureCube: D3D12_TEXCUBE_SRV {
                    MostDetailedMip: most_detailed_mip,
                    MipLevels: mip_levels,
                    ResourceMinLODClamp: resource_min_lod_clamp,
                },
            },
            Shader4ComponentMapping: 0x7,
        })
    }

    #[inline]
    pub fn texture_cube_array(
        format: Format,
        most_detailed_mip: u32,
        mip_levels: u32,
        resource_min_lod_clamp: f32,
        array: Range<u32>,
    ) -> Self {
        Self(D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_SRV_DIMENSION_TEXTURECUBEARRAY,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                TextureCubeArray: D3D12_TEXCUBE_ARRAY_SRV {
                    MostDetailedMip: most_detailed_mip,
                    MipLevels: mip_levels,
                    ResourceMinLODClamp: resource_min_lod_clamp,
                    First2DArrayFace: array.start,
                    NumCubes: array.count() as u32,
                },
            },
            Shader4ComponentMapping: 0x7,
        })
    }

    #[inline]
    pub fn raytracing_acceleration_structure(format: Format, location: GpuVirtualAddress) -> Self {
        Self(D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                RaytracingAccelerationStructure: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
                    Location: location,
                },
            },
            Shader4ComponentMapping: 0x7,
        })
    }
}

/// A handle to the object of event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
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
#[repr(transparent)]
pub struct StaticSamplerDesc(pub(crate) D3D12_STATIC_SAMPLER_DESC);

impl StaticSamplerDesc {
    #[inline]
    pub fn point() -> Self {
        Self::default().with_filter(Filter::Point)
    }

    #[inline]
    pub fn linear() -> Self {
        Self::default().with_filter(Filter::Linear)
    }

    #[inline]
    pub fn anisotropic() -> Self {
        Self::default().with_filter(Filter::Anisotropic)
    }

    #[inline]
    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.0.Filter = filter.as_raw();
        self
    }

    #[inline]
    pub fn with_address_u(mut self, address: AddressMode) -> Self {
        self.0.AddressU = address.as_raw();
        self
    }

    #[inline]
    pub fn with_address_v(mut self, address: AddressMode) -> Self {
        self.0.AddressV = address.as_raw();
        self
    }

    #[inline]
    pub fn with_address_w(mut self, address: AddressMode) -> Self {
        self.0.AddressW = address.as_raw();
        self
    }

    #[inline]
    pub fn with_mip_lod_bias(mut self, mip_lod_bias: f32) -> Self {
        self.0.MipLODBias = mip_lod_bias;
        self
    }

    #[inline]
    pub fn with_max_anisotropy(mut self, max_anisotropy: u32) -> Self {
        self.0.MaxAnisotropy = max_anisotropy;
        self
    }

    #[inline]
    pub fn with_comparison_func(mut self, comparison_func: ComparisonFunc) -> Self {
        self.0.ComparisonFunc = comparison_func.as_raw();
        self
    }

    #[inline]
    pub fn with_border_color(mut self, border_color: BorderColor) -> Self {
        self.0.BorderColor = border_color.as_raw();
        self
    }

    #[inline]
    pub fn with_lod(mut self, lod: Range<f32>) -> Self {
        self.0.MinLOD = lod.start;
        self.0.MaxLOD = lod.end;
        self
    }

    #[inline]
    pub fn with_shader_register(mut self, shader_register: u32) -> Self {
        self.0.ShaderRegister = shader_register;
        self
    }

    #[inline]
    pub fn with_register_space(mut self, register_space: u32) -> Self {
        self.0.RegisterSpace = register_space;
        self
    }

    #[inline]
    pub fn with_visibility(mut self, visibility: ShaderVisibility) -> Self {
        self.0.ShaderVisibility = visibility.as_raw();
        self
    }
}

/// Describes a stream output buffer.
///
/// For more information: [`D3D12_STREAM_OUTPUT_BUFFER_VIEW structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_stream_output_buffer_view)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct StreamOutputBufferView(pub(crate) D3D12_STREAM_OUTPUT_BUFFER_VIEW);

impl StreamOutputBufferView {
    #[inline]
    pub fn new(
        buffer_location: GpuVirtualAddress,
        size: usize,
        buffer_filled_size_location: u64,
    ) -> Self {
        Self(D3D12_STREAM_OUTPUT_BUFFER_VIEW {
            BufferLocation: buffer_location,
            SizeInBytes: size as u64,
            BufferFilledSizeLocation: buffer_filled_size_location,
        })
    }
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct SubresourceFootprint(pub(crate) D3D12_SUBRESOURCE_FOOTPRINT);

impl SubresourceFootprint {
    #[inline]
    pub fn with_format(mut self, format: Format) -> Self {
        self.0.Format = format.as_raw();
        self
    }

    #[inline]
    pub fn with_width(mut self, width: u32) -> Self {
        self.0.Width = width;
        self
    }

    #[inline]
    pub fn with_height(mut self, height: u32) -> Self {
        self.0.Height = height;
        self
    }

    #[inline]
    pub fn with_depth(mut self, depth: u32) -> Self {
        self.0.Depth = depth;
        self
    }

    #[inline]
    pub fn with_row_pitch(mut self, row_pitch: u32) -> Self {
        self.0.RowPitch = row_pitch;
        self
    }

    #[inline]
    pub fn format(&self) -> Format {
        self.0.Format.into()
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.0.Width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.0.Height
    }

    #[inline]
    pub fn depth(&self) -> u32 {
        self.0.Depth
    }

    #[inline]
    pub fn row_pitch(&self) -> u32 {
        self.0.RowPitch
    }
}

/// Describes a tiled subresource volume.
///
/// For more information: [`D3D12_SUBRESOURCE_TILING structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_subresource_tiling)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct SubresourceTiling(pub(crate) D3D12_SUBRESOURCE_TILING);

impl SubresourceTiling {
    #[inline]
    pub fn width(&self) -> u32 {
        self.0.WidthInTiles
    }

    #[inline]
    pub fn height(&self) -> u16 {
        self.0.HeightInTiles
    }

    #[inline]
    pub fn depth(&self) -> u16 {
        self.0.DepthInTiles
    }

    #[inline]
    pub fn start_tile_index_in_overall_resource(&self) -> u32 {
        self.0.StartTileIndexInOverallResource
    }
}

/// Describes a swap chain.
///
/// For more information: [`DXGI_SWAP_CHAIN_DESC1 structure`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/ns-dxgi1_2-dxgi_swap_chain_desc1)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct SwapchainDesc1(pub(crate) DXGI_SWAP_CHAIN_DESC1);

impl SwapchainDesc1 {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self(DXGI_SWAP_CHAIN_DESC1 {
            Width: width,
            Height: height,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 1,
            AlphaMode: DXGI_ALPHA_MODE_UNSPECIFIED,
            ..Default::default()
        })
    }

    #[inline]
    pub fn with_format(mut self, format: Format) -> Self {
        self.0.Format = format.as_raw();
        self
    }

    #[inline]
    pub fn enable_stereo(mut self) -> Self {
        self.0.Stereo = true.into();
        self
    }

    #[inline]
    pub fn with_sample_desc(mut self, sample_desc: SampleDesc) -> Self {
        self.0.SampleDesc = sample_desc.0;
        self
    }

    #[inline]
    pub fn with_usage(mut self, usage: FrameBufferUsage) -> Self {
        self.0.BufferUsage = usage.as_raw();
        self
    }

    #[inline]
    pub fn with_buffer_count(mut self, buffer_count: u32) -> Self {
        self.0.BufferCount = buffer_count;
        self
    }

    #[inline]
    pub fn with_scaling(mut self, scaling: Scaling) -> Self {
        self.0.Scaling = scaling.as_raw();
        self
    }

    #[inline]
    pub fn with_swap_effect(mut self, swap_effect: SwapEffect) -> Self {
        self.0.SwapEffect = swap_effect.as_raw();
        self
    }

    #[inline]
    pub fn with_alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.0.AlphaMode = alpha_mode.as_raw();
        self
    }

    #[inline]
    pub fn with_flags(mut self, flags: SwapchainFlags) -> Self {
        self.0.Flags = flags.bits() as u32;
        self
    }
}

/// Describes a swap chain.
///
/// For more information: [`DXGI_SWAP_CHAIN_FULLSCREEN_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/ns-dxgi1_2-dxgi_swap_chain_fullscreen_desc)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct SwapchainFullscreenDesc(pub(crate) DXGI_SWAP_CHAIN_FULLSCREEN_DESC);

impl SwapchainFullscreenDesc {
    #[inline]
    pub fn with_refresh_rate(mut self, refresh_rate: Rational) -> Self {
        self.0.RefreshRate = refresh_rate.0;
        self
    }

    #[inline]
    pub fn with_scanline_ordering(mut self, scanline_ordering: ScanlineOrdering) -> Self {
        self.0.ScanlineOrdering = scanline_ordering.as_raw();
        self
    }

    #[inline]
    pub fn with_scanline(mut self, scaling: ScalingMode) -> Self {
        self.0.Scaling = scaling.as_raw();
        self
    }

    #[inline]
    pub fn windowed(mut self) -> Self {
        self.0.Windowed = true.into();
        self
    }
}

/// Describes a portion of a texture for the purpose of texture copies.
///
/// For more information: [`D3D12_TEXTURE_COPY_LOCATION structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_texture_copy_location)
#[derive(Clone)]
#[repr(transparent)]
pub struct TextureCopyLocation<'a>(pub(crate) D3D12_TEXTURE_COPY_LOCATION, PhantomData<&'a ()>);

impl<'a> TextureCopyLocation<'a> {
    #[inline]
    pub fn subresource(resource: &'a Resource, index: u32) -> Self {
        Self(
            D3D12_TEXTURE_COPY_LOCATION {
                pResource: unsafe { std::mem::transmute_copy(resource.as_raw()) },
                Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                    SubresourceIndex: index,
                },
            },
            Default::default(),
        )
    }

    #[inline]
    pub fn placed_footprint(resource: &'a Resource, footprint: PlacedSubresourceFootprint) -> Self {
        Self(
            D3D12_TEXTURE_COPY_LOCATION {
                pResource: unsafe { std::mem::transmute_copy(resource.as_raw()) },
                Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                    PlacedFootprint: footprint.0,
                },
            },
            Default::default(),
        )
    }
}

/// Describes the size of a tiled region.
///
/// For more information: [`D3D12_TILE_REGION_SIZE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_tile_region_size)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct TileRegionSize(pub(crate) D3D12_TILE_REGION_SIZE);

impl TileRegionSize {
    #[inline]
    pub fn with_tiles(mut self, num_tiles: u32) -> Self {
        self.0.NumTiles = num_tiles;
        self
    }

    #[inline]
    pub fn with_width(mut self, width: u32) -> Self {
        self.0.Width = width;
        self
    }

    #[inline]
    pub fn with_height(mut self, height: u16) -> Self {
        self.0.Height = height;
        self
    }

    #[inline]
    pub fn with_depth(mut self, depth: u16) -> Self {
        self.0.Depth = depth;
        self
    }

    #[inline]
    pub fn use_box(mut self) -> Self {
        self.0.UseBox = true.into();
        self
    }
}

/// Describes the shape of a tile by specifying its dimensions.
///
/// For more information: [`D3D12_TILE_SHAPE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_tile_shape)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct TileShape(pub(crate) D3D12_TILE_SHAPE);

impl TileShape {
    #[inline]
    pub fn width(&self) -> u32 {
        self.0.WidthInTexels
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.0.HeightInTexels
    }

    #[inline]
    pub fn depth(&self) -> u32 {
        self.0.DepthInTexels
    }
}

/// Describes the coordinates of a tiled resource.
///
/// For more information: [`D3D12_TILED_RESOURCE_COORDINATE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_tiled_resource_coordinate)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct TiledResourceCoordinate(pub(crate) D3D12_TILED_RESOURCE_COORDINATE);

impl TiledResourceCoordinate {
    #[inline]
    pub fn new(x: u32, y: u32, z: u32, subresource: u32) -> Self {
        Self(D3D12_TILED_RESOURCE_COORDINATE {
            X: x,
            Y: y,
            Z: z,
            Subresource: subresource,
        })
    }
}

/// Describes the subresources from a resource that are accessible by using an unordered-access view.
///
/// For more information: [`D3D12_UNORDERED_ACCESS_VIEW_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_unordered_access_view_desc)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct UnorderedAccessViewDesc(pub(crate) D3D12_UNORDERED_ACCESS_VIEW_DESC);
impl UnorderedAccessViewDesc {
    #[inline]
    pub fn buffer(
        format: Format,
        elements: Range<u64>,
        structure_byte_stride: u32,
        counter_offset: u64,
        flags: BufferUavFlags,
    ) -> Self {
        Self(D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_UAV_DIMENSION_BUFFER,
            Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Buffer: D3D12_BUFFER_UAV {
                    FirstElement: elements.start,
                    NumElements: elements.count() as u32,
                    StructureByteStride: structure_byte_stride,
                    CounterOffsetInBytes: counter_offset,
                    Flags: flags.as_raw(),
                },
            },
        })
    }

    #[inline]
    pub fn texture_1d(format: Format, mip_slice: u32) -> Self {
        Self(D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_UAV_DIMENSION_TEXTURE1D,
            Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture1D: D3D12_TEX1D_UAV {
                    MipSlice: mip_slice,
                },
            },
        })
    }

    #[inline]
    pub fn texture_2d(format: Format, mip_slice: u32, plane_slice: u32) -> Self {
        Self(D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_UAV_DIMENSION_TEXTURE2D,
            Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_UAV {
                    MipSlice: mip_slice,
                    PlaneSlice: plane_slice,
                },
            },
        })
    }

    #[inline]
    pub fn texture_3d(format: Format, mip_slice: u32, slices: Range<u32>) -> Self {
        Self(D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_UAV_DIMENSION_TEXTURE3D,
            Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture3D: D3D12_TEX3D_UAV {
                    MipSlice: mip_slice,
                    FirstWSlice: slices.start,
                    WSize: slices.count() as u32,
                },
            },
        })
    }

    #[inline]
    pub fn texture_1d_array(format: Format, mip_slice: u32, array: Range<u32>) -> Self {
        Self(D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_UAV_DIMENSION_TEXTURE1DARRAY,
            Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture1DArray: D3D12_TEX1D_ARRAY_UAV {
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
        Self(D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_UAV_DIMENSION_TEXTURE2DARRAY,
            Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture2DArray: D3D12_TEX2D_ARRAY_UAV {
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
        Self(D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_UAV_DIMENSION_TEXTURE2DMS,
            Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture2DMS: D3D12_TEX2DMS_UAV::default(),
            },
        })
    }

    #[inline]
    pub fn texture_2d_ms_array(format: Format, array: Range<u32>) -> Self {
        Self(D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: format.as_raw(),
            ViewDimension: D3D12_UAV_DIMENSION_TEXTURE2DMSARRAY,
            Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                Texture2DMSArray: D3D12_TEX2DMS_ARRAY_UAV {
                    FirstArraySlice: array.start,
                    ArraySize: array.count() as u32,
                },
            },
        })
    }
}

/// Describes a vertex buffer view.
///
/// For more information: [`D3D12_VERTEX_BUFFER_VIEW structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_vertex_buffer_view)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct VertexBufferView(pub(crate) D3D12_VERTEX_BUFFER_VIEW);

impl VertexBufferView {
    #[inline]
    pub fn new(buffer_location: GpuVirtualAddress, stride: u32, size: u32) -> Self {
        Self(D3D12_VERTEX_BUFFER_VIEW {
            BufferLocation: buffer_location,
            StrideInBytes: stride,
            SizeInBytes: size,
        })
    }
}

/// Describes the dimensions of a viewport.
///
/// For more information: [`D3D12_VIEWPORT structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_viewport)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Viewport(pub(crate) D3D12_VIEWPORT);

impl Viewport {
    /// Creates a viewport with a minimum depth of 0 and a maximum depth of 1.
    #[inline]
    pub fn from_position_and_size(
        position: impl Into<(f32, f32)>,
        size: impl Into<(f32, f32)>,
    ) -> Self {
        let (width, height) = size.into();
        let (x, y) = position.into();

        Self(D3D12_VIEWPORT {
            TopLeftX: x,
            TopLeftY: y,
            Width: width,
            Height: height,
            MinDepth: MIN_DEPTH,
            MaxDepth: MAX_DEPTH,
        })
    }

    /// Creates a viewport with a minimum depth of 0 and a maximum depth of 1 and with position in (0, 0).
    #[inline]
    pub fn from_size(size: impl Into<(f32, f32)>) -> Self {
        Self::from_position_and_size((0.0, 0.0), size)
    }
}
