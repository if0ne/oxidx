use windows::core::{Interface, Param};
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

use crate::error::DxError;
use crate::resources::IResource;
use crate::types::{
    AlphaMode, PresentFlags, SampleDesc, Scaling, ScalingMode, ScanlineOrdering, SwapEffect,
};
use crate::{
    create_type,
    types::{Format, FrameBufferUsage},
};
use crate::{impl_trait, HasInterface};

pub trait ISwapchain1: HasInterface {
    fn present(&self, interval: u32, flags: PresentFlags) -> Result<(), DxError>;
    fn get_buffer<R: IResource>(&self, buffer: u32) -> Result<R, DxError>;
}

pub trait ISwapchain2: ISwapchain1 {}

pub trait ISwapchain3: ISwapchain2 {
    fn get_current_back_buffer_index(&self) -> u32;
}

create_type! { Swapchain1 wrap IDXGISwapChain1 }
create_type! { Swapchain2 wrap IDXGISwapChain2; decorator for Swapchain1 }
create_type! { Swapchain3 wrap IDXGISwapChain3; decorator for Swapchain2, Swapchain1 }

impl_trait! {
    impl ISwapchain1 =>
    Swapchain1,
    Swapchain2,
    Swapchain3;

    fn present(&self, interval: u32, flags: PresentFlags) -> Result<(), DxError> {
        let res = unsafe {
            self.0.Present(interval, flags.bits())
        };

        res.ok().map_err(DxError::from)
    }

    fn get_buffer<R: IResource>(&self, buffer: u32) -> Result<R, DxError> {
        let buffer: R::Raw = unsafe {
            self.0.GetBuffer(buffer).map_err(|_| DxError::Dummy)?
        };

        Ok(R::new(buffer))
    }
}

impl_trait! {
    impl ISwapchain2 =>
    Swapchain2,
    Swapchain3;
}

impl_trait! {
    impl ISwapchain3 =>
    Swapchain3;

    fn get_current_back_buffer_index(&self) -> u32 {
        unsafe {
            self.0.GetCurrentBackBufferIndex()
        }
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
pub struct Rational {
    pub numerator: u32,
    pub denominator: u32,
}

#[derive(Debug, Default, Clone)]
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

pub trait IOutput:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<IDXGIOutput>>
{
}

create_type! { Output wrap IDXGIOutput }

impl_trait! {
    impl IOutput =>
    Output;
}
