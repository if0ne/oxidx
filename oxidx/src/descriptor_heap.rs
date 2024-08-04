use windows::{core::Interface, Win32::Graphics::Direct3D12::*};

use crate::{
    create_type, impl_trait,
    types::{CpuDescriptorHandle, DescriptorHeapDesc, GpuDescriptorHandle},
    HasInterface,
};

/// A descriptor heap is a collection of contiguous allocations of descriptors, one allocation for every descriptor.
/// Descriptor heaps contain many object types that are not part of a Pipeline State Object (PSO), such as Shader Resource Views (SRVs), Unordered Access Views (UAVs),
/// Constant Buffer Views (CBVs), and Samplers.
pub trait IDescriptorHeap: HasInterface<Raw: Interface> {
    /// Gets the CPU descriptor handle that represents the start of the heap.
    ///
    /// For more information: [`ID3D12DescriptorHeap::GetCPUDescriptorHandleForHeapStart method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12descriptorheap-getcpudescriptorhandleforheapstart)
    fn get_cpu_descriptor_handle_for_heap_start(&self) -> CpuDescriptorHandle;

    /// Gets the descriptor heap description.
    ///
    /// For more information: [`ID3D12DescriptorHeap::GetDesc method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12descriptorheap-getdesc)
    fn get_desc(&self) -> DescriptorHeapDesc;

    /// Gets the GPU descriptor handle that represents the start of the heap.
    ///
    /// For more information: [`ID3D12DescriptorHeap::GetGPUDescriptorHandleForHeapStart method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12descriptorheap-getgpudescriptorhandleforheapstart)
    fn get_gpu_descriptor_handle_for_heap_start(&self) -> GpuDescriptorHandle;
}

create_type! {
    /// A descriptor heap is a collection of contiguous allocations of descriptors, one allocation for every descriptor.
    /// Descriptor heaps contain many object types that are not part of a Pipeline State Object (PSO), such as Shader Resource Views (SRVs), Unordered Access Views (UAVs),
    /// Constant Buffer Views (CBVs), and Samplers.
    DescriptorHeap wrap ID3D12DescriptorHeap
}

impl_trait! {
    impl IDescriptorHeap =>
    DescriptorHeap;

    fn get_cpu_descriptor_handle_for_heap_start(&self) -> CpuDescriptorHandle {
        unsafe {
            self.0.GetCPUDescriptorHandleForHeapStart().into()
        }
    }

    fn get_desc(&self) -> DescriptorHeapDesc {
        unsafe {
            self.0.GetDesc().into()
        }
    }

    fn get_gpu_descriptor_handle_for_heap_start(&self) -> GpuDescriptorHandle {
        unsafe {
            self.0.GetGPUDescriptorHandleForHeapStart().into()
        }
    }
}
