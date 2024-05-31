use windows::Win32::Graphics::{
    Direct3D::Fxc::{D3DCOMPILE_DEBUG, D3DCOMPILE_SKIP_OPTIMIZATION},
    Direct3D12::*,
    Dxgi::{Common::*, *},
};

pub const MIN_DEPTH: f32 = D3D12_MIN_DEPTH;
pub const MAX_DEPTH: f32 = D3D12_MAX_DEPTH;
pub const BARRIER_ALL_SUBRESOURCES: u32 = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;

pub const COMPILE_DEBUG: u32 = D3DCOMPILE_DEBUG;
pub const COMPILE_SKIP_OPT: u32 = D3DCOMPILE_SKIP_OPTIMIZATION;

#[derive(Debug, Default, Clone, Copy)]
#[repr(i32)]
pub enum Format {
    #[default]
    Less = 0,
    Bgra8Unorm = DXGI_FORMAT_B8G8R8A8_UNORM.0,
    Rgb32Float = DXGI_FORMAT_R32G32B32_FLOAT.0,
    Rgba32Float = DXGI_FORMAT_R32G32B32A32_FLOAT.0,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct FrameBufferUsage: u32 {
        const BackBuffer = DXGI_USAGE_BACK_BUFFER.0;
        const ReadOnly = DXGI_USAGE_READ_ONLY.0;
        const RenderTargetOutput = DXGI_USAGE_RENDER_TARGET_OUTPUT.0;
        const ShaderInput = DXGI_USAGE_SHADER_INPUT.0;
        const Shared = DXGI_USAGE_SHARED.0;
        const UnorderedAccess = DXGI_USAGE_UNORDERED_ACCESS.0;
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(i32)]
pub enum AlphaMode {
    #[default]
    Unspecified = DXGI_ALPHA_MODE_UNSPECIFIED.0,
    Premultiplied = DXGI_ALPHA_MODE_PREMULTIPLIED.0,
    Straight = DXGI_ALPHA_MODE_STRAIGHT.0,
    Ignore = DXGI_ALPHA_MODE_IGNORE.0,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(i32)]
pub enum Scaling {
    #[default]
    Stretch = DXGI_SCALING_STRETCH.0,
    None = DXGI_SCALING_NONE.0,
    AspectRatioStretch = DXGI_SCALING_ASPECT_RATIO_STRETCH.0,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(i32)]
pub enum ScalingMode {
    #[default]
    Unspecified = DXGI_MODE_SCALING_UNSPECIFIED.0,
    Centered = DXGI_MODE_SCALING_CENTERED.0,
    Stretched = DXGI_MODE_SCALING_STRETCHED.0,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(i32)]
pub enum ScanlineOrdering {
    #[default]
    Unspecified = DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED.0,
    Progressive = DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE.0,
    UpperFieldFirst = DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST.0,
    LowerFieldFirst = DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST.0,
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(i32)]
pub enum SwapEffect {
    #[default]
    Discard = DXGI_SWAP_EFFECT_DISCARD.0,
    Sequential = DXGI_SWAP_EFFECT_SEQUENTIAL.0,
    FlipSequential = DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL.0,
    FlipDiscard = DXGI_SWAP_EFFECT_FLIP_DISCARD.0,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct PresentFlags: u32 {
        const DoNotSequence = DXGI_PRESENT_DO_NOT_SEQUENCE;
        const Test = DXGI_PRESENT_TEST;
        const Restart = DXGI_PRESENT_RESTART;
        const DoNotWait = DXGI_PRESENT_DO_NOT_WAIT;
        const RestrictToOutput = DXGI_PRESENT_RESTRICT_TO_OUTPUT;
        const StereoPreferRight = DXGI_PRESENT_STEREO_PREFER_RIGHT;
        const StereoTemporaryMono = DXGI_PRESENT_STEREO_TEMPORARY_MONO;
        const UseDuration = DXGI_PRESENT_USE_DURATION;
        const AllowTearing = DXGI_PRESENT_ALLOW_TEARING;
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(i32)]
pub enum CommandListType {
    #[default]
    Direct = D3D12_COMMAND_LIST_TYPE_DIRECT.0,
    Copy = D3D12_COMMAND_LIST_TYPE_COPY.0,
    Compute = D3D12_COMMAND_LIST_TYPE_COMPUTE.0,
}

#[derive(Debug, Clone)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl Viewport {
    #[inline]
    pub fn from_position_and_size(
        position: impl Into<(f32, f32)>,
        size: impl Into<(f32, f32)>,
    ) -> Self {
        let (width, height) = size.into();
        let (x, y) = position.into();

        Self {
            x,
            y,
            width,
            height,
            min_depth: MIN_DEPTH,
            max_depth: MAX_DEPTH,
        }
    }

    #[inline]
    pub fn from_size(size: impl Into<(f32, f32)>) -> Self {
        Self::from_position_and_size((0.0, 0.0), size)
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    #[inline]
    pub fn from_size(size: impl Into<(i32, i32)>) -> Self {
        let (width, height) = size.into();

        Self {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        }
    }
}
