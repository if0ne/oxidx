use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{create_type, impl_trait, types::HeapDesc, HasInterface};

/// A heap is an abstraction of contiguous memory allocation, used to manage physical memory.
/// This heap can be used with [`ResourceInterface`](crate::resources::ResourceInterface) objects to support placed resources or reserved resources.
///
/// For more information: [`ID3D12Heap interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12heap)
pub trait IHeap: for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12Heap>> {
    fn get_desc(&self) -> HeapDesc;
}

create_type! {
    /// A heap is an abstraction of contiguous memory allocation, used to manage physical memory.
    /// This heap can be used with [`ResourceInterface`](crate::resources::ResourceInterface) objects to support placed resources or reserved resources.
    ///
    /// For more information: [`ID3D12Heap interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12heap)
    Heap wrap ID3D12Heap
}

impl_trait! {
    impl IHeap =>
    Heap;

    fn get_desc(&self) -> HeapDesc {
        unsafe {
            self.0.GetDesc().into()
        }
    }
}
