use windows::Win32::Graphics::Direct3D12::*;

use crate::{
    create_type, impl_interface,
    types::{CpuDescriptorHandle, DescriptorHeapDesc, GpuDescriptorHandle},
};

create_type! {
    /// A descriptor heap is a collection of contiguous allocations of descriptors, one allocation for every descriptor.
    /// Descriptor heaps contain many object types that are not part of a Pipeline State Object (PSO), such as Shader Resource Views (SRVs), Unordered Access Views (UAVs),
    /// Constant Buffer Views (CBVs), and Samplers.
    DescriptorHeap wrap ID3D12DescriptorHeap
}

impl_interface! {
    DescriptorHeap;

    /// Gets the CPU descriptor handle that represents the start of the heap.
    ///
    /// For more information: [`ID3D12DescriptorHeap::GetCPUDescriptorHandleForHeapStart method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12descriptorheap-getcpudescriptorhandleforheapstart)
    pub fn get_cpu_descriptor_handle_for_heap_start(&self) -> CpuDescriptorHandle {
        unsafe {
            CpuDescriptorHandle(self.0.GetCPUDescriptorHandleForHeapStart())
        }
    }

    /// Gets the descriptor heap description.
    ///
    /// For more information: [`ID3D12DescriptorHeap::GetDesc method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12descriptorheap-getdesc)
    pub fn get_desc(&self) -> DescriptorHeapDesc {
        unsafe {
            DescriptorHeapDesc(self.0.GetDesc())
        }
    }

    /// Gets the GPU descriptor handle that represents the start of the heap.
    ///
    /// For more information: [`ID3D12DescriptorHeap::GetGPUDescriptorHandleForHeapStart method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12descriptorheap-getgpudescriptorhandleforheapstart)
    pub fn get_gpu_descriptor_handle_for_heap_start(&self) -> GpuDescriptorHandle {
        unsafe {
            GpuDescriptorHandle(self.0.GetGPUDescriptorHandleForHeapStart())
        }
    }
}
