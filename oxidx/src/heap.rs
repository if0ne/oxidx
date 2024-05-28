use windows::{
    core::Interface,
    Win32::Graphics::Direct3D12::{
        ID3D12DescriptorHeap, D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
        D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
    },
};

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

#[derive(Clone, Debug, Default)]
pub struct DescriptorHeapDesc {
    pub r#type: DescriptorHeapType,
    pub num: u32,
    pub flags: DescriptorHeapFlags,
    pub node_mask: u32,
}

#[derive(Clone, Debug, Default)]
pub enum DescriptorHeapType {
    #[default]
    Rtv,
    Dsv,
    CbvSrvUav,
    Sampler,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct DescriptorHeapFlags: i32 {
        const None = D3D12_DESCRIPTOR_HEAP_FLAG_NONE.0;
        const ShaderVisible = D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE.0;
    }
}

#[derive(Clone, Debug)]
pub struct CpuDescriptorHandle(pub usize);

impl CpuDescriptorHandle {
    pub fn offset(&self, offset: usize) -> Self {
        Self(self.0 + offset)
    }
}
