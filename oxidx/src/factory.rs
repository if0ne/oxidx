use std::num::NonZeroIsize;

use windows::core::Interface;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dxgi::{
    IDXGIAdapter, IDXGIAdapter3, IDXGIFactory4, IDXGIFactory6, IDXGIFactory7,
};

use crate::dx::CommandQueue;
use crate::swapchain::{Output1, Swapchain1};
use crate::types::*;
use crate::{adapter::Adapter3, error::DxError};
use crate::{create_type, impl_interface};

create_type! { Factory4 wrap IDXGIFactory4 }
create_type! { Factory6 wrap IDXGIFactory6; decorator for Factory4 }
create_type! { Factory7 wrap IDXGIFactory7; decorator for Factory4, Factory6 }

impl_interface! {
    Factory4,
    Factory6,
    Factory7;

    /// Creates a swap chain that you can use to send Direct3D content into the DirectComposition API, to the Windows.UI.Xaml framework, or to Windows UI Library (WinUI) XAML, to compose in a window.
    ///
    /// For more information: [`IDXGIFactory2::CreateSwapChainForComposition method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-createswapchainforcomposition)
    pub fn create_swapchain_for_composition<'a>(
        &self,
        command_queue: impl AsRef<CommandQueue>,
        desc: &SwapchainDesc1,
        restrict_to_output: impl Into<Option<&'a Output1>>,
    ) -> Result<Swapchain1, DxError>
    {
        unsafe {
            let cq = command_queue.as_ref();
            let o = restrict_to_output.into();

            let desc = desc.0;

            let swapchain = if let Some(o) = o {
                self.0
                    .CreateSwapChainForComposition(&cq.0, &desc, &o.0)
                    .map_err(DxError::from)?
            } else {
                self.0
                    .CreateSwapChainForComposition(&cq.0, &desc, None)
                    .map_err(DxError::from)?
            };

            Ok(Swapchain1(swapchain))
        }
    }

    /// Creates a swap chain that is associated with an HWND handle to the output window for the swap chain.
    ///
    /// For more information: [`IDXGIFactory2::CreateSwapChainForHwnd method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-createswapchainforhwnd)
    pub fn create_swapchain_for_hwnd<'a>(
        &self,
        command_queue: impl AsRef<CommandQueue>,
        hwnd: NonZeroIsize,
        desc: &SwapchainDesc1,
        fullscreen_desc: Option<&SwapchainFullscreenDesc>,
        restrict_to_output: impl Into<Option<&'a Output1>>,
    ) -> Result<Swapchain1, DxError>
    {
        unsafe {
            let cq = command_queue.as_ref();
            let o = restrict_to_output.into();

            let desc = desc.0;
            let fullscreen_desc = fullscreen_desc.as_ref().map(|f| &f.0 as *const _);

            let swapchain = if let Some(o) = o {
                self.0
                    .CreateSwapChainForHwnd(&cq.0, HWND(hwnd.get() as *mut _), &desc, fullscreen_desc, &o.0)
                    .map_err(DxError::from)?
            } else {
                self.0
                    .CreateSwapChainForHwnd(&cq.0, HWND(hwnd.get() as *mut _), &desc, fullscreen_desc, None)
                    .map_err(DxError::from)?
            };

            Ok(Swapchain1(swapchain))
        }
    }

    /// Enumerates the adapters (video cards).
    ///
    /// For more information: [`IDXGIFactory::EnumAdapters method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-enumadapters)
    pub fn enum_adapters(&self, index: u32) -> Result<Adapter3, DxError> {
        unsafe {
            let adapter = self.0
                .EnumAdapters1(index)
                .map_err(DxError::from)?;

            let adapter = adapter
                .cast::<IDXGIAdapter3>()
                .map_err(|_| DxError::Cast("IUnknown", "IAdapter3"))?;

            Ok(Adapter3(adapter))
        }
    }

    /// Provides an adapter which can be provided to D3D12CreateDevice to use the WARP renderer.
    ///
    /// For more information: [`IDXGIFactory4::EnumWarpAdapter method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_4/nf-dxgi1_4-idxgifactory4-enumwarpadapter)
    pub fn enum_warp_adapters(&self) -> Result<Adapter3, DxError> {
        unsafe {
            let adapter = self.0
                .EnumWarpAdapter::<IDXGIAdapter>()
                .map_err(DxError::from)?;

            let adapter = adapter
                .cast::<IDXGIAdapter3>()
                .map_err(|_| DxError::Cast("IUnknown", "IAdapter3"))?;

            Ok(Adapter3(adapter))
        }
    }

    /// Allows DXGI to monitor an application's message queue for the alt-enter key sequence (which causes the application to switch from windowed to full screen or vice versa).
    ///
    /// For more information: [`IDXGIFactory::MakeWindowAssociation method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-makewindowassociation)
    pub fn make_window_association(&self, hwnd: NonZeroIsize, flags: WindowAssociationFlags) -> Result<(), DxError> {
        unsafe {
            self.0.MakeWindowAssociation(HWND(hwnd.get() as *mut _), flags.as_raw()).map_err(DxError::from)?;

            Ok(())
        }
    }
}

impl_interface! {
    Factory6,
    Factory7;

    /// Enumerates graphics adapters based on a given GPU preference.
    ///
    /// For more information: [`IDXGIFactory::EnumAdapterByGpuPreference method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_6/nf-dxgi1_6-idxgifactory6-enumadapterbygpupreference)
    pub fn enum_adapters_by_gpu_preference(&self, adapter: u32, preference: GpuPreference) -> Result<Adapter3, DxError> {
        unsafe {
            self.0.EnumAdapterByGpuPreference::<IDXGIAdapter3>(adapter, preference.as_raw())
                .map(Adapter3)
                .map_err(DxError::from)
        }
    }
}
