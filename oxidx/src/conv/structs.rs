use smallvec::SmallVec;
use windows::Win32::Graphics::Direct3D12::*;

use super::*;

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

impl IndexBufferView {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_INDEX_BUFFER_VIEW {
        D3D12_INDEX_BUFFER_VIEW {
            BufferLocation: self.buffer_location,
            SizeInBytes: self.size_in_bytes,
            Format: self.format.as_raw(),
        }
    }
}

impl From<D3D12_PACKED_MIP_INFO> for PackedMipDesc {
    #[inline]
    fn from(value: D3D12_PACKED_MIP_INFO) -> Self {
        Self {
            num_standard_mips: value.NumStandardMips,
            num_packed_mips: value.NumPackedMips,
            num_tiles_for_packed_mips: value.NumTilesForPackedMips,
            start_tile_index_in_overall_resource: value.StartTileIndexInOverallResource,
        }
    }
}

impl PlacedSubresourceFootprint {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
        D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
            Offset: self.offset,
            Footprint: self.footprint.as_raw(),
        }
    }
}

impl From<D3D12_PLACED_SUBRESOURCE_FOOTPRINT> for PlacedSubresourceFootprint {
    #[inline]
    fn from(value: D3D12_PLACED_SUBRESOURCE_FOOTPRINT) -> Self {
        Self {
            offset: value.Offset,
            footprint: value.Footprint.into(),
        }
    }
}

impl QueryHeapDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_QUERY_HEAP_DESC {
        D3D12_QUERY_HEAP_DESC {
            Type: self.r#type.as_raw(),
            Count: self.count,
            NodeMask: self.node_mask,
        }
    }
}

impl Rational {
    pub(crate) fn as_raw(&self) -> DXGI_RATIONAL {
        DXGI_RATIONAL {
            Numerator: self.numerator,
            Denominator: self.denominator,
        }
    }
}

impl Rect {
    pub(crate) fn as_raw(&self) -> RECT {
        RECT {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }
}

impl<'a> RootParameter<'a> {
    #[inline]
    pub(crate) fn as_raw<const N: usize>(
        &self,
        param: &[SmallVec<[D3D12_DESCRIPTOR_RANGE; N]>],
    ) -> D3D12_ROOT_PARAMETER {
        D3D12_ROOT_PARAMETER {
            ParameterType: self.r#type.as_type_raw(),
            Anonymous: self.r#type.as_raw(param),
            ShaderVisibility: self.visibility.as_raw(),
        }
    }
}

impl<'a> RootSignatureDesc<'a> {
    #[inline]
    pub(crate) fn as_raw<const N: usize>(
        &self,
        params: &[SmallVec<[D3D12_DESCRIPTOR_RANGE; N]>],
    ) -> D3D12_ROOT_SIGNATURE_DESC {
        let parameters = self
            .parameters
            .iter()
            .map(|param| param.as_raw(params))
            .collect::<SmallVec<[_; 16]>>();
        let sampler = self
            .samplers
            .iter()
            .map(|sampler| sampler.as_raw())
            .collect::<SmallVec<[_; 16]>>();

        D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: self.parameters.len() as u32,
            pParameters: parameters.as_ptr(),
            NumStaticSamplers: self.samplers.len() as u32,
            pStaticSamplers: sampler.as_ptr(),
            Flags: self.flags.as_raw(),
        }
    }
}

impl ShaderResourceViewDesc {
    pub(crate) fn as_raw(&self) -> D3D12_SHADER_RESOURCE_VIEW_DESC {
        D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: self.format.as_raw(),
            ViewDimension: self.dimension.as_type_raw(),
            Anonymous: self.dimension.as_raw(),
            // TODO: Make it in shader resource view desc
            Shader4ComponentMapping: 0x7,
        }
    }
}

impl StaticSamplerDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_STATIC_SAMPLER_DESC {
        D3D12_STATIC_SAMPLER_DESC {
            Filter: self.filter.as_raw(),
            AddressU: self.address_u.as_raw(),
            AddressV: self.address_v.as_raw(),
            AddressW: self.address_w.as_raw(),
            MipLODBias: self.mip_lod_bias,
            MaxAnisotropy: self.max_anisotropy,
            ComparisonFunc: self.comparison_func.as_raw(),
            BorderColor: self.border_color.as_raw(),
            MinLOD: self.min_lod,
            MaxLOD: self.max_lod,
            ShaderRegister: self.shader_register,
            RegisterSpace: self.register_space,
            ShaderVisibility: self.visibility.as_raw(),
        }
    }
}

impl StreamOutputBufferView {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_STREAM_OUTPUT_BUFFER_VIEW {
        D3D12_STREAM_OUTPUT_BUFFER_VIEW {
            BufferLocation: self.buffer_location,
            SizeInBytes: self.size_in_bytes,
            BufferFilledSizeLocation: self.buffer_filled_size_location,
        }
    }
}

impl SubresourceFootprint {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_SUBRESOURCE_FOOTPRINT {
        D3D12_SUBRESOURCE_FOOTPRINT {
            Format: self.format.as_raw(),
            Width: self.width,
            Height: self.height,
            Depth: self.depth,
            RowPitch: self.row_pitch,
        }
    }
}

impl From<D3D12_SUBRESOURCE_FOOTPRINT> for SubresourceFootprint {
    #[inline]
    fn from(value: D3D12_SUBRESOURCE_FOOTPRINT) -> Self {
        Self {
            format: value.Format.into(),
            width: value.Width,
            height: value.Height,
            depth: value.Depth,
            row_pitch: value.RowPitch,
        }
    }
}

impl From<D3D12_SUBRESOURCE_TILING> for SubresourceTiling {
    #[inline]
    fn from(value: D3D12_SUBRESOURCE_TILING) -> Self {
        Self {
            width_in_tiles: value.WidthInTiles,
            height_in_tiles: value.HeightInTiles,
            depth_in_tiles: value.DepthInTiles,
            start_tile_index_in_overall_resource: value.StartTileIndexInOverallResource,
        }
    }
}

impl SwapchainFullscreenDesc {
    pub(crate) fn as_raw(&self) -> DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
        DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
            RefreshRate: self.rational.as_raw(),
            ScanlineOrdering: self.scanline_ordering.as_raw(),
            Scaling: self.scaling.as_raw(),
            Windowed: self.windowed.into(),
        }
    }
}

impl<'a> TextureCopyLocation<'a> {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_TEXTURE_COPY_LOCATION {
        unsafe {
            D3D12_TEXTURE_COPY_LOCATION {
                pResource: std::mem::transmute_copy(self.resource.as_raw()),
                Type: self.r#type.as_raw_type(),
                Anonymous: self.r#type.as_raw(),
            }
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

impl From<D3D12_TILE_SHAPE> for TileShape {
    #[inline]
    fn from(value: D3D12_TILE_SHAPE) -> Self {
        Self {
            width_in_texels: value.WidthInTexels,
            height_in_texels: value.HeightInTexels,
            depth_in_texels: value.DepthInTexels,
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

impl UnorderedAccessViewDesc {
    #[inline]
    pub(crate) fn as_raw(&self) -> D3D12_UNORDERED_ACCESS_VIEW_DESC {
        D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: self.format.as_raw(),
            ViewDimension: self.dimension.as_type_raw(),
            Anonymous: self.dimension.as_raw(),
        }
    }
}

impl VertexBufferView {
    pub(crate) fn as_raw(&self) -> D3D12_VERTEX_BUFFER_VIEW {
        D3D12_VERTEX_BUFFER_VIEW {
            BufferLocation: self.buffer_location,
            SizeInBytes: self.size_in_bytes,
            StrideInBytes: self.stride_in_bytes,
        }
    }
}

impl Viewport {
    pub(crate) fn as_raw(&self) -> D3D12_VIEWPORT {
        D3D12_VIEWPORT {
            TopLeftX: self.x,
            TopLeftY: self.y,
            Width: self.width,
            Height: self.height,
            MinDepth: self.min_depth,
            MaxDepth: self.max_depth,
        }
    }
}
