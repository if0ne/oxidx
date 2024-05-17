use windows::Win32::Graphics::Dxgi::{
    DXGI_USAGE_BACK_BUFFER, DXGI_USAGE_READ_ONLY, DXGI_USAGE_RENDER_TARGET_OUTPUT,
    DXGI_USAGE_SHADER_INPUT, DXGI_USAGE_SHARED, DXGI_USAGE_UNORDERED_ACCESS,
};

#[derive(Debug, Clone, Copy)]
pub enum Format {}

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
#[repr(u32)]
pub enum AlphaMode {
    #[default]
    Unspecified = 0,
    Premultiplied = 1,
    Straight = 2,
    Ignore = 3,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Scaling {
    #[default]
    Stretch = 0,
    None = 1,
    AspectRatioStretch = 2,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum SwapEffect {
    #[default]
    Discard = 0,
    Sequential = 1,
    FlipSequential = 3,
    FlipDiscard = 4,
}
