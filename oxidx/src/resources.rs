use std::ops::Range;

use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{create_type, error::DxError, impl_trait, HasInterface};

/// Encapsulates a generalized ability of the CPU and GPU to read and write to physical memory, or heaps.
/// It contains abstractions for organizing and manipulating simple arrays of data as well as multidimensional data optimized for shader sampling.
///
/// For more information: [`ID3D12Resource interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12resource)
pub trait IResource:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12Resource>>
{
    /// This method returns the GPU virtual address of a buffer resource.
    ///
    /// For more information: [`ID3D12Resource::GetGPUVirtualAddress method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12resource-getgpuvirtualaddress)
    fn get_gpu_virtual_address(&self) -> u64;

    /// Gets a CPU pointer to the specified subresource in the resource, but may not disclose the pointer value to applications.
    /// Map also invalidates the CPU cache, when necessary, so that CPU reads to this address reflect any modifications made by the GPU.
    ///
    /// For more information: [`ID3D12Resource::Map method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12resource-map)
    fn map(&self, subresource: u32, read_range: Option<Range<usize>>) -> Result<*mut (), DxError>;

    /// Invalidates the CPU pointer to the specified subresource in the resource.
    ///
    /// For more information: [`ID3D12Resource::Unmap method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12resource-unmap)
    fn unmap(&self, subresource: u32, written_range: Option<Range<usize>>);
}

create_type! {
    /// Encapsulates a generalized ability of the CPU and GPU to read and write to physical memory, or heaps.
    /// It contains abstractions for organizing and manipulating simple arrays of data as well as multidimensional data optimized for shader sampling.
    ///
    /// For more information: [`ID3D12Resource interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12resource)
    Resource wrap ID3D12Resource
}

impl_trait! {
    impl IResource =>
    Resource;

    fn map(&self, subresource: u32, read_range: Option<Range<usize>>) -> Result<*mut (), DxError> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let range = read_range.map(|r| D3D12_RANGE {
                Begin: r.start,
                End: r.end,
            });

            self.0
                .Map(
                    subresource,
                    range.as_ref().map(|r| r as *const _),
                    Some(&mut ptr),
                )
                .map_err(DxError::from)?;

            Ok(ptr as *mut ())
        }
    }

    fn unmap(&self, subresource: u32, written_range: Option<Range<usize>>) {
        unsafe {
            let range = written_range.map(|r| D3D12_RANGE {
                Begin: r.start,
                End: r.end,
            });

            self.0
                .Unmap(subresource, range.as_ref().map(|r| r as *const _));
        }
    }

    fn get_gpu_virtual_address(&self) -> u64 {
        unsafe {
            self.0.GetGPUVirtualAddress()
        }
    }
}
