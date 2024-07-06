use smallvec::SmallVec;
use windows::Win32::Graphics::Direct3D12::*;

use super::*;

impl CommandQueueDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_COMMAND_QUEUE_DESC {
        D3D12_COMMAND_QUEUE_DESC {
            Type: self.r#type.as_raw(),
            Priority: self.priority.as_raw(),
            Flags: self.flags.as_raw(),
            NodeMask: self.node_mask,
        }
    }
}

impl<'a> CommandSignatureDesc<'a> {
    #[inline(always)]
    pub(crate) fn as_raw(&self) -> D3D12_COMMAND_SIGNATURE_DESC {
        let num_argument_descs = self.argument_descs.len() as u32;
        let argument_descs = self
            .argument_descs
            .iter()
            .map(|a| a.as_raw())
            .collect::<SmallVec<[_; 16]>>();

        D3D12_COMMAND_SIGNATURE_DESC {
            ByteStride: self.byte_stride,
            NumArgumentDescs: num_argument_descs,
            pArgumentDescs: argument_descs.as_ptr(),
            NodeMask: self.node_mask,
        }
    }
}

impl<'a> ComputePipelineStateDesc<'a> {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_COMPUTE_PIPELINE_STATE_DESC {
        unsafe {
            D3D12_COMPUTE_PIPELINE_STATE_DESC {
                pRootSignature: std::mem::transmute_copy(self.root_signature.as_raw()),
                CS: self.cs.as_shader_bytecode(),
                NodeMask: self.node_mask,
                CachedPSO: self
                    .cached_pso
                    .map(|pso| pso.as_cached_pipeline_state())
                    .unwrap_or_default(),
                Flags: self.flags.as_raw(),
            }
        }
    }
}

impl ConstantBufferViewDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_CONSTANT_BUFFER_VIEW_DESC {
        D3D12_CONSTANT_BUFFER_VIEW_DESC {
            BufferLocation: self.buffer_location,
            SizeInBytes: self.size_in_bytes,
        }
    }
}

impl CpuDescriptorHandle {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        D3D12_CPU_DESCRIPTOR_HANDLE { ptr: self.0 }
    }
}

impl From<D3D12_CPU_DESCRIPTOR_HANDLE> for CpuDescriptorHandle {
    #[inline]
    fn from(value: D3D12_CPU_DESCRIPTOR_HANDLE) -> Self {
        Self(value.ptr)
    }
}

impl GpuDescriptorHandle {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        D3D12_GPU_DESCRIPTOR_HANDLE { ptr: self.0 as u64 }
    }
}

impl From<D3D12_GPU_DESCRIPTOR_HANDLE> for GpuDescriptorHandle {
    #[inline]
    fn from(value: D3D12_GPU_DESCRIPTOR_HANDLE) -> Self {
        Self(value.ptr as usize)
    }
}

impl From<D3D12_COMMAND_QUEUE_DESC> for CommandQueueDesc {
    #[inline]
    fn from(value: D3D12_COMMAND_QUEUE_DESC) -> Self {
        Self {
            r#type: value.Type.into(),
            priority: value.Priority.into(),
            flags: value.Flags.into(),
            node_mask: value.NodeMask,
        }
    }
}

impl DescriptorHeapDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_DESCRIPTOR_HEAP_DESC {
        D3D12_DESCRIPTOR_HEAP_DESC {
            Type: self.r#type.as_raw(),
            NumDescriptors: self.num,
            Flags: self.flags.as_raw(),
            NodeMask: self.node_mask,
        }
    }
}

impl From<D3D12_DESCRIPTOR_HEAP_DESC> for DescriptorHeapDesc {
    #[inline]
    fn from(value: D3D12_DESCRIPTOR_HEAP_DESC) -> Self {
        Self {
            r#type: value.Type.into(),
            num: value.NumDescriptors,
            flags: value.Flags.into(),
            node_mask: value.NodeMask,
        }
    }
}

impl DepthStencilViewDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_DEPTH_STENCIL_VIEW_DESC {
        D3D12_DEPTH_STENCIL_VIEW_DESC {
            Format: self.format.as_raw(),
            ViewDimension: self.view_dimension.as_raw(),
            Flags: self.flags.as_raw(),
            Anonymous: match self.view_dimension {
                DsvDimension::Tex1D { mip_slice } => D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture1D: D3D12_TEX1D_DSV {
                        MipSlice: mip_slice,
                    },
                },
                DsvDimension::ArrayTex1D {
                    mip_slice,
                    first_array_slice,
                    array_size,
                } => D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture1DArray: D3D12_TEX1D_ARRAY_DSV {
                        MipSlice: mip_slice,
                        FirstArraySlice: first_array_slice,
                        ArraySize: array_size,
                    },
                },
                DsvDimension::Tex2D { mip_slice } => D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2D: D3D12_TEX2D_DSV {
                        MipSlice: mip_slice,
                    },
                },
                DsvDimension::ArrayTex2D {
                    mip_slice,
                    first_array_slice,
                    array_size,
                } => D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DArray: D3D12_TEX2D_ARRAY_DSV {
                        MipSlice: mip_slice,
                        FirstArraySlice: first_array_slice,
                        ArraySize: array_size,
                    },
                },
                DsvDimension::Tex2DMs => D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DMS: D3D12_TEX2DMS_DSV::default(),
                },
                DsvDimension::ArrayTex2DMs {
                    first_array_slice,
                    array_size,
                } => D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DMSArray: D3D12_TEX2DMS_ARRAY_DSV {
                        FirstArraySlice: first_array_slice,
                        ArraySize: array_size,
                    },
                },
            },
        }
    }
}

impl HeapDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_HEAP_DESC {
        D3D12_HEAP_DESC {
            SizeInBytes: self.size,
            Properties: self.props.as_raw(),
            Alignment: self.alignment.as_raw(),
            Flags: self.flags.as_raw(),
        }
    }
}

impl From<D3D12_HEAP_DESC> for HeapDesc {
    #[inline]
    fn from(value: D3D12_HEAP_DESC) -> Self {
        Self {
            size: value.SizeInBytes,
            props: value.Properties.into(),
            alignment: value.Alignment.into(),
            flags: value.Flags.into(),
        }
    }
}

impl HeapProperties {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_HEAP_PROPERTIES {
        D3D12_HEAP_PROPERTIES {
            Type: self.r#type.as_raw(),
            CPUPageProperty: self.cpu_page_propery.as_raw(),
            MemoryPoolPreference: self.memory_pool_preference.as_raw(),
            CreationNodeMask: self.creation_node_mask,
            VisibleNodeMask: self.visible_node_mask,
        }
    }
}

impl From<D3D12_HEAP_PROPERTIES> for HeapProperties {
    #[inline]
    fn from(value: D3D12_HEAP_PROPERTIES) -> Self {
        Self {
            r#type: value.Type.into(),
            cpu_page_propery: value.CPUPageProperty.into(),
            memory_pool_preference: value.MemoryPoolPreference.into(),
            creation_node_mask: value.CreationNodeMask,
            visible_node_mask: value.VisibleNodeMask,
        }
    }
}

impl ResourceDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_RESOURCE_DESC {
        D3D12_RESOURCE_DESC {
            Dimension: self.dimension.as_raw(),
            Alignment: self.alignment,
            Width: self.width,
            Height: self.height,
            DepthOrArraySize: self.depth_or_array_size,
            MipLevels: self.mip_levels,
            Format: self.format.as_raw(),
            SampleDesc: self.sample_desc.as_raw(),
            Layout: self.layout.as_raw(),
            Flags: self.flags.as_raw(),
        }
    }
}

impl SampleDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> DXGI_SAMPLE_DESC {
        DXGI_SAMPLE_DESC {
            Count: self.count,
            Quality: self.quality,
        }
    }
}

impl TileRegionSize {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_TILE_REGION_SIZE {
        D3D12_TILE_REGION_SIZE {
            NumTiles: self.num_tiles,
            UseBox: self.use_box.into(),
            Width: self.width,
            Height: self.height,
            Depth: self.depth,
        }
    }
}

impl TiledResourceCoordinate {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_TILED_RESOURCE_COORDINATE {
        D3D12_TILED_RESOURCE_COORDINATE {
            X: self.x,
            Y: self.y,
            Z: self.z,
            Subresource: self.subresource,
        }
    }
}
