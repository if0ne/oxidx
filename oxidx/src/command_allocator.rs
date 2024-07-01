use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::ID3D12CommandAllocator,
};

use crate::{create_type, error::DxError, impl_trait, HasInterface};

/// Represents the allocations of storage for graphics processing unit (GPU) commands.
///
/// For more information: [`ID3D12CommandAllocator interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12commandallocator)
pub trait CommandAllocatorInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12CommandAllocator>>
{
    /// Indicates to re-use the memory that is associated with the command allocator.
    ///
    /// For more information: [`ID3D12CommandAllocator::Reset method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandallocator-reset)
    fn reset(&self) -> Result<(), DxError>;
}

create_type! {
    /// Represents the allocations of storage for graphics processing unit (GPU) commands.
    ///
    /// For more information: [`ID3D12CommandAllocator interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12commandallocator)
    CommandAllocator wrap ID3D12CommandAllocator
}

impl_trait! {
    impl CommandAllocatorInterface =>
    CommandAllocator;

    fn reset(&self) -> Result<(), DxError> {
        unsafe {
            self.0.Reset().map_err(DxError::from)?;
        }

        Ok(())
    }
}
