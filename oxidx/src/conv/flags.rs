use windows::Win32::Graphics::Direct3D12::*;

use crate::conv_flags;

use super::*;

conv_flags!(BufferSrvFlags to D3D12_BUFFER_SRV_FLAGS);
conv_flags!(BufferUavFlags to D3D12_BUFFER_UAV_FLAGS);
conv_flags!(CacheSupportFlags to D3D12_SHADER_CACHE_SUPPORT_FLAGS);
conv_flags!(CallbackFlags to D3D12_MESSAGE_CALLBACK_FLAGS);
conv_flags!(ClearFlags to D3D12_CLEAR_FLAGS);
conv_flags!(ColorWriteEnable to D3D12_COLOR_WRITE_ENABLE);
conv_flags!(CommandListSupportFlags to D3D12_COMMAND_LIST_SUPPORT_FLAGS);
conv_flags!(CommandQueueFlags to D3D12_COMMAND_QUEUE_FLAGS);
conv_flags!(DepthWriteMask to D3D12_DEPTH_WRITE_MASK);
conv_flags!(DescriptorHeapFlags to D3D12_DESCRIPTOR_HEAP_FLAGS);
conv_flags!(DsvFlags to D3D12_DSV_FLAGS);
conv_flags!(FenceFlags to D3D12_FENCE_FLAGS);
conv_flags!(FormatSupport1 to D3D12_FORMAT_SUPPORT1);
conv_flags!(FormatSupport2 to D3D12_FORMAT_SUPPORT2);
conv_flags!(FrameBufferUsage to DXGI_USAGE);
conv_flags!(GpuBasedValidationFlags to D3D12_GPU_BASED_VALIDATION_FLAGS);
conv_flags!(HeapFlags to D3D12_HEAP_FLAGS);
conv_flags!(MultisampleQualityLevelFlags to D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS);
conv_flags!(PipelineStateFlags to D3D12_PIPELINE_STATE_FLAGS);
conv_flags!(ProtectedResourceSessionSupportFlags to D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS);
conv_flags!(ResourceBarrierFlags to D3D12_RESOURCE_BARRIER_FLAGS);
conv_flags!(ResourceFlags to D3D12_RESOURCE_FLAGS);
conv_flags!(ResourceStates to D3D12_RESOURCE_STATES);
conv_flags!(RootSignatureFlags to D3D12_ROOT_SIGNATURE_FLAGS);
conv_flags!(TileCopyFlags to D3D12_TILE_COPY_FLAGS);
conv_flags!(TileRangeFlags to D3D12_TILE_RANGE_FLAGS);
