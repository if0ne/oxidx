use windows::core::{Interface, Param};
use windows::Win32::Graphics::Dxgi::{
    IDXGIOutput, IDXGISwapChain1, IDXGISwapChain2, IDXGISwapChain3,
};

use crate::error::DxError;
use crate::resources::IResource;
use crate::types::PresentFlags;
use crate::{create_type, impl_trait, HasInterface};

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

pub trait IOutput: for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<IDXGIOutput>> {}

create_type! { Output wrap IDXGIOutput }

impl_trait! {
    impl IOutput =>
    Output;
}
