use windows::Win32::Graphics::Direct3D12::ID3D12QueryHeap;

use crate::{create_type, impl_interface};

create_type! {
    /// Manages a query heap. A query heap holds an array of queries, referenced by indexes.
    ///
    /// For more information: [`ID3D12QueryHeap interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12queryheap)
    QueryHeap wrap ID3D12QueryHeap
}

impl_interface! {
    QueryHeap;
}
