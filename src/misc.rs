use windows::Win32::Graphics::Dxgi::{
    Common::{
        DXGI_ALPHA_MODE_IGNORE, DXGI_ALPHA_MODE_PREMULTIPLIED, DXGI_ALPHA_MODE_STRAIGHT,
        DXGI_ALPHA_MODE_UNSPECIFIED, DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_MODE_SCALING_CENTERED,
        DXGI_MODE_SCALING_STRETCHED, DXGI_MODE_SCALING_UNSPECIFIED,
        DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST, DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE,
        DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST,
    },
    DXGI_PRESENT_ALLOW_TEARING, DXGI_PRESENT_DO_NOT_SEQUENCE, DXGI_PRESENT_DO_NOT_WAIT,
    DXGI_PRESENT_RESTART, DXGI_PRESENT_RESTRICT_TO_OUTPUT, DXGI_PRESENT_STEREO_PREFER_RIGHT,
    DXGI_PRESENT_STEREO_TEMPORARY_MONO, DXGI_PRESENT_TEST, DXGI_PRESENT_USE_DURATION,
    DXGI_SCALING_ASPECT_RATIO_STRETCH, DXGI_SCALING_NONE, DXGI_SCALING_STRETCH,
    DXGI_SWAP_EFFECT_DISCARD, DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
    DXGI_SWAP_EFFECT_SEQUENTIAL, DXGI_USAGE_BACK_BUFFER, DXGI_USAGE_READ_ONLY,
    DXGI_USAGE_RENDER_TARGET_OUTPUT, DXGI_USAGE_SHADER_INPUT, DXGI_USAGE_SHARED,
    DXGI_USAGE_UNORDERED_ACCESS,
};

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Format {
    Bgra8Unorm = DXGI_FORMAT_B8G8R8A8_UNORM.0,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
