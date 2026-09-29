mod enums;
mod flags;
mod structs;

pub mod features;

pub use enums::*;
pub use flags::*;
pub use structs::*;

use windows::Win32::Graphics::{
    Direct3D12::*,
    Dxgi::{Common::*, *},
};

use windows::Win32::Graphics::Direct3D::Fxc::{
    D3DCOMPILE_DEBUG, D3DCOMPILE_PACK_MATRIX_ROW_MAJOR, D3DCOMPILE_SKIP_OPTIMIZATION,
};

use crate::dx::{Adapter3, Output1, PipelineState, Resource};

pub const MIN_DEPTH: f32 = D3D12_MIN_DEPTH;
pub const MAX_DEPTH: f32 = D3D12_MAX_DEPTH;
pub const BARRIER_ALL_SUBRESOURCES: u32 = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
pub const TEXTURE_DATA_PITCH_ALIGNMENT: u32 = D3D12_TEXTURE_DATA_PITCH_ALIGNMENT;
pub const APPEND_ALIGNED_ELEMENT: u32 = D3D12_APPEND_ALIGNED_ELEMENT;

pub const COMPILE_DEBUG: u32 = D3DCOMPILE_DEBUG;
pub const COMPILE_SKIP_OPT: u32 = D3DCOMPILE_SKIP_OPTIMIZATION;
pub const PACK_MATRIX_ROW_MAJOR: u32 = D3DCOMPILE_PACK_MATRIX_ROW_MAJOR;

pub const DESCRIPTOR_RANGE_OFFSET_APPEND: u32 = D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND;

/// Command line arguments understood by the DXC compiler, as taken by
/// [`DxcCompiler3::compile`](crate::dxc::DxcCompiler3::compile).
#[cfg(feature = "dxc")]
mod dxc_args {
    pub const DXC_ARG_DEBUG: &str = "-Zi";
    pub const DXC_ARG_SKIP_VALIDATION: &str = "-Vd";
    pub const DXC_ARG_SKIP_OPTIMIZATIONS: &str = "-Od";
    pub const DXC_ARG_PACK_MATRIX_ROW_MAJOR: &str = "-Zpr";
    pub const DXC_ARG_PACK_MATRIX_COLUMN_MAJOR: &str = "-Zpc";
    pub const DXC_ARG_AVOID_FLOW_CONTROL: &str = "-Gfa";
    pub const DXC_ARG_PREFER_FLOW_CONTROL: &str = "-Gfp";
    pub const DXC_ARG_ENABLE_STRICTNESS: &str = "-Ges";
    pub const DXC_ARG_ENABLE_BACKWARDS_COMPATIBILITY: &str = "-Gec";
    pub const DXC_ARG_IEEE_STRICTNESS: &str = "-Gis";
    pub const DXC_ARG_OPTIMIZATION_LEVEL0: &str = "-O0";
    pub const DXC_ARG_OPTIMIZATION_LEVEL1: &str = "-O1";
    pub const DXC_ARG_OPTIMIZATION_LEVEL2: &str = "-O2";
    pub const DXC_ARG_OPTIMIZATION_LEVEL3: &str = "-O3";
    pub const DXC_ARG_WARNINGS_ARE_ERRORS: &str = "-WX";
    pub const DXC_ARG_RESOURCES_MAY_ALIAS: &str = "-res_may_alias";
    pub const DXC_ARG_ALL_RESOURCES_BOUND: &str = "-all_resources_bound";
    pub const DXC_ARG_DEBUG_NAME_FOR_SOURCE: &str = "-Zss";
    pub const DXC_ARG_DEBUG_NAME_FOR_BINARY: &str = "-Zsb";
}

#[cfg(feature = "dxc")]
pub use dxc_args::*;

pub const ADAPTER_NONE: Option<&Adapter3> = None;
pub const PSO_NONE: Option<&PipelineState> = None;
pub const OUTPUT_NONE: Option<&Output1> = None;
pub const RES_NONE: Option<&Resource> = None;

pub type GpuVirtualAddress = u64;

#[cfg(feature = "callback")]
pub type CallbackData =
    std::boxed::Box<dyn Fn(MessageCategory, MessageSeverity, MessageId, &'_ str) + Send + Sync>;
