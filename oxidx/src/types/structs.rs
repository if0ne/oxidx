use std::ffi::CStr;

use windows::Win32::Foundation::HANDLE;

use crate::{blob::Blob, root_signature::RootSignature};

use super::*;

/// Describes the blend state.
///
/// For more information: [`D3D12_BLEND_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_blend_desc)
#[derive(Clone, Debug, Default)]
pub struct BlendDesc {
    /// Specifies whether to use alpha-to-coverage as a multisampling technique when setting a pixel to a render target.
    pub alpha_to_coverage_enable: bool,

    /// Specifies whether to enable independent blending in simultaneous render targets.
    /// Set to TRUE to enable independent blending. If set to FALSE, only the RenderTarget[0] members are used; RenderTarget[1..7] are ignored.
    pub independent_blend_enable: bool,

    /// An array of [`RenderTargetBlendDesc`] structures that describe the blend states for render targets;
    /// these correspond to the eight render targets that can be bound to the output-merger stage at one time.
    pub render_targets: [RenderTargetBlendDesc; 8],
}

/// Describes a command queue.
///
/// For more information: [`D3D12_COMMAND_QUEUE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_command_queue_desc)
#[derive(Clone, Copy, Debug, Default)]
pub struct CommandQueueDesc {
    /// Specifies one member of [`CommandListType`].
    pub r#type: CommandListType,

    /// The priority for the command queue, as a [`CommandQueuePriority`] enumeration constant to select normal or high priority.
    pub priority: CommandQueuePriority,

    /// Specifies any flags from the [`CommandQueueFlags`] enumeration.
    pub flags: CommandQueueFlags,

    /// For single GPU operation, set this to zero. If there are multiple GPU nodes, set a bit to identify the node (the device's physical adapter) to which the command queue applies. Each bit in the mask corresponds to a single node. Only 1 bit must be set.
    pub node_mask: u32,
}

/// Describes the arguments (parameters) of a command signature.
///
/// For more information: [`D3D12_COMMAND_SIGNATURE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_command_signature_desc)
#[derive(Debug)]
pub struct CommandSignatureDesc<'a> {
    /// Specifies the size of each command in the drawing buffer, in bytes.
    pub byte_stride: u32,

    /// An array of [`IndirectArgumentDesc`] enumeration, containing details of the arguments, including whether the argument is a vertex buffer, constant, constant buffer view, shader resource view, or unordered access view.
    pub argument_descs: &'a [IndirectArgumentDesc],

    /// For single GPU operation, set this to zero.
    /// If there are multiple GPU nodes, set bits to identify the nodes (the device's physical adapters) for which the command signature is to apply.
    /// Each bit in the mask corresponds to a single node.
    pub node_mask: u32,
}

/// Describes a compute pipeline state object.
///
/// For more information: [`D3D12_COMPUTE_PIPELINE_STATE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_compute_pipeline_state_desc)
#[derive(Debug)]
pub struct ComputePipelineStateDesc<'a> {
    /// A reference to the [`RootSignatureInterface`] object.
    pub root_signature: &'a RootSignature,

    /// /// A reference to the [`BlobInterface`] object that contains compute shader.
    pub cs: &'a Blob,

    /// For single GPU operation, set this to zero.
    /// If there are multiple GPU nodes, set bits to identify the nodes (the device's physical adapters) for which the compute pipeline state is to apply.
    /// Each bit in the mask corresponds to a single node.
    pub node_mask: u32,

    /// A cached pipeline state object, as a [`CachedPipelineState`] structure. `cached_blob` and `cached_blob_size_in_bytes` may be set to None and 0 respectively.
    pub cached_pso: Option<&'a Blob>,

    /// A [`PipelineStateFlags`] enumeration constant such as for "tool debug".
    pub flags: PipelineStateFlags,
}

/// Describes a constant buffer to view.
///
/// For more information: [`D3D12_CONSTANT_BUFFER_VIEW_DESC structure `](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_constant_buffer_view_desc)
#[derive(Clone, Copy, Debug)]
pub struct ConstantBufferViewDesc {
    // GPU virtual address
    pub buffer_location: u64,

    /// The size in bytes of the constant buffer.
    pub size_in_bytes: u32,
}

/// Describes a CPU descriptor handle.
///
/// For more information: [`D3D12_CPU_DESCRIPTOR_HANDLE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_cpu_descriptor_handle)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CpuDescriptorHandle(pub(crate) usize);

impl CpuDescriptorHandle {
    pub fn offset(&self, offset: usize) -> Self {
        Self(self.0 + offset)
    }
}

/// Describes a vertex element in a vertex buffer in an output slot.
///
/// For more information: [`D3D12_SO_DECLARATION_ENTRY structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_so_declaration_entry)
#[derive(Clone, Debug)]
pub struct DeclarationEntry {
    /// Zero-based, stream number.
    pub stream: u32,

    /// Type of output element; possible values include: "POSITION", "NORMAL", or "TEXCOORD0".
    /// Note that if SemanticName is NULL then ComponentCount can be greater than 4 and the described entry will be a gap in the stream out where no data will be written.
    pub semantic_name: &'static CStr,

    /// Output element's zero-based index. Use, for example, if you have more than one texture coordinate stored in each vertex.
    pub semantic_index: u32,

    /// The component of the entry to begin writing out to. Valid values are 0 to 3. For example, if you only wish to output to the y and z components of a position, StartComponent is 1 and ComponentCount is 2.
    pub start_component: u8,

    /// The number of components of the entry to write out to. Valid values are 1 to 4.
    /// For example, if you only wish to output to the y and z components of a position, StartComponent is 1 and ComponentCount is 2.
    /// Note that if SemanticName is NULL then ComponentCount can be greater than 4 and the described entry will be a gap in the stream out where no data will be written.
    pub component_count: u8,

    /// The associated stream output buffer that is bound to the pipeline. The valid range for OutputSlot is 0 to 3.
    pub output_slot: u8,
}

/// Describes depth-stencil state.
///
/// For more information: [`D3D12_DEPTH_STENCIL_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_depth_stencil_desc)
#[derive(Clone, Copy, Debug, Default)]
pub struct DepthStencilDesc {
    /// Specifies whether to enable depth testing. Set this member to TRUE to enable depth testing.
    pub depth_enable: bool,

    /// A [`DepthWriteMask`]-typed value that identifies a portion of the depth-stencil buffer that can be modified by depth data.
    pub depth_write_mask: DepthWriteMask,

    /// A [`ComparisonFunc`]-typed value that identifies a function that compares depth data against existing depth data.
    pub depth_func: ComparisonFunc,

    /// Specifies whether to enable stencil testing. Set this member to TRUE to enable stencil testing.
    pub stencil_enable: bool,

    /// Identify a portion of the depth-stencil buffer for reading stencil data.
    pub stencil_read_mask: u8,

    /// Identify a portion of the depth-stencil buffer for writing stencil data.
    pub stencil_write_mask: u8,

    /// A [`DepthStencilOpDesc`] structure that describes how to use the results of the depth test and the stencil test for pixels whose surface normal is facing towards the camera.
    pub front_face: DepthStencilOpDesc,

    /// A [`DepthStencilOpDesc`] structure that describes how to use the results of the depth test and the stencil test for pixels whose surface normal is facing away from the camera.
    pub back_face: DepthStencilOpDesc,
}

/// Describes stencil operations that can be performed based on the results of stencil test.
///
/// For more information: [`D3D12_DEPTH_STENCILOP_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_depth_stencilop_desc)
#[derive(Clone, Copy, Debug, Default)]
pub struct DepthStencilOpDesc {
    /// A [`StencilOp`]-typed value that identifies the stencil operation to perform when stencil testing fails.
    pub stencil_fail_op: StencilOp,

    /// A [`StencilOp`]-typed value that identifies the stencil operation to perform when stencil testing passes and depth testing fails.
    pub stencil_depth_fail_op: StencilOp,

    /// A [`StencilOp`]-typed value that identifies the stencil operation to perform when stencil testing and depth testing both pass.
    pub stencil_pass_op: StencilOp,

    /// A [`ComparisonFunc`]-typed value that identifies the function that compares stencil data against existing stencil data.
    pub stencil_func: ComparisonFunc,
}

/// Describes the subresources of a texture that are accessible from a depth-stencil view.
///
/// For more information: [`D3D12_DEPTH_STENCIL_VIEW_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_depth_stencil_view_desc)
#[derive(Clone, Copy, Debug)]
pub struct DepthStencilViewDesc {
    /// A [`Format`]-typed value that specifies the viewing format.
    pub format: Format,

    /// A [`DsvDimension`]-typed value that specifies how the depth-stencil resource will be accessed. This member also determines which _DSV to use in the following union.
    pub view_dimension: DsvDimension,

    /// A combination of [`DsvFlags`] enumeration constants that are combined by using a bitwise OR operation. The resulting value specifies whether the texture is read only.
    ///
    /// Pass `empty` to specify that it isn't read only; otherwise, pass one or more of the members of the [`DsvFlags`] enumerated type.
    pub flags: DsvFlags,
}

/// Describes the descriptor heap.
///
/// For more information: [`D3D12_DESCRIPTOR_HEAP_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_descriptor_heap_desc)
#[derive(Clone, Copy, Debug, Default)]
pub struct DescriptorHeapDesc {
    /// A [`DescriptorHeapType`]-typed value that specifies the types of descriptors in the heap.
    pub r#type: DescriptorHeapType,

    /// The number of descriptors in the heap.
    pub num: u32,

    /// A combination of [`DescriptorHeapFlags]-typed values that are combined by using a bitwise OR operation. The resulting value specifies options for the heap.
    pub flags: DescriptorHeapFlags,

    /// For single-adapter operation, set this to zero. If there are multiple adapter nodes, set a bit to identify the node (one of the device's physical adapters) to which the descriptor heap applies. Each bit in the mask corresponds to a single node. Only one bit must be set.
    pub node_mask: u32,
}

/// Describes a descriptor range.
///
/// For more information: [`D3D12_DESCRIPTOR_RANGE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_descriptor_range)
#[derive(Clone, Debug)]
pub struct DescriptorRange {
    /// A [`DescriptorRangeType`]-typed value that specifies the type of descriptor range.
    pub r#type: DescriptorRangeType,

    /// The number of descriptors in the range.
    pub num: u32,

    /// The base shader register in the range.
    pub base_shader_register: u32,

    /// The register space. Can typically be 0, but allows multiple descriptor arrays of unknown size to not appear to overlap.
    pub register_space: u32,

    /// The offset in descriptors, from the start of the descriptor table which was set as the root argument value for this parameter slot.
    pub offset_in_descriptors_from_table_start: u32,
}

/// Describes a GPU descriptor handle.
///
/// For more information: [`D3D12_GPU_DESCRIPTOR_HANDLE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_gpu_descriptor_handle)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuDescriptorHandle(pub(crate) usize);

impl GpuDescriptorHandle {
    pub fn offset(&self, offset: usize) -> Self {
        Self(self.0 + offset)
    }
}

/// Describes a graphics pipeline state object.
///
/// For more information: [`D3D12_GRAPHICS_PIPELINE_STATE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_graphics_pipeline_state_desc)
#[derive(Debug)]
pub struct GraphicsPipelineDesc<'a> {
    /// A reference to the [`RootSignature`] object.
    pub root_signature: &'a RootSignature,

    /// A [`Blob`] that contains the vertex shader.
    pub vs: &'a Blob,

    /// A [`Blob`] that contains the pixel shader.
    pub ps: Option<&'a Blob>,

    /// A [`Blob`] that contains the domain shader.
    pub ds: Option<&'a Blob>,

    /// A [`Blob`] that contains the hull shader.
    pub hs: Option<&'a Blob>,

    /// A [`Blob`] that contains the geometry shader.
    pub gs: Option<&'a Blob>,

    /// A [`StreamOutputDesc`] structure that describes a streaming output buffer.
    pub stream_output: Option<StreamOutputDesc<'a>>,

    /// A [`BlendDesc`] structure that describes the blend state.
    pub blend_state: BlendDesc,

    /// The sample mask for the blend state.
    pub sample_mask: u32,

    /// A [`RasterizerDesc`] structure that describes the rasterizer state.
    pub rasterizer_state: RasterizerDesc,

    /// A [`DepthStencilDesc`] structure that describes the depth-stencil state.
    pub depth_stencil: Option<DepthStencilDesc>,

    /// An array of [`InputElementDesc`] that describes the input-buffer data for the input-assembler stage.
    pub input_layout: &'a [InputElementDesc],

    /// Specifies the properties of the index buffer in a [`IndexBufferStripCutValue`] structure.
    pub ib_strip_cut_value: Option<IndexBufferStripCutValue>,

    /// A [`PrimitiveTopology`]-typed value for the type of primitive, and ordering of the primitive data.
    pub primitive_topology: PipelinePrimitiveTopology,

    /// The number of render target formats in the rtv_formats member.
    pub num_render_targets: u32,

    /// An array of [`Format`]-typed values for the render target formats.
    pub rtv_formats: [Format; 8],

    /// A [`Format`]-typed value for the depth-stencil format.
    pub dsv_format: Option<Format>,

    /// A [`SampleDesc`] structure that specifies multisampling parameters.
    pub sampler_desc: SampleDesc,

    /// For single GPU operation, set this to zero.
    /// If there are multiple GPU nodes, set bits to identify the nodes (the device's physical adapters) for which the graphics pipeline state is to apply.
    /// Each bit in the mask corresponds to a single node.
    pub node_mask: u32,

    /// A cached pipeline state object, as a [`Blob`].
    pub cached_pso: Option<&'a Blob>,

    /// A [`PipelineStateFlags`] enumeration constant such as for "tool debug".
    pub flags: PipelineStateFlags,
}

/// Describes a single element for the input-assembler stage of the graphics pipeline.
///
/// For more information: [`D3D12_INPUT_ELEMENT_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_input_element_desc)
#[derive(Clone, Debug)]
pub struct InputElementDesc {
    /// The HLSL semantic associated with this element in a shader input-signature.
    pub semantic_name: &'static CStr,

    /// The semantic index for the element.
    /// A semantic index modifies a semantic, with an integer index number.
    ///  A semantic index is only needed in a case where there is more than one element with the same semantic.
    /// For example, a 4x4 matrix would have four components each with the semantic name matrix, however each of the four component would have different semantic indices (0, 1, 2, and 3).
    pub semantic_index: u32,

    /// A [`Format`]-typed value that specifies the format of the element data.
    pub format: Format,

    /// An integer value that identifies the input-assembler. Valid values are between 0 and 15.
    pub input_slot: u32,

    /// Optional. Offset, in bytes, to this element from the start of the vertex.
    /// Use `0xffffffff` for convenience to define the current element directly after the previous one, including any packing if necessary.
    pub offset: u32,

    /// A value that identifies the input data class for a single input slot.
    pub slot_class: InputSlotClass,

    /// The number of instances to draw using the same per-instance data before advancing in the buffer by one element.
    pub instance_data_step_rate: u32,
}

/// Describes a heap.
///
/// For more information: [`D3D12_HEAP_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_heap_desc)
#[derive(Clone, Copy, Debug)]
pub struct HeapDesc {
    /// The size, in bytes, of the heap. To avoid wasting memory, applications should pass size values which are multiples of the effective Alignment;
    /// but non-aligned size is also supported, for convenience.
    /// To find out how large a heap must be to support textures with undefined layouts and adapter-specific sizes, call [`get_resource_allocation_info`](crate::device::DeviceInterface::get_resource_allocation_info)
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
#[derive(Clone, Copy, Debug)]
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

/// The LUID structure is an opaque structure that specifies an identifier that is guaranteed to be unique on the local machine.
///
/// For more information: [`LUID structure`](https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-luid)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Luid {
    /// TBD
    pub low_part: u32,

    /// TBD
    pub high_part: i32,
}

/// Describes the purpose of a query heap. A query heap contains an array of individual queries.
///
/// For more information: [`D3D12_QUERY_HEAP_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_query_heap_desc)
#[derive(Clone, Copy, Debug, Default)]
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
#[derive(Clone, Copy, Debug, Default)]
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

/// Describes the subresources from a resource that are accessible by using a render-target view.
///
/// For more information: [`D3D12_RENDER_TARGET_VIEW_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_render_target_view_desc)
#[derive(Clone, Copy, Debug)]
pub struct RenderTargetViewDesc {
    /// A [`Format`]-typed value that specifies the viewing format.
    pub format: Format,

    /// A [`RtvDimension`]-typed value that specifies how the render-target resource will be accessed. This type specifies how the resource will be accessed.
    pub dimension: RtvDimension,
}

/// Describes a resource, such as a texture. This structure is used extensively.
///
/// For more information: [`D3D12_RESOURCE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_resource_desc)
#[derive(Clone, Copy, Debug)]
pub struct ResourceDesc {
    /// One member of [`ResourceDimension`], specifying the dimensions of the resource.
    pub dimension: ResourceDimension,

    /// Specifies the alignment.
    pub alignment: u64,

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

/// Describes the layout of a root signature version 1.0.
///
/// For more information: [`D3D12_ROOT_SIGNATURE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_root_signature_desc)
#[derive(Debug, Default)]
pub struct RootSignatureDesc<'a> {
    /// An array of [`RootParameter`] structures for the slots in the root signature.
    pub parameters: &'a [RootParameter<'a>],

    /// Pointer to one or more [`StaticSamplerDesc`] structures.
    pub samplers: &'a [StaticSamplerDesc],

    /// A combination of [`RootSignatureFlags`]-typed values that are combined by using a bitwise OR operation. The resulting value specifies options for the root signature layout.
    pub flags: RootSignatureFlags,
}

/// Describes the slot of a root signature version 1.0.
///
/// For more information: [`D3D12_ROOT_PARAMETER structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_root_parameter)
#[derive(Clone, Debug)]
pub struct RootParameter<'a> {
    /// A [`RootParameterType`]-typed value that specifies the type of root signature slot. This member determines which type to use in the union below.
    pub r#type: RootParameterType<'a>,

    /// A [`ShaderVisibility`]-typed value that specifies the shaders that can access the contents of the root signature slot.
    pub visibility: ShaderVisibility,
}

/// Describes multi-sampling parameters for a resource.
///
/// For more information: [`DXGI_SAMPLE_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/dxgicommon/ns-dxgicommon-dxgi_sample_desc)
#[derive(Debug, Default, Clone, Copy)]
pub struct SampleDesc {
    /// The number of multisamples per pixel.
    pub count: u32,

    /// The image quality level. The higher the quality, the lower the performance.
    pub quality: u32,
}

/// Describes a sampler state.
///
/// For more information: [`D3D12_SAMPLER_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_sampler_desc)
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct ShaderResourceViewDesc {
    /// A [`Format`]-typed value that specifies the viewing format.
    pub format: Format,

    /// A [`SrvDimension`]-typed value that specifies the resource type of the view. This type is the same as the resource type of the underlying resource.
    pub dimension: SrvDimension,
}

/// A handle to the object of event.
#[derive(Clone, Copy, Debug)]
pub struct SharedHandle(pub(crate) HANDLE);

/// Describes a static sampler.
///
/// For more information: [`D3D12_STATIC_SAMPLER_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_static_sampler_desc)
#[derive(Clone, Debug)]
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

/// Describes a streaming output buffer.
///
/// For more information: [`D3D12_STREAM_OUTPUT_DESC structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_stream_output_desc)
#[derive(Clone, Debug)]
pub struct StreamOutputDesc<'a> {
    /// An array of [`DeclarationEntry`] structures
    pub entries: &'a [DeclarationEntry],

    /// An array of buffer strides; each stride is the size of an element for that buffer.
    pub buffer_strides: &'a [u32],

    /// The index number of the stream to be sent to the rasterizer stage.
    pub rasterized_stream: u32,
}

/// Describes the size of a tiled region.
///
/// For more information: [`D3D12_TILE_REGION_SIZE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_tile_region_size)
#[derive(Clone, Copy, Debug)]
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

/// Describes the coordinates of a tiled resource.
///
/// For more information: [`D3D12_TILED_RESOURCE_COORDINATE structure`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_tiled_resource_coordinate)
#[derive(Clone, Copy, Debug)]
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
#[derive(Debug, Clone)]
pub struct UnorderedAccessViewDesc {
    /// A [`Format`]-typed value that specifies the viewing format.
    pub format: Format,

    /// A [`UavDimension`]-typed value that specifies the resource type of the view. This type specifies how the resource will be accessed.
    pub dimension: UavDimension,
}
