use windows::Win32::Graphics::Direct3D12::*;

use crate::conv_flags;

use super::*;

conv_flags!(CommandQueueFlags to D3D12_COMMAND_QUEUE_FLAGS);
conv_flags!(DescriptorHeapFlags to D3D12_DESCRIPTOR_HEAP_FLAGS);
conv_flags!(FenceFlags to D3D12_FENCE_FLAGS);
conv_flags!(FormatSupport1 to D3D12_FORMAT_SUPPORT1);
conv_flags!(FormatSupport2 to D3D12_FORMAT_SUPPORT2);
conv_flags!(GpuBasedValidationFlags to D3D12_GPU_BASED_VALIDATION_FLAGS);
conv_flags!(TileRangeFlags to D3D12_TILE_RANGE_FLAGS);
conv_flags!(HeapFlags to D3D12_HEAP_FLAGS);
