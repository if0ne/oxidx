use windows::core::{Interface, Param};
use windows::Win32::Graphics::Dxgi::{
    IDXGIOutput, IDXGIOutput1, IDXGISwapChain1, IDXGISwapChain2, IDXGISwapChain3,
};

use crate::error::DxError;
use crate::resources::IResource;
use crate::types::*;
use crate::{create_type, impl_trait, HasInterface};

/// Provides presentation capabilities that are enhanced from IDXGISwapChain.
/// These presentation capabilities consist of specifying dirty rectangles and scroll rectangle to optimize the presentation.
///
/// For more information: [`IDXGISwapChain1 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nn-dxgi1_2-idxgiswapchain1)
pub trait ISwapchain1: HasInterface {
    /// Accesses one of the swap-chain's back buffers.
    ///
    /// For more information: [`IDXGISwapChain::GetBuffer method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-getbuffer)
    fn get_buffer<R: IResource>(&self, buffer: usize) -> Result<R, DxError>;

    /// Presents a rendered image to the user.
    ///
    /// For more information: [`IDXGISwapChain::Present method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-present)
    fn present(&self, interval: u32, flags: PresentFlags) -> Result<(), DxError>;

    /// Changes the swap chain's back buffer size, format, and number of buffers. This should be called when the application window is resized.
    ///
    /// For more information: [`IDXGISwapChain::ResizeBuffers method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-resizebuffers)
    fn resize_buffers(
        &self,
        buffer_count: usize,
        width: u32,
        height: u32,
        new_format: Format,
        flags: SwapchainFlags,
    ) -> Result<(), DxError>;
}

/// Extends [`ISwapchain1`] with methods to support swap back buffer scaling and lower-latency swap chains.
///
/// For more information: [`IDXGISwapChain2 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nn-dxgi1_3-idxgiswapchain2)
pub trait ISwapchain2: ISwapchain1 {}

/// Extends [`ISwapchain2`] with methods to support getting the index of the swap chain's current back buffer and support for color space.
///
/// For more information: [`IDXGISwapChain3 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_4/nn-dxgi1_4-idxgiswapchain3)
pub trait ISwapchain3: ISwapchain2 {
    /// Gets the index of the swap chain's current back buffer.
    ///
    /// For more information: [`IDXGISwapChain3::GetCurrentBackBufferIndex method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_4/nf-dxgi1_4-idxgiswapchain3-getcurrentbackbufferindex)
    fn get_current_back_buffer_index(&self) -> u32;
}

create_type! {
    /// Provides presentation capabilities that are enhanced from IDXGISwapChain.
    /// These presentation capabilities consist of specifying dirty rectangles and scroll rectangle to optimize the presentation.
    ///
    /// For more information: [`IDXGISwapChain1 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nn-dxgi1_2-idxgiswapchain1)
    Swapchain1 wrap IDXGISwapChain1
}

create_type! {
    /// Extends [`ISwapchain1`] with methods to support swap back buffer scaling and lower-latency swap chains.
    ///
    /// For more information: [`IDXGISwapChain2 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nn-dxgi1_3-idxgiswapchain2)
    Swapchain2 wrap IDXGISwapChain2; decorator for Swapchain1
}

create_type! {
    Swapchain3 wrap IDXGISwapChain3; decorator for Swapchain2, Swapchain1
}

impl_trait! {
    impl ISwapchain1 =>
    Swapchain1,
    Swapchain2,
    Swapchain3;

    fn get_buffer<R: IResource>(&self, buffer: usize) -> Result<R, DxError> {
        unsafe {
            let buffer: R::Raw = self.0.GetBuffer(buffer as u32).map_err(DxError::from)?;

            Ok(R::new(buffer))
        }
    }

    fn present(&self, interval: u32, flags: PresentFlags) -> Result<(), DxError> {
        unsafe {
            self.0.Present(interval, flags.as_raw()).ok().map_err(DxError::from)
        }
    }

    fn resize_buffers(
        &self,
        buffer_count: usize,
        width: u32,
        height: u32,
        new_format: Format,
        flags: SwapchainFlags,
    ) -> Result<(), DxError> {
        unsafe {
            self.0.ResizeBuffers(
                buffer_count as u32,
                width,
                height,
                new_format.as_raw(),
                flags.as_raw()
            ).map_err(DxError::from)
        }
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

/// An [`IOutput`] interface represents an adapter output (such as a monitor).
///
/// For more information: [`IDXGIOutput interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgioutput)
pub trait IOutput1: for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<IDXGIOutput>> {
    /// Get a description of the output.
    ///
    /// For more information: [`IDXGIOutput::GetDesc method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgioutput-getdesc)
    fn get_desc(&self) -> Result<OutputDesc, DxError>;

    /// Gets the display modes that match the requested format and other input options.
    ///
    /// For more information: [`IDXGIOutput1::GetDisplayModeList1 method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgioutput1-getdisplaymodelist1)
    fn get_display_mode_list1(
        &self,
        format: Format,
        flags: EnumModeFlags,
    ) -> Result<Vec<ModeDesc1>, DxError>;
}

create_type! {
    /// An [`IOutput`] interface represents an adapter output (such as a monitor).
    ///
    /// For more information: [`IDXGIOutput interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nn-dxgi-idxgioutput)
    Output1 wrap IDXGIOutput1
}

impl_trait! {
    impl IOutput1 =>
    Output1;

    fn get_desc(&self) -> Result<OutputDesc, DxError> {
        unsafe {
            self.0.GetDesc()
                .map(|d| OutputDesc(d))
                .map_err(DxError::from)
        }
    }

    fn get_display_mode_list1(&self, format: Format, flags: EnumModeFlags) -> Result<Vec<ModeDesc1>, DxError> {
        unsafe {
            let mut count = 0;
            self.0.GetDisplayModeList1(
                format.as_raw(),
                flags.as_raw(),
                &mut count,
                None
            ).map_err(DxError::from)?;

            let mut vec = vec![];
            vec.resize(count as usize, std::mem::zeroed());

            self.0.GetDisplayModeList1(
                format.as_raw(),
                flags.as_raw(),
                &mut count,
                Some(vec.as_mut_ptr() as *mut _)
            ).map_err(DxError::from)?;

            Ok(vec)
        }
    }
}
