use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{create_type, impl_trait, HasInterface};

/// Represents the state of all currently set shaders as well as certain fixed function state objects.
///
/// For more information: [`ID3D12PipelineState interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12pipelinestate)
pub trait IPipelineState:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12PipelineState>>
{
}

create_type! {
    /// Represents the state of all currently set shaders as well as certain fixed function state objects.
    ///
    /// For more information: [`ID3D12PipelineState interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12pipelinestate)
    PipelineState wrap ID3D12PipelineState
}

impl_trait! {
    impl IPipelineState =>
    PipelineState;
}
