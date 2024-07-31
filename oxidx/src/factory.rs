use std::num::NonZeroIsize;

use windows::core::Interface;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D12::{D3D12CreateDevice, D3D12GetDebugInterface};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory2, IDXGIAdapter, IDXGIAdapter3, IDXGIFactory4, IDXGIFactory6, IDXGIFactory7,
};

use crate::adapter::IAdapter3;
use crate::command_queue::ICommandQueue;
use crate::debug::IDebug;
use crate::device::IDevice;
use crate::swapchain::{IOutput, Swapchain1};
use crate::types::{
    FactoryCreationFlags, FeatureLevel, SwapchainDesc, SwapchainFullscreenDesc,
    WindowAssociationFlags,
};
use crate::{adapter::Adapter3, error::DxError};
use crate::{create_type, impl_trait, HasInterface};

pub trait FactoryInterface4: HasInterface<Raw: Interface> {
    fn enum_adapters(&self, index: usize) -> Result<Adapter3, DxError>;
    fn enum_warp_adapters(&self) -> Result<Adapter3, DxError>;
    fn create_swapchain_for_hwnd<CQ>(
        &self,
        command_queue: &CQ,
        hwnd: NonZeroIsize,
        desc: &SwapchainDesc,
        fullscreen_desc: Option<&SwapchainFullscreenDesc>,
        restrict_to_output: Option<&impl IOutput>,
    ) -> Result<Swapchain1, DxError>
    where
        CQ: ICommandQueue;

    fn create_swapchain_for_composition<CQ>(
        &self,
        command_queue: &CQ,
        desc: &SwapchainDesc,
        restrict_to_output: Option<&impl IOutput>,
    ) -> Result<Swapchain1, DxError>
    where
        CQ: ICommandQueue;

    fn make_window_association(
        &self,
        hwnd: NonZeroIsize,
        flags: WindowAssociationFlags,
    ) -> Result<(), DxError>;
}

create_type! { Factory4 wrap IDXGIFactory4 }
create_type! { Factory6 wrap IDXGIFactory6; decorator for Factory4 }
create_type! { Factory7 wrap IDXGIFactory7; decorator for Factory4, Factory6 }

impl_trait! {
    impl FactoryInterface4 =>
    Factory4,
    Factory6,
    Factory7;

    fn enum_adapters(&self, index: usize) -> Result<Adapter3, DxError> {
        let adapter = unsafe {
            self.0
                .EnumAdapters1(index as u32)
                .map_err(DxError::from)?
        }
        .cast::<IDXGIAdapter3>()
        .expect("IDXGIFactory4 should support IDXGIAdapter3");

        Ok(Adapter3::new(adapter))
    }

    fn enum_warp_adapters(&self) -> Result<Adapter3, DxError> {
        let adapter = unsafe {
            self.0
                .EnumWarpAdapter::<IDXGIAdapter>()
                .map_err(DxError::from)?
        }
        .cast::<IDXGIAdapter3>()
        .expect("IDXGIFactory4 should support IDXGIAdapter3");

        Ok(Adapter3::new(adapter))
    }

    fn create_swapchain_for_hwnd<CQ>(
        &self,
        command_queue: &CQ,
        hwnd: NonZeroIsize,
        desc: &SwapchainDesc,
        fullscreen_desc: Option<&SwapchainFullscreenDesc>,
        restrict_to_output: Option<&impl IOutput>,
    ) -> Result<Swapchain1, DxError>
    where
        CQ: ICommandQueue
    {
        let cq = command_queue.as_raw_ref();
        let o = restrict_to_output.as_ref().map(|o| o.as_raw_ref());

        let desc = desc.as_raw();
        let fullscreen_desc = fullscreen_desc.map(|f| f.as_raw());
        let fullscreen_desc = fullscreen_desc.as_ref().map(|f| f as *const _);

        let swapchain = unsafe {
            if let Some(o) = o {
                self.0
                    .CreateSwapChainForHwnd(cq, HWND(hwnd.get()), &desc, fullscreen_desc, o)
                    .map_err(DxError::from)?
            } else {
                self.0
                    .CreateSwapChainForHwnd(cq, HWND(hwnd.get()), &desc, fullscreen_desc, None)
                    .map_err(DxError::from)?
            }
        };

        Ok(Swapchain1::new(swapchain))
    }

    fn create_swapchain_for_composition<CQ>(
        &self,
        command_queue: &CQ,
        desc: &SwapchainDesc,
        restrict_to_output: Option<&impl IOutput>,
    ) -> Result<Swapchain1, DxError>
    where
        CQ: ICommandQueue
    {
        let cq = command_queue.as_raw_ref();
        let o = restrict_to_output.as_ref().map(|o| o.as_raw_ref());

        let desc = desc.as_raw();

        let swapchain = unsafe {
            if let Some(o) = o {
                self.0
                    .CreateSwapChainForComposition(cq, &desc, o)
                    .map_err(DxError::from)?
            } else {
                self.0
                    .CreateSwapChainForComposition(cq, &desc, None)
                    .map_err(DxError::from)?
            }
        };

        Ok(Swapchain1::new(swapchain))
    }

    fn make_window_association(&self, hwnd: NonZeroIsize, flags: WindowAssociationFlags) -> Result<(), DxError> {
        unsafe {
            self.0.MakeWindowAssociation(HWND(hwnd.get()), flags.bits()).map_err(|_| DxError::Dummy)?;
        }

        Ok(())
    }
}

pub struct Entry;

impl Entry {
    pub fn create_factory<F: FactoryInterface4>(
        &self,
        flags: FactoryCreationFlags,
    ) -> Result<F, DxError> {
        let inner: F::Raw = unsafe { CreateDXGIFactory2(flags.bits()) }.map_err(DxError::from)?;

        Ok(F::new(inner))
    }

    pub fn create_device<A: IAdapter3, D: IDevice>(
        &self,
        adapter: &A,
        feature_level: FeatureLevel,
    ) -> Result<D, DxError> {
        let mut inner: Option<D::Raw> = None;
        unsafe {
            D3D12CreateDevice(adapter.as_raw_ref(), feature_level.as_raw(), &mut inner)
                .map_err(DxError::from)?
        };
        let inner = inner.unwrap();

        Ok(D::new(inner))
    }

    pub fn create_debug<D: IDebug>(&self) -> Result<D, DxError> {
        let mut inner: Option<D::Raw> = None;

        unsafe { D3D12GetDebugInterface(&mut inner).map_err(DxError::from)? };
        let inner = inner.unwrap();

        Ok(D::new(inner))
    }
}

#[cfg(test)]
mod test {
    use crate::{device::Device, types::FactoryCreationFlags};

    use super::*;

    #[test]
    fn create_factory4_test() {
        let entry = Entry;
        let factory = entry.create_factory::<Factory4>(FactoryCreationFlags::Debug);

        assert!(factory.is_ok())
    }

    #[test]
    fn create_factory6_test() {
        let entry = Entry;
        let factory = entry.create_factory::<Factory6>(FactoryCreationFlags::Debug);

        assert!(factory.is_ok())
    }

    #[test]
    fn create_factory7_test() {
        let entry = Entry;
        let factory = entry.create_factory::<Factory7>(FactoryCreationFlags::Debug);

        assert!(factory.is_ok())
    }

    #[test]
    fn create_device_test() {
        let entry = Entry;

        let factory = entry.create_factory::<Factory4>(FactoryCreationFlags::Debug);
        assert!(factory.is_ok());
        let factory = factory.unwrap();

        let adapter = factory.enum_adapters(0);
        assert!(adapter.is_ok());
        let adapter = adapter.unwrap();

        let device = entry.create_device::<_, Device>(&adapter, FeatureLevel::Level11);
        assert!(device.is_ok());
    }
}
