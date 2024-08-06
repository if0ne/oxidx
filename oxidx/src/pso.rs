use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{blob::Blob, create_type, error::DxError, impl_trait, HasInterface};

/// Represents the state of all currently set shaders as well as certain fixed function state objects.
///
/// For more information: [`ID3D12PipelineState interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12pipelinestate)
pub trait IPipelineState:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12PipelineState>>
{
    /// Gets the cached blob representing the pipeline state.
    ///
    /// For more information: [`ID3D12PipelineState::GetCachedBlob interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12pipelinestate-getcachedblob)
    fn get_cached_blob(&self) -> Result<Blob, DxError>;
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

    fn get_cached_blob(&self) -> Result<Blob, DxError> {
        unsafe {
            self.0.GetCachedBlob()
                .map(|b| Blob::new(b))
                .map_err(DxError::from)
        }
    }
}
