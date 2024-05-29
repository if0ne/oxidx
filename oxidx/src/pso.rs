use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::ID3D12PipelineState,
};

use crate::HasInterface;

pub trait PipelineInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12PipelineState>>
{
}
