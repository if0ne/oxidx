use windows::{core::Interface, Win32::Graphics::Direct3D12::ID3D12DescriptorHeap};

use crate::{create_type, impl_trait, HasInterface};

pub trait DescriptorHeapInterface: HasInterface<Raw: Interface> {
    fn get_cpu_descriptor_handle_for_heap_start(&self) -> CpuDescriptorHandle;
}

create_type! { DescriptorHeap wrap ID3D12DescriptorHeap }

impl_trait! {
    impl DescriptorHeapInterface =>
    DescriptorHeap;

    fn get_cpu_descriptor_handle_for_heap_start(&self) -> CpuDescriptorHandle {
        unsafe {
            CpuDescriptorHandle(self.0.GetCPUDescriptorHandleForHeapStart().ptr)
        }
    }
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

#[derive(Clone, Debug)]
pub struct CpuDescriptorHandle(pub(crate) usize);
