use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::ID3D12Pageable,
};

use crate::{
    create_type, descriptor_heap::DescriptorHeap, heap::Heap, impl_trait, impl_up_down_cast,
    query_heap::QueryHeap, resources::Resource, HasInterface,
};

/// An interface from which many other core interfaces inherit from.
/// It indicates that the object type encapsulates some amount of GPU-accessible memory;
/// but does not strongly indicate whether the application can manipulate the object's residency.
///
/// For more information: [`ID3D12Pageable interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12pageable)
pub trait IPageable: for<'a> HasInterface<RawRef<'a>: Param<ID3D12Pageable>> {}

create_type! {
    /// An interface from which many other core interfaces inherit from.
    /// It indicates that the object type encapsulates some amount of GPU-accessible memory;
    /// but does not strongly indicate whether the application can manipulate the object's residency.
    ///
    /// For more information: [`ID3D12Pageable interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12pageable)
    Pageable wrap ID3D12Pageable
}

impl_trait! {
    impl IPageable => Pageable;
}

impl_up_down_cast!(DescriptorHeap inherit Pageable);
impl_up_down_cast!(Heap inherit Pageable);
impl_up_down_cast!(Resource inherit Pageable);
impl_up_down_cast!(QueryHeap inherit Pageable);
