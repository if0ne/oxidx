use std::ops::Range;

use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{
    create_type,
    error::DxError,
    impl_trait,
    types::{Box, GpuVirtualAddress, HeapFlags, HeapProperties, ResourceDesc},
    HasInterface,
};

/// Encapsulates a generalized ability of the CPU and GPU to read and write to physical memory, or heaps.
/// It contains abstractions for organizing and manipulating simple arrays of data as well as multidimensional data optimized for shader sampling.
///
/// For more information: [`ID3D12Resource interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12resource)
pub trait IResource:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12Resource>>
{
    /// Gets the resource description.
    ///
    /// For more information: [`ID3D12Resource::GetDesc method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12resource-getdesc)
    fn get_desc(&self) -> ResourceDesc;

    /// This method returns the GPU virtual address of a buffer resource.
    ///
    /// For more information: [`ID3D12Resource::GetGPUVirtualAddress method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12resource-getgpuvirtualaddress)
    fn get_gpu_virtual_address(&self) -> GpuVirtualAddress;

    /// Retrieves the properties of the resource heap, for placed and committed resources.
    ///
    /// For more information: [`ID3D12Resource::GetHeapProperties method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12resource-getheapproperties)
    fn get_heap_properties(&self) -> Result<(HeapProperties, HeapFlags), DxError>;

    /// Gets a CPU pointer to the specified subresource in the resource, but may not disclose the pointer value to applications.
    /// Map also invalidates the CPU cache, when necessary, so that CPU reads to this address reflect any modifications made by the GPU.
    ///
    /// For more information: [`ID3D12Resource::Map method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12resource-map)
    fn map<T>(
        &self,
        subresource: u32,
        read_range: Option<Range<usize>>,
    ) -> Result<std::ptr::NonNull<T>, DxError>;

    /// Uses the CPU to copy data from a subresource, enabling the CPU to read the contents of most textures with undefined layouts.
    ///
    /// For more information: [`ID3D12Resource::ReadFromSubresource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12resource-readfromsubresource)
    fn read_from_subresource(
        &self,
        dst_data: &mut [u8],
        dst_row_pitch: u32,
        dst_depth_pitch: u32,
        src_subresource: u32,
        src_box: Option<&Box>,
    ) -> Result<(), DxError>;

    /// Invalidates the CPU pointer to the specified subresource in the resource.
    ///
    /// For more information: [`ID3D12Resource::Unmap method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12resource-unmap)
    fn unmap(&self, subresource: u32, written_range: Option<Range<usize>>);

    /// Uses the CPU to copy data into a subresource, enabling the CPU to modify the contents of most textures with undefined layouts.
    ///
    /// For more information: [`ID3D12Resource::WriteToSubresource method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12resource-writetosubresource)
    fn write_to_subresource(
        &self,
        dst_subresource: u32,
        dst_box: Option<&Box>,
        src_data: &mut [u8],
        src_row_pitch: u32,
        src_depth_pitch: u32,
    ) -> Result<(), DxError>;
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

    fn get_desc(&self) -> ResourceDesc {
        unsafe {
            self.0.GetDesc().into()
        }
    }

    fn get_heap_properties(&self) -> Result<(HeapProperties, HeapFlags), DxError> {
        unsafe {
            let mut properties = Default::default();
            let mut flags = Default::default();

            self.0.GetHeapProperties(Some(&mut properties), Some(&mut flags)).map_err(DxError::from)?;

            Ok((properties.into(), flags.into()))
        }
    }

    fn get_gpu_virtual_address(&self) -> GpuVirtualAddress {
        unsafe {
            self.0.GetGPUVirtualAddress()
        }
    }

    fn map<T>(&self, subresource: u32, read_range: Option<Range<usize>>) -> Result<std::ptr::NonNull<T>, DxError> {
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

            Ok(std::ptr::NonNull::new(ptr as *mut _).expect("Expected valid pointer"))
        }
    }

    fn read_from_subresource(
        &self,
        dst_data: &mut [u8],
        dst_row_pitch: u32,
        dst_depth_pitch: u32,
        src_subresource: u32,
        src_box: Option<&Box>,
    ) -> Result<(), DxError> {
        unsafe {
            let src_box = src_box.map(|s| s.as_raw());
            let src_box = src_box.as_ref().map(|s| s as *const _);

            self.0.ReadFromSubresource(
                dst_data.as_mut_ptr() as *mut _,
                dst_row_pitch,
                dst_depth_pitch,
                src_subresource,
                src_box
            ).map_err(DxError::from)
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

    fn write_to_subresource(
        &self,
        dst_subresource: u32,
        dst_box: Option<&Box>,
        src_data: &mut [u8],
        src_row_pitch: u32,
        src_depth_pitch: u32,
    ) -> Result<(), DxError> {
        unsafe {
            let dst_box = dst_box.map(|s| s.as_raw());
            let dst_box = dst_box.as_ref().map(|s| s as *const _);

            self.0.WriteToSubresource(
                dst_subresource,
                dst_box,
                src_data.as_mut_ptr() as *mut _,
                src_row_pitch,
                src_depth_pitch
            ).map_err(DxError::from)
        }
    }
}
