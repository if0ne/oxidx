use windows::{core::Interface, Win32::Graphics::Direct3D12::ID3D12DescriptorHeap};

use crate::{create_type, impl_trait, HasInterface};

pub trait DescriptorHeapInterface: HasInterface<Raw: Interface> {}

create_type! { DescriptorHeap wrap ID3D12DescriptorHeap }

impl_trait! {
    impl DescriptorHeapInterface =>
    DescriptorHeap;
}

#[derive(Clone, Debug)]
pub struct DescriptorHeapDesc {
    pub r#type: DescriptorHeapType,
    pub num: u32,
    pub flags: DescriptorHeapFlags,
    pub node_mask: u32,
}

#[derive(Clone, Debug)]
pub enum DescriptorHeapType {
    Rtv,
    Dsv,
    CbvSrvUav,
    Sampler,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct DescriptorHeapFlags: i32 {
        const None = 0;
        const ShaderVisible = 1;
    }
}
