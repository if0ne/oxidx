use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::{
        ID3D12PipelineState, ID3D12RootSignature, D3D12_COMPARISON_FUNC_ALWAYS,
        D3D12_COMPARISON_FUNC_EQUAL, D3D12_COMPARISON_FUNC_GREATER,
        D3D12_COMPARISON_FUNC_GREATER_EQUAL, D3D12_COMPARISON_FUNC_LESS,
        D3D12_COMPARISON_FUNC_LESS_EQUAL, D3D12_COMPARISON_FUNC_NEVER, D3D12_COMPARISON_FUNC_NONE,
        D3D12_COMPARISON_FUNC_NOT_EQUAL, D3D12_DESCRIPTOR_RANGE_TYPE_CBV,
        D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER, D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
        D3D12_DESCRIPTOR_RANGE_TYPE_UAV, D3D12_FILTER_MIN_MAG_POINT_MIP_LINEAR,
        D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
        D3D12_ROOT_SIGNATURE_FLAG_ALLOW_STREAM_OUTPUT,
        D3D12_ROOT_SIGNATURE_FLAG_CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED,
        D3D12_ROOT_SIGNATURE_FLAG_DENY_AMPLIFICATION_SHADER_ROOT_ACCESS,
        D3D12_ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS,
        D3D12_ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS,
        D3D12_ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS,
        D3D12_ROOT_SIGNATURE_FLAG_DENY_MESH_SHADER_ROOT_ACCESS,
        D3D12_ROOT_SIGNATURE_FLAG_DENY_PIXEL_SHADER_ROOT_ACCESS,
        D3D12_ROOT_SIGNATURE_FLAG_DENY_VERTEX_SHADER_ROOT_ACCESS,
        D3D12_ROOT_SIGNATURE_FLAG_LOCAL_ROOT_SIGNATURE, D3D12_ROOT_SIGNATURE_FLAG_NONE,
        D3D12_ROOT_SIGNATURE_FLAG_SAMPLER_HEAP_DIRECTLY_INDEXED, D3D12_SHADER_VISIBILITY_ALL,
        D3D12_SHADER_VISIBILITY_AMPLIFICATION, D3D12_SHADER_VISIBILITY_DOMAIN,
        D3D12_SHADER_VISIBILITY_GEOMETRY, D3D12_SHADER_VISIBILITY_HULL,
        D3D12_SHADER_VISIBILITY_MESH, D3D12_SHADER_VISIBILITY_PIXEL,
        D3D12_SHADER_VISIBILITY_VERTEX, D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK,
        D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK_UINT, D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE,
        D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE_UINT, D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK,
        D3D12_TEXTURE_ADDRESS_MODE_BORDER, D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
        D3D12_TEXTURE_ADDRESS_MODE_MIRROR, D3D12_TEXTURE_ADDRESS_MODE_MIRROR_ONCE,
        D3D12_TEXTURE_ADDRESS_MODE_WRAP, D3D_ROOT_SIGNATURE_VERSION_1_0,
        D3D_ROOT_SIGNATURE_VERSION_1_1, D3D_ROOT_SIGNATURE_VERSION_1_2,
    },
};

use crate::{create_type, impl_trait, HasInterface};

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
    //fn serialize(desc: &RootSignatureDesc<'_>, );
}

create_type! { RootSignature wrap ID3D12RootSignature }

impl_trait! {
    impl RootSignatureInterface =>
    RootSignature;
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
        const None = D3D12_ROOT_SIGNATURE_FLAG_NONE.0;
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

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum RootSignatureVersion {
    V1_0 = D3D_ROOT_SIGNATURE_VERSION_1_0.0,
    V1_1 = D3D_ROOT_SIGNATURE_VERSION_1_1.0,
    V1_2 = D3D_ROOT_SIGNATURE_VERSION_1_2.0,
}
