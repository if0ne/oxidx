use windows::core::{Interface, Param};
use windows::Win32::Foundation::HANDLE;
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
    fn get_buffer<R: IResource>(&self, buffer: u32) -> Result<R, DxError>;

    /// Presents a rendered image to the user.
    ///
    /// For more information: [`IDXGISwapChain::Present method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-present)
    fn present(&self, interval: u32, flags: PresentFlags) -> Result<(), DxError>;

    /// Changes the swap chain's back buffer size, format, and number of buffers. This should be called when the application window is resized.
    ///
    /// For more information: [`IDXGISwapChain::ResizeBuffers method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiswapchain-resizebuffers)
    fn resize_buffers(
        &self,
        buffer_count: u32,
        width: u32,
        height: u32,
        new_format: Format,
        flags: SwapchainFlags,
    ) -> Result<(), DxError>;
}

#[derive(Clone, Copy, Debug)]
pub struct WaitableObject(pub(crate) HANDLE);

/// Extends [`ISwapchain1`] with methods to support swap back buffer scaling and lower-latency swap chains.
///
/// For more information: [`IDXGISwapChain2 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nn-dxgi1_3-idxgiswapchain2)
pub trait ISwapchain2: ISwapchain1 {
    /// Returns a waitable handle that signals when the DXGI adapter has finished presenting a new frame.
    ///
    /// For more information: [`IDXGISwapChain2::GetFrameLatencyWaitableObject method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-idxgiswapchain2-getframelatencywaitableobject)
    fn get_frame_latency_waitable_object(&self) -> WaitableObject;

    /// Gets the transform matrix that will be applied to a composition swap chain upon the next present.
    ///
    /// For more information: [`IDXGISwapChain2::GetMatrixTransform method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-idxgiswapchain2-getmatrixtransform)
    fn get_matirx_transform(&self) -> Result<[[f32; 2]; 3], DxError>;

    /// Gets the number of frames that the swap chain is allowed to queue for rendering.
    ///
    /// For more information: [`IDXGISwapChain2::GetMaximumFrameLatency method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-idxgiswapchain2-getmaximumframelatency)
    fn get_maximum_frame_latency(&self) -> Result<u32, DxError>;

    /// Gets the source region used for the swap chain.
    ///
    /// For more information: [`IDXGISwapChain2::GetSourceSize method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-idxgiswapchain2-getsourcesize)
    fn get_source_size(&self) -> Result<(u32, u32), DxError>;

    /// Sets the transform matrix that will be applied to a composition swap chain upon the next present.
    ///
    /// For more information: [`IDXGISwapChain2::SetMatrixTransform method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-idxgiswapchain2-setmatrixtransform)
    fn set_matrix_transform(&self, matrix: impl Into<[[f32; 2]; 3]>) -> Result<(), DxError>;

    /// Sets the number of frames that the swap chain is allowed to queue for rendering.
    ///
    /// For more information: [`IDXGISwapChain2::SetMaximumFrameLatency method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-idxgiswapchain2-setmaximumframelatency)
    fn set_maximum_frame_latency(&self, max_latency: u32) -> Result<(), DxError>;

    /// Sets the source region to be used for the swap chain.
    ///
    /// For more information: [`IDXGISwapChain2::SetSourceSize method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-idxgiswapchain2-setsourcesize)
    fn set_source_size(&self, width: u32, height: u32) -> Result<(), DxError>;
}

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

    fn get_buffer<R: IResource>(&self, buffer: u32) -> Result<R, DxError> {
        unsafe {
            let buffer: R::Raw = self.0.GetBuffer(buffer).map_err(DxError::from)?;

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
        buffer_count: u32,
        width: u32,
        height: u32,
        new_format: Format,
        flags: SwapchainFlags,
    ) -> Result<(), DxError> {
        unsafe {
            self.0.ResizeBuffers(
                buffer_count,
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

    fn get_frame_latency_waitable_object(&self) -> WaitableObject {
        let handle = unsafe {
            self.0.GetFrameLatencyWaitableObject()
        };

        WaitableObject(handle)
    }

    fn get_matirx_transform(&self) -> Result<[[f32; 2]; 3], DxError> {
        let mut matrix = Default::default();
        unsafe {
            self.0.GetMatrixTransform(&mut matrix)?;
        };

        Ok([
            [matrix._11, matrix._12],
            [matrix._21, matrix._22],
            [matrix._31, matrix._32]
        ])
    }

    fn get_maximum_frame_latency(&self) -> Result<u32, DxError> {
        unsafe { Ok(self.0.GetMaximumFrameLatency()?) }
    }

    fn get_source_size(&self) -> Result<(u32, u32), DxError> {
        let mut width = 0;
        let mut height = 0;

        unsafe {
            self.0.GetSourceSize(&mut width, &mut height)?;
        }

        Ok((width, height))
    }

    fn set_matrix_transform(&self, matrix: impl Into<[[f32; 2]; 3]>) -> Result<(), DxError> {
        let matrix = matrix.into();
        let matrix = windows::Win32::Graphics::Dxgi::DXGI_MATRIX_3X2_F {
            _11: matrix[0][0],
            _12: matrix[0][1],
            _21: matrix[1][0],
            _22: matrix[1][1],
            _31: matrix[2][0],
            _32: matrix[2][1]
        };

        unsafe {
            self.0.SetMatrixTransform(&matrix)?;
        }

        Ok(())
    }

    fn set_maximum_frame_latency(&self, max_latency: u32) -> Result<(), DxError> {
        unsafe {
            self.0.SetMaximumFrameLatency(max_latency)?;
        }

        Ok(())
    }

    fn set_source_size(&self, width: u32, height: u32) -> Result<(), DxError> {
        unsafe { self.0.SetSourceSize(width, height)?; }

        Ok(())
    }
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
                .map(OutputDesc)
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
