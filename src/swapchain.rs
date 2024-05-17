use windows::core::Interface;
use windows::Win32::Graphics::Dxgi::{IDXGISwapChain1, IDXGISwapChain2, IDXGISwapChain3};

use crate::misc::{AlphaMode, Scaling, SwapEffect};
use crate::{
    create_type,
    misc::{Format, FrameBufferUsage},
};

create_type! { Swapchain1, IDXGISwapChain1; }
create_type! { Swapchain2, IDXGISwapChain2; Swapchain1 }
create_type! { Swapchain3, IDXGISwapChain3; Swapchain2, Swapchain1 }

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct SwapchainFlags: u32 {

    }
}

#[derive(Debug, Clone, Copy)]
pub struct SampleDesc {
    pub count: u32,
    pub quality: u32,
}

#[derive(Debug, Clone, Copy)]
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
