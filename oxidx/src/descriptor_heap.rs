use windows::{core::Interface, Win32::Graphics::Direct3D12::*};

use crate::{create_type, impl_trait, types::CpuDescriptorHandle, HasInterface};

/// A descriptor heap is a collection of contiguous allocations of descriptors, one allocation for every descriptor.
/// Descriptor heaps contain many object types that are not part of a Pipeline State Object (PSO), such as Shader Resource Views (SRVs), Unordered Access Views (UAVs),
/// Constant Buffer Views (CBVs), and Samplers.
pub trait DescriptorHeapInterface: HasInterface<Raw: Interface> {
    /// Gets the CPU descriptor handle that represents the start of the heap.
    ///
    /// # Returns
    /// The CPU descriptor handle that represents the start of the heap.
    ///
    /// For more information: [`ID3D12DescriptorHeap::GetCPUDescriptorHandleForHeapStart method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12descriptorheap-getcpudescriptorhandleforheapstart)
    fn get_cpu_descriptor_handle_for_heap_start(&self) -> CpuDescriptorHandle;
}

create_type! {
    /// A descriptor heap is a collection of contiguous allocations of descriptors, one allocation for every descriptor.
    /// Descriptor heaps contain many object types that are not part of a Pipeline State Object (PSO), such as Shader Resource Views (SRVs), Unordered Access Views (UAVs),
    /// Constant Buffer Views (CBVs), and Samplers.
    DescriptorHeap wrap ID3D12DescriptorHeap
}

impl_trait! {
    impl DescriptorHeapInterface =>
    DescriptorHeap;

    fn get_cpu_descriptor_handle_for_heap_start(&self) -> CpuDescriptorHandle {
        unsafe {
            CpuDescriptorHandle(self.0.GetCPUDescriptorHandleForHeapStart().ptr)
        }
    }
}
