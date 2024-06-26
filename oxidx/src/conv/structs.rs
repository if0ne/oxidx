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
