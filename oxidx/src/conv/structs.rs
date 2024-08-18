use windows::Win32::Graphics::Direct3D12::*;

use super::*;

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
