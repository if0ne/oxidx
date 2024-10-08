use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{create_type, impl_trait, types::HeapDesc, HasInterface};

/// A heap is an abstraction of contiguous memory allocation, used to manage physical memory.
/// This heap can be used with [`IResource`](crate::resources::IResource)  objects to support placed resources or reserved resources.
///
/// For more information: [`ID3D12Heap interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12heap)
pub trait IHeap: for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12Heap>> {
    /// Gets the heap description.
    ///
    /// For more information: [`ID3D12Heap::GetDesc method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12heap-getdesc)
    fn get_desc(&self) -> HeapDesc;
}

create_type! {
    /// A heap is an abstraction of contiguous memory allocation, used to manage physical memory.
    /// This heap can be used with [`IResource`](crate::resources::IResource) objects to support placed resources or reserved resources.
    ///
    /// For more information: [`ID3D12Heap interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12heap)
    Heap wrap ID3D12Heap
}

impl_trait! {
    impl IHeap =>
    Heap;

    fn get_desc(&self) -> HeapDesc {
        unsafe {
            HeapDesc(self.0.GetDesc())
        }
    }
}
