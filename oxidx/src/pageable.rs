use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::ID3D12Pageable,
};

use crate::{
    create_type, descriptor_heap::DescriptorHeap, heap::Heap, impl_trait, impl_up_down_cast,
    query_heap::QueryHeap, resources::Resource, HasInterface,
};

pub trait PageableInterface: for<'a> HasInterface<RawRef<'a>: Param<ID3D12Pageable>> {}

create_type!(
    Pageable wrap ID3D12Pageable
);

impl_trait! {
    impl PageableInterface => Pageable;
}

impl_up_down_cast!(DescriptorHeap inherit Pageable);
impl_up_down_cast!(Heap inherit Pageable);
impl_up_down_cast!(Resource inherit Pageable);
impl_up_down_cast!(QueryHeap inherit Pageable);
