use windows::{core::Interface, Win32::Graphics::Direct3D12::*};

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

#[derive(Clone, Debug)]
pub struct HeapProperties {
    pub r#type: HeapType,
    pub cpu_page_propery: CpuPageProperty,
    pub memory_pool_preference: MemoryPool,
    pub creation_node_mask: u32,
    pub visible_node_mask: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum HeapType {
    Default = D3D12_HEAP_TYPE_DEFAULT.0,
    Upload = D3D12_HEAP_TYPE_UPLOAD.0,
    Readback = D3D12_HEAP_TYPE_READBACK.0,
    Custom = D3D12_HEAP_TYPE_CUSTOM.0,
    GpuUpload = D3D12_HEAP_TYPE_GPU_UPLOAD.0,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum CpuPageProperty {
    Unknown = D3D12_CPU_PAGE_PROPERTY_UNKNOWN.0,
    NotAvailable = D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE.0,
    Combine = D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE.0,
    WriteBack = D3D12_CPU_PAGE_PROPERTY_WRITE_BACK.0,
}

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum MemoryPool {
    Unknown = D3D12_MEMORY_POOL_UNKNOWN.0,
    L0 = D3D12_MEMORY_POOL_L0.0,
    L1 = D3D12_MEMORY_POOL_L1.0,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct HeapFlags: i32 {
        const Shared = D3D12_HEAP_FLAG_SHARED.0;
        const DenyBuffers = D3D12_HEAP_FLAG_DENY_BUFFERS.0;
        const AllowDisplay = D3D12_HEAP_FLAG_ALLOW_DISPLAY.0;
        const SharedCrossAdapter = D3D12_HEAP_FLAG_SHARED_CROSS_ADAPTER.0;
        const DenyRtDsTextures = D3D12_HEAP_FLAG_DENY_RT_DS_TEXTURES.0;
        const DenyNonRtDsTextures = D3D12_HEAP_FLAG_DENY_NON_RT_DS_TEXTURES.0;
        const HardwareProtected = D3D12_HEAP_FLAG_HARDWARE_PROTECTED.0;
        const AllowWriteWatch = D3D12_HEAP_FLAG_ALLOW_WRITE_WATCH.0;
        const AllowSharedAtomics = D3D12_HEAP_FLAG_ALLOW_SHADER_ATOMICS.0;
        const CreateNotResident = D3D12_HEAP_FLAG_CREATE_NOT_RESIDENT.0;
        const CreateNotZeroed = D3D12_HEAP_FLAG_CREATE_NOT_ZEROED.0;
        const ToolsUseManualWriteTracking = D3D12_HEAP_FLAG_TOOLS_USE_MANUAL_WRITE_TRACKING.0;
        const AllowAllBuffersAndTextures = D3D12_HEAP_FLAG_ALLOW_ALL_BUFFERS_AND_TEXTURES.0;
        const AllowOnlyBuffers = D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS.0;
        const AllowOnlyNonRtDsTextures = D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES.0;
        const AllowOnlyRtDsTextures = D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES.0;
    }
}
