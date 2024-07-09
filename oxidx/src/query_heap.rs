use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::ID3D12QueryHeap,
};

use crate::{create_type, impl_trait, HasInterface};

/// Manages a query heap. A query heap holds an array of queries, referenced by indexes.
///
/// For more information: [`ID3D12QueryHeap interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12queryheap)
pub trait QueryHeapInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12QueryHeap>>
{
}

create_type! {
    /// Manages a query heap. A query heap holds an array of queries, referenced by indexes.
    ///
    /// For more information: [`ID3D12QueryHeap interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12queryheap)
    QueryHeap wrap ID3D12QueryHeap
}

impl_trait! {
    impl QueryHeapInterface =>
    QueryHeap;
}