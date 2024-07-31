use std::ops::Range;

use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{create_type, error::DxError, impl_trait, HasInterface};

pub trait IResource:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12Resource>>
{
    //TODO: Type for pointer
    fn map(&self, subresource: u32, read_range: Option<Range<usize>>) -> Result<*mut (), DxError>;
    fn unmap(&self, subresource: u32, written_range: Option<Range<usize>>);
    fn get_gpu_virtual_address(&self) -> u64;
}

create_type! { Resource wrap ID3D12Resource }

impl_trait! {
    impl IResource =>
    Resource;

    fn map(&self, subresource: u32, read_range: Option<Range<usize>>) -> Result<*mut (), DxError> {
        let mut ptr = std::ptr::null_mut();
        let range = read_range.map(|r| D3D12_RANGE {
            Begin: r.start,
            End: r.end,
        });

        unsafe {
            self.0
                .Map(
                    subresource,
                    range.as_ref().map(|r| r as *const _),
                    Some(&mut ptr),
                )
                .map_err(|_| DxError::Dummy)?;
        }

        Ok(ptr as *mut ())
    }

    fn unmap(&self, subresource: u32, written_range: Option<Range<usize>>) {
        let range = written_range.map(|r| D3D12_RANGE {
            Begin: r.start,
            End: r.end,
        });

        unsafe {
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
