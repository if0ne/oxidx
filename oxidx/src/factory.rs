use std::num::NonZeroIsize;

use windows::core::Interface;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dxgi::{
    IDXGIAdapter, IDXGIAdapter3, IDXGIFactory4, IDXGIFactory6, IDXGIFactory7,
};

use crate::command_queue::ICommandQueue;
use crate::swapchain::{IOutput1, Swapchain1};
use crate::types::*;
use crate::{adapter::Adapter3, error::DxError};
use crate::{create_type, impl_trait, HasInterface};

/// Enables creating Microsoft DirectX Graphics Infrastructure (DXGI) objects.
///
/// For more information: [`IDXGIFactory4 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_4/nn-dxgi1_4-idxgifactory4)
pub trait IFactory4: HasInterface<Raw: Interface> {
    /// Creates a swap chain that you can use to send Direct3D content into the DirectComposition API, to the Windows.UI.Xaml framework, or to Windows UI Library (WinUI) XAML, to compose in a window.
    ///
    /// For more information: [`IDXGIFactory2::CreateSwapChainForComposition method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-createswapchainforcomposition)
    fn create_swapchain_for_composition<CQ>(
        &self,
        command_queue: &CQ,
        desc: &SwapchainDesc1,
        restrict_to_output: Option<&impl IOutput1>,
    ) -> Result<Swapchain1, DxError>
    where
        CQ: ICommandQueue;

    /// Creates a swap chain that is associated with an HWND handle to the output window for the swap chain.
    ///
    /// For more information: [`IDXGIFactory2::CreateSwapChainForHwnd method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-createswapchainforhwnd)
    fn create_swapchain_for_hwnd<CQ>(
        &self,
        command_queue: &CQ,
        hwnd: NonZeroIsize,
        desc: &SwapchainDesc1,
        fullscreen_desc: Option<&SwapchainFullscreenDesc>,
        restrict_to_output: Option<&impl IOutput1>,
    ) -> Result<Swapchain1, DxError>
    where
        CQ: ICommandQueue;

    /// Enumerates the adapters (video cards).
    ///
    /// For more information: [`IDXGIFactory::EnumAdapters method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-enumadapters)
    fn enum_adapters(&self, index: usize) -> Result<Adapter3, DxError>;

    /// Provides an adapter which can be provided to D3D12CreateDevice to use the WARP renderer.
    ///
    /// For more information: [`IDXGIFactory4::EnumWarpAdapter method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_4/nf-dxgi1_4-idxgifactory4-enumwarpadapter)
    fn enum_warp_adapters(&self) -> Result<Adapter3, DxError>;

    /// Allows DXGI to monitor an application's message queue for the alt-enter key sequence (which causes the application to switch from windowed to full screen or vice versa).
    ///
    /// For more information: [`IDXGIFactory::MakeWindowAssociation method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-makewindowassociation)
    fn make_window_association(
        &self,
        hwnd: NonZeroIsize,
        flags: WindowAssociationFlags,
    ) -> Result<(), DxError>;
}

pub trait IFactory6 {
    /// Enumerates graphics adapters based on a given GPU preference.
    ///
    /// For more information: [`IDXGIFactory::EnumAdapterByGpuPreference method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_6/nf-dxgi1_6-idxgifactory6-enumadapterbygpupreference)
    fn enum_adapters_by_gpu_preference(
        &self,
        adapter: u32,
        preference: GpuPreference,
    ) -> Result<Adapter3, DxError>;
}

create_type! { Factory4 wrap IDXGIFactory4 }
create_type! { Factory6 wrap IDXGIFactory6; decorator for Factory4 }
create_type! { Factory7 wrap IDXGIFactory7; decorator for Factory4, Factory6 }

impl_trait! {
    impl IFactory4 =>
    Factory4,
    Factory6,
    Factory7;

    fn create_swapchain_for_hwnd<CQ>(
        &self,
        command_queue: &CQ,
        hwnd: NonZeroIsize,
        desc: &SwapchainDesc1,
        fullscreen_desc: Option<&SwapchainFullscreenDesc>,
        restrict_to_output: Option<&impl IOutput1>,
    ) -> Result<Swapchain1, DxError>
    where
        CQ: ICommandQueue
    {
        unsafe {
            let cq = command_queue.as_raw_ref();
            let o = restrict_to_output.as_ref().map(|o| o.as_raw_ref());

            let desc = desc.0;
            let fullscreen_desc = fullscreen_desc.as_ref().map(|f| &f.0 as *const _);

            let swapchain = if let Some(o) = o {
                self.0
                    .CreateSwapChainForHwnd(cq, HWND(hwnd.get()), &desc, fullscreen_desc, o)
                    .map_err(DxError::from)?
            } else {
                self.0
                    .CreateSwapChainForHwnd(cq, HWND(hwnd.get()), &desc, fullscreen_desc, None)
                    .map_err(DxError::from)?
            };

            Ok(Swapchain1::new(swapchain))
        }
    }

    fn enum_adapters(&self, index: usize) -> Result<Adapter3, DxError> {
        unsafe {
            let adapter = self.0
                .EnumAdapters1(index as u32)
                .map_err(DxError::from)?;

            let adapter = adapter
                .cast::<IDXGIAdapter3>()
                .map_err(|_| DxError::Cast("IUnknown", "IAdapter3"))?;

            Ok(Adapter3::new(adapter))
        }
    }

    fn enum_warp_adapters(&self) -> Result<Adapter3, DxError> {
        unsafe {
            let adapter = self.0
                .EnumWarpAdapter::<IDXGIAdapter>()
                .map_err(DxError::from)?;

            let adapter = adapter
                .cast::<IDXGIAdapter3>()
                .map_err(|_| DxError::Cast("IUnknown", "IAdapter3"))?;

            Ok(Adapter3::new(adapter))
        }
    }

    fn create_swapchain_for_composition<CQ>(
        &self,
        command_queue: &CQ,
        desc: &SwapchainDesc1,
        restrict_to_output: Option<&impl IOutput1>,
    ) -> Result<Swapchain1, DxError>
    where
        CQ: ICommandQueue
    {
        unsafe {
            let cq = command_queue.as_raw_ref();
            let o = restrict_to_output.as_ref().map(|o| o.as_raw_ref());

            let desc = desc.0;

            let swapchain = if let Some(o) = o {
                self.0
                    .CreateSwapChainForComposition(cq, &desc, o)
                    .map_err(DxError::from)?
            } else {
                self.0
                    .CreateSwapChainForComposition(cq, &desc, None)
                    .map_err(DxError::from)?
            };

            Ok(Swapchain1::new(swapchain))
        }
    }

    fn make_window_association(&self, hwnd: NonZeroIsize, flags: WindowAssociationFlags) -> Result<(), DxError> {
        unsafe {
            self.0.MakeWindowAssociation(HWND(hwnd.get()), flags.bits()).map_err(DxError::from)?;

            Ok(())
        }
    }
}

impl_trait! {
    impl IFactory6 =>
    Factory6,
    Factory7;

    fn enum_adapters_by_gpu_preference(&self, adapter: u32, preference: GpuPreference) -> Result<Adapter3, DxError> {
        unsafe {
            self.0.EnumAdapterByGpuPreference::<IDXGIAdapter3>(adapter, preference.as_raw())
                .map(Adapter3::new)
                .map_err(DxError::from)
        }
    }
}
