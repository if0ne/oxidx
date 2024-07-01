use std::{ffi::CStr, path::Path};

use smallvec::SmallVec;
use windows::{
    core::{Interface, Param, HSTRING, PCSTR},
    Win32::Graphics::{
        Direct3D::{Fxc::D3DCompileFromFile, ID3DBlob},
        Direct3D12::*,
    },
};

use crate::{
    create_type, error::DxError, impl_trait, swapchain::SampleDesc, types::{Format, RootSignatureVersion}, HasInterface,
};

pub trait PipelineStateInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12PipelineState>>
{
}

create_type! { PipelineState wrap ID3D12PipelineState }

impl_trait! {
    impl PipelineStateInterface =>
    PipelineState;
}

pub trait RootSignatureInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12RootSignature>>
{
    fn serialize(
        desc: &RootSignatureDesc<'_>,
        version: RootSignatureVersion,
    ) -> Result<Blob, DxError>;
}

create_type! { RootSignature wrap ID3D12RootSignature }

impl_trait! {
    impl RootSignatureInterface =>
    RootSignature;

    fn serialize(desc: &RootSignatureDesc<'_>, version: RootSignatureVersion) -> Result<Blob, DxError> {
        let mut signature = None;

        let parameters = desc.parameters.iter().map(|param| param.as_raw()).collect::<SmallVec<[_; 16]>>();
        let sampler = desc.samplers.iter().map(|sampler| sampler.as_raw()).collect::<SmallVec<[_; 16]>>();

        let desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: desc.parameters.len() as u32,
            pParameters: parameters.as_ptr(),
            NumStaticSamplers: desc.samplers.len() as u32,
            pStaticSamplers: sampler.as_ptr(),
            Flags: desc.flags.as_raw(),
        };

        let signature = unsafe {
            D3D12SerializeRootSignature(
                &desc,
                version.as_raw(),
                &mut signature,
                None,
            )
        }
        .map(|()| signature.unwrap())
        .map_err(|_| DxError::Dummy)?;

        Ok(Blob::new(signature))
    }
}

pub trait BlobInterface: HasInterface<Raw: Interface> {
    // TODO: type for target
    fn compile_from_file(
        filename: impl AsRef<Path>,
        /*defines, includes,*/
        entry_point: impl AsRef<CStr>,
        target: impl AsRef<CStr>,
        flags1: u32,
        flags2: u32,
    ) -> Result<Self, DxError>
    where
        Self: Sized;

    fn get_buffer_ptr(&self) -> *mut ();
    fn get_buffer_size(&self) -> usize;
}

create_type! { Blob wrap ID3DBlob }

impl_trait! {
    impl BlobInterface =>
    Blob;

    fn compile_from_file(
        filename: impl AsRef<Path>,
        /*defines, includes,*/
        entry_point: impl AsRef<CStr>,
        target: impl AsRef<CStr>,
        flags1: u32,
        flags2: u32,
    ) -> Result<Self, DxError>
    where
        Self: Sized,
    {
        let filename: HSTRING = filename.as_ref().to_str().unwrap_or("").into();
        let entry_point = PCSTR::from_raw(entry_point.as_ref().as_ptr() as *const _);
        let target = PCSTR::from_raw(target.as_ref().as_ptr() as *const _);

        let mut shader = None;
        let mut error = None;

        unsafe {
            D3DCompileFromFile(
                &filename,
                None,
                None,
                entry_point,
                target,
                flags1,
                flags2,
                &mut shader,
                Some(&mut error),
            )
            .map_err(|_| DxError::Dummy)?;
        }

        //TODO: Error message to error
        Ok(Blob::new(shader.unwrap()))
    }


    fn get_buffer_ptr(&self) -> *mut () {
        unsafe {
            self.0.GetBufferPointer() as *mut _
        }
    }

    fn get_buffer_size(&self) -> usize {
        unsafe {
            self.0.GetBufferSize()
        }
    }
}

#[derive(Debug, Default)]
pub struct RootSignatureDesc<'a> {
    pub parameters: &'a [RootParameter<'a>],
    pub samplers: &'a [StaticSamplerDesc],
    pub flags: RootSignatureFlags,
}

#[derive(Clone, Debug)]
pub struct RootParameter<'a> {
    pub r#type: RootParameterType<'a>,
    pub visibility: ShaderVisibility,
}

#[derive(Clone, Debug)]
pub struct StaticSamplerDesc {
    pub filter: Filter,
    pub address_u: AddressMode,
    pub address_v: AddressMode,
    pub address_w: AddressMode,
    pub mip_lod_bias: f32,
    pub max_anisotropy: f32,
    pub comparison_func: ComparisonFunc,
    pub border_color: BorderColor,
    pub min_lod: f32,
    pub max_lod: f32,
    pub shader_register: u32,
    pub register_space: u32,
    pub visibility: ShaderVisibility,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct RootSignatureFlags: i32 {
        const AllowInputAssemblerInputLayout = D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT.0;
        const DenyVertexShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_VERTEX_SHADER_ROOT_ACCESS.0;
        const DenyHullShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS.0;
        const DenyDomainShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS.0;
        const DenyGeometryShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS.0;
        const DenyPixelShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_PIXEL_SHADER_ROOT_ACCESS.0;
        const AllowStreamOutput = D3D12_ROOT_SIGNATURE_FLAG_ALLOW_STREAM_OUTPUT.0;
        const Local = D3D12_ROOT_SIGNATURE_FLAG_LOCAL_ROOT_SIGNATURE.0;
        const DenyAmplificationShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_AMPLIFICATION_SHADER_ROOT_ACCESS.0;
        const DenyMeshShaderAccess = D3D12_ROOT_SIGNATURE_FLAG_DENY_MESH_SHADER_ROOT_ACCESS.0;
        const CbvSrvUavHeapDirectlyIndexed = D3D12_ROOT_SIGNATURE_FLAG_CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED.0;
        const SamplerHeapDirectlyIndexed = D3D12_ROOT_SIGNATURE_FLAG_SAMPLER_HEAP_DIRECTLY_INDEXED.0;
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum ShaderVisibility {
    All = D3D12_SHADER_VISIBILITY_ALL.0,
    Vertex = D3D12_SHADER_VISIBILITY_VERTEX.0,
    Hull = D3D12_SHADER_VISIBILITY_HULL.0,
    Domain = D3D12_SHADER_VISIBILITY_DOMAIN.0,
    Geometry = D3D12_SHADER_VISIBILITY_GEOMETRY.0,
    Pixel = D3D12_SHADER_VISIBILITY_PIXEL.0,
    Amplification = D3D12_SHADER_VISIBILITY_AMPLIFICATION.0,
    Mesh = D3D12_SHADER_VISIBILITY_MESH.0,
}

#[derive(Clone, Debug)]
pub enum RootParameterType<'a> {
    Cbv {
        shader_register: u32,
        register_space: u32,
    },
    Srv {
        shader_register: u32,
        register_space: u32,
    },
    Uav {
        shader_register: u32,
        register_space: u32,
    },
    DescriptorTable {
        ranges: &'a [DescriptorRange],
    },
    Constants {
        shader_register: u32,
        register_space: u32,
        num_32bit_values: u32,
    },
}

// MUST BE repr(C) for casting in raw format
#[derive(Clone, Debug)]
#[repr(C)]
pub struct DescriptorRange {
    r#type: DescriptorRangeType,
    num: u32,
    base_shader_register: u32,
    register_space: u32,
    offset_in_descriptors_from_table_start: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum DescriptorRangeType {
    Srv = D3D12_DESCRIPTOR_RANGE_TYPE_SRV.0,
    Uav = D3D12_DESCRIPTOR_RANGE_TYPE_UAV.0,
    Cbv = D3D12_DESCRIPTOR_RANGE_TYPE_CBV.0,
    Sampler = D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER.0,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Filter {
    MinMagPointMipLinear = D3D12_FILTER_MIN_MAG_POINT_MIP_LINEAR.0,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum AddressMode {
    Wrap = D3D12_TEXTURE_ADDRESS_MODE_WRAP.0,
    Mirror = D3D12_TEXTURE_ADDRESS_MODE_MIRROR.0,
    Clamp = D3D12_TEXTURE_ADDRESS_MODE_CLAMP.0,
    Border = D3D12_TEXTURE_ADDRESS_MODE_BORDER.0,
    MirrorOnce = D3D12_TEXTURE_ADDRESS_MODE_MIRROR_ONCE.0,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum ComparisonFunc {
    None = D3D12_COMPARISON_FUNC_NONE.0,
    Never = D3D12_COMPARISON_FUNC_NEVER.0,
    Less = D3D12_COMPARISON_FUNC_LESS.0,
    Equal = D3D12_COMPARISON_FUNC_EQUAL.0,
    LessEqual = D3D12_COMPARISON_FUNC_LESS_EQUAL.0,
    FuncGreater = D3D12_COMPARISON_FUNC_GREATER.0,
    NotEqual = D3D12_COMPARISON_FUNC_NOT_EQUAL.0,
    GreaterEqual = D3D12_COMPARISON_FUNC_GREATER_EQUAL.0,
    Always = D3D12_COMPARISON_FUNC_ALWAYS.0,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum BorderColor {
    TransparentBlack = D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK.0,
    OpaqueBlack = D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK.0,
    OpaqueWhite = D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE.0,
    OpaqueBlackUint = D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK_UINT.0,
    OpaqueWhiteUint = D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE_UINT.0,
}

#[derive(Clone, Debug)]
pub struct InputElementDesc {
    pub semantic_name: &'static CStr,
    pub semantic_index: u32,
    pub format: Format,
    pub input_slot: u32,
    pub offset: u32,
    pub slot_class: InputSlotClass,
    pub instance_data_step_rate: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum InputSlotClass {
    PerVertex = D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA.0,
}

#[derive(Debug)]
pub struct GraphicsPipelineDesc<'a> {
    pub root_signature: &'a RootSignature,
    pub input_layout: &'a [InputElementDesc],
    pub vs: &'a Blob,
    pub ps: Option<&'a Blob>,
    pub ds: Option<&'a Blob>,
    pub hs: Option<&'a Blob>,
    pub gs: Option<&'a Blob>,
    pub stream_output: Option<StreamOutputDesc<'a>>,
    pub blend_state: BlendDesc,
    pub sample_mask: u32,
    pub rasterizer_state: RasterizerDesc,
    pub depth_stencil: Option<DepthStencilDesc>,
    pub ib_strip_cut_value: Option<IndexBufferStripCutValue>,
    pub primitive_topology: PrimitiveTopology,
    pub rtv_formats: SmallVec<[Format; 8]>, // TODO: Custom Type
    pub dsv_format: Option<Format>,
    pub sampler_desc: SampleDesc,
    pub node_mask: u32,
    pub cached_pso: Option<CachedPipeline>,
    pub flags: PipelineFlags,
}

#[derive(Clone, Debug)]
pub struct StreamOutputDesc<'a> {
    pub entries: &'a [DeclarationEntry],
    pub buffer_strides: &'a [u32],
    pub rasterized_stream: u32,
}

#[derive(Clone, Debug)]
pub struct DeclarationEntry {
    pub stream: u32,
    pub semantic_name: &'static CStr,
    pub semantic_index: u32,
    pub start_component: u8,
    pub component_count: u8,
    pub output_slot: u8,
}

#[derive(Clone, Debug)]
pub struct BlendDesc {
    pub render_targets: SmallVec<[RenderTargetBlendDesc; 8]>,
    pub alpha_to_coverage_enable: bool,
    pub independent_blend_enable: bool,
}

#[derive(Clone, Debug)]
pub struct RenderTargetBlendDesc {
    pub blend_enable: bool,
    pub logic_op_enable: bool,
    pub src_blend: Blend,
    pub dst_blend: Blend,
    pub blend_op: BlendOp,
    pub src_blend_alpha: Blend,
    pub dst_blend_alpha: Blend,
    pub blend_op_alpha: BlendOp,
    pub logic_op: LogicOp,
    pub mask: BlendMask,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Blend {
    One = D3D12_BLEND_ONE.0,
    Zero = D3D12_BLEND_ZERO.0,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum BlendOp {
    Add = D3D12_BLEND_OP_ADD.0,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct BlendMask: u8 {
        const R = 1;
        const G = 2;
        const B = 4;
        const A = 8;
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum LogicOp {
    Noop = D3D12_LOGIC_OP_NOOP.0,
}

#[derive(Clone, Debug)]
pub struct RasterizerDesc {
    pub fill_mode: FillMode,
    pub cull_mode: CullMode,
}

#[derive(Clone, Debug)]
pub enum FillMode {
    Solid,
    Wireframe,
}

#[derive(Clone, Debug)]
pub enum CullMode {
    None,
}

#[derive(Clone, Debug)]
pub struct DepthStencilDesc {}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum IndexBufferStripCutValue {
    Disabled = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED.0,
}

#[derive(Clone, Debug)]
pub enum PrimitiveTopology {
    Triangle,
    Point,
}

#[derive(Clone, Debug)]
pub struct CachedPipeline {}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct PipelineFlags: i32 {

    }
}
