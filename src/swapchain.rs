use windows::core::Interface;
use windows::Win32::Graphics::Dxgi::{
    IDXGIOutput, IDXGISwapChain1, IDXGISwapChain2, IDXGISwapChain3,
    DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING,
    DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY, DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER,
    DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT, DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO,
    DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE, DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED,
    DXGI_SWAP_CHAIN_FLAG_NONPREROTATED, DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT,
    DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS,
    DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER, DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO,
};

use crate::misc::{AlphaMode, Scaling, ScalingMode, ScanlineOrdering, SwapEffect};
use crate::HasInterface;
use crate::{
    create_type,
    misc::{Format, FrameBufferUsage},
};

#[allow(dead_code)]
pub(crate) trait SwapchainInterface: HasInterface {}

create_type! { SwapchainInterface => Swapchain1 wrap IDXGISwapChain1; decorator for }
create_type! { SwapchainInterface => Swapchain2 wrap IDXGISwapChain2; decorator for Swapchain1 }
create_type! { SwapchainInterface => Swapchain3 wrap IDXGISwapChain3; decorator for Swapchain2, Swapchain1 }

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct SwapchainFlags: i32 {
        const NonPrerotated = DXGI_SWAP_CHAIN_FLAG_NONPREROTATED.0;
        const AllowModeSwitch = DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0;
        const GdiCompatible = DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE.0;
        const RestrictContent = DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT.0;
        const RestrictSharedResourceDriver = DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER.0;
        const DisplayOnly = DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY.0;
        const FrameLatencyWaitableObject = DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT.0;
        const ForegroundLayer = DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER.0;
        const FullscreenVideo = DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO.0;
        const YuvVideo = DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO.0;
        const Protected = DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED.0;
        const AllowTearing = DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING.0;
        const RestrictedToAllHolographicDisplays = DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS.0;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SampleDesc {
    pub count: u32,
    pub quality: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Rational {
    pub numerator: u32,
    pub denominator: u32,
}

#[derive(Debug, Clone)]
pub struct SwapchainDesc {
    pub width: u32,
    pub height: u32,
    pub format: Format,
    pub stereo: bool,
    pub sample_desc: SampleDesc,
    pub usage: FrameBufferUsage,
    pub buffer_count: u32,
    pub scaling: Scaling,
    pub swap_effect: SwapEffect,
    pub alpha_mode: AlphaMode,
    pub flags: SwapchainFlags,
}

#[derive(Debug, Clone)]
pub struct SwapchainFullscreenDesc {
    pub rational: Rational,
    pub scanline_ordering: ScanlineOrdering,
    pub scaling: ScalingMode,
    pub windowed: bool,
}

#[allow(dead_code)]
pub(crate) trait OutputInterface: HasInterface {}

create_type! { OutputInterface => Output wrap IDXGIOutput; decorator for }
