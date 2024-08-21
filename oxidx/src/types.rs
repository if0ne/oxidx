mod enums;
mod flags;
mod structs;

pub mod features;

pub use enums::*;
pub use flags::*;
pub use structs::*;

use windows::Win32::Graphics::{
    Direct3D::Fxc::{D3DCOMPILE_DEBUG, D3DCOMPILE_SKIP_OPTIMIZATION},
    Direct3D12::*,
    Dxgi::{Common::*, *},
};

use crate::dx::{Adapter3, Output1, PipelineState};

pub const MIN_DEPTH: f32 = D3D12_MIN_DEPTH;
pub const MAX_DEPTH: f32 = D3D12_MAX_DEPTH;
pub const BARRIER_ALL_SUBRESOURCES: u32 = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
pub const TEXTURE_DATA_PITCH_ALIGNMENT: u32 = D3D12_TEXTURE_DATA_PITCH_ALIGNMENT;

pub const COMPILE_DEBUG: u32 = D3DCOMPILE_DEBUG;
pub const COMPILE_SKIP_OPT: u32 = D3DCOMPILE_SKIP_OPTIMIZATION;

pub type GpuVirtualAddress = u64;

pub const DESCRIPTOR_RANGE_OFFSET_APPEND: u32 = D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND;

pub const ADAPTER_NONE: Option<&Adapter3> = None;
pub const PSO_NONE: Option<&PipelineState> = None;
pub const OUTPUT_NONE: Option<&Output1> = None;
