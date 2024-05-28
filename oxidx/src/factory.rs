use std::num::NonZeroIsize;

use windows::core::Interface;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D::{
    D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_12_0, D3D_FEATURE_LEVEL_12_1,
    D3D_FEATURE_LEVEL_12_2,
};
use windows::Win32::Graphics::Direct3D12::{D3D12CreateDevice, D3D12GetDebugInterface};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory2, IDXGIAdapter, IDXGIAdapter3, IDXGIFactory4, IDXGIFactory6, IDXGIFactory7,
    DXGI_CREATE_FACTORY_DEBUG, DXGI_MWA_NO_ALT_ENTER, DXGI_MWA_NO_PRINT_SCREEN,
    DXGI_MWA_NO_WINDOW_CHANGES,
};

use crate::adapter::AdapterInterface3;
use crate::command_queue::CommandQueueInterface;
use crate::debug::DebugInterface;
use crate::device::DeviceInterface;
use crate::swapchain::{OutputInterface, Swapchain1, SwapchainDesc, SwapchainFullscreenDesc};
use crate::{adapter::Adapter3, error::DxError};
use crate::{create_type, impl_trait, HasInterface};

pub trait FactoryInterface4: HasInterface<Raw: Interface> {
    fn enum_adapters(&self, index: usize) -> Result<Adapter3, DxError>;
    fn enum_warp_adapters(&self) -> Result<Adapter3, DxError>;
    fn create_swapchain_for_hwnd<CQ, O>(
        &self,
        command_queue: &CQ,
        hwnd: NonZeroIsize,
        desc: &SwapchainDesc,
        fullscreen_desc: Option<&SwapchainFullscreenDesc>,
        restrict_to_output: Option<&O>,
    ) -> Result<Swapchain1, DxError>
    where
        CQ: CommandQueueInterface,
        O: OutputInterface;

    fn create_swapchain_for_composition<CQ, O>(
        &self,
        command_queue: &CQ,
        desc: &SwapchainDesc,
        restrict_to_output: Option<&O>,
    ) -> Result<Swapchain1, DxError>
    where
        CQ: CommandQueueInterface,
        O: OutputInterface;

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
                .map_err(|_| DxError::NotFoundAdaptersError)?
        }
        .cast::<IDXGIAdapter3>()
        .expect("IDXGIFactory4 should support IDXGIAdapter3");

        Ok(Adapter3::new(adapter))
    }

    fn enum_warp_adapters(&self) -> Result<Adapter3, DxError> {
        let adapter = unsafe {
            self.0
                .EnumWarpAdapter::<IDXGIAdapter>()
                .map_err(|_| DxError::NotFoundAdaptersError)?
        }
        .cast::<IDXGIAdapter3>()
        .expect("IDXGIFactory4 should support IDXGIAdapter3");

        Ok(Adapter3::new(adapter))
    }

    fn create_swapchain_for_hwnd<CQ, O>(
        &self,
        command_queue: &CQ,
        hwnd: NonZeroIsize,
        desc: &SwapchainDesc,
        fullscreen_desc: Option<&SwapchainFullscreenDesc>,
        restrict_to_output: Option<&O>,
    ) -> Result<Swapchain1, DxError>
    where
        CQ: CommandQueueInterface,
        O: OutputInterface,
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
                    .map_err(|_| DxError::SwapchainCreationError)?
            } else {
                self.0
                    .CreateSwapChainForHwnd(cq, HWND(hwnd.get()), &desc, fullscreen_desc, None)
                    .map_err(|_| DxError::SwapchainCreationError)?
            }
        };

        Ok(Swapchain1::new(swapchain))
    }

    fn create_swapchain_for_composition<CQ, O>(
        &self,
        command_queue: &CQ,
        desc: &SwapchainDesc,
        restrict_to_output: Option<&O>,
    ) -> Result<Swapchain1, DxError>
    where
        CQ: CommandQueueInterface,
        O: OutputInterface
    {
        let cq = command_queue.as_raw_ref();
        let o = restrict_to_output.as_ref().map(|o| o.as_raw_ref());

        let desc = desc.as_raw();

        let swapchain = unsafe {
            if let Some(o) = o {
                self.0
                    .CreateSwapChainForComposition(cq, &desc, o)
                    .map_err(|_| DxError::SwapchainCreationError)?
            } else {
                self.0
                    .CreateSwapChainForComposition(cq, &desc, None)
                    .map_err(|_| DxError::SwapchainCreationError)?
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

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct FactoryCreationFlags: u32 {
        const Debug = DXGI_CREATE_FACTORY_DEBUG;
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum FeatureLevel {
    Level11 = D3D_FEATURE_LEVEL_11_0.0,
    Level11_1 = D3D_FEATURE_LEVEL_11_1.0,
    Level12 = D3D_FEATURE_LEVEL_12_0.0,
    Level12_1 = D3D_FEATURE_LEVEL_12_1.0,
    Level12_2 = D3D_FEATURE_LEVEL_12_2.0,
}

pub struct Entry;

impl Entry {
    pub fn create_factory<F: FactoryInterface4>(
        &self,
        flags: FactoryCreationFlags,
    ) -> Result<F, DxError> {
        let inner: F::Raw = unsafe { CreateDXGIFactory2(flags.bits()) }
            .map_err(|_| DxError::FactoryCreationError)?;

        Ok(F::new(inner))
    }

    pub fn create_device<A: AdapterInterface3, D: DeviceInterface>(
        &self,
        adapter: &A,
        feature_level: FeatureLevel,
    ) -> Result<D, DxError> {
        let mut inner: Option<D::Raw> = None;
        unsafe {
            D3D12CreateDevice(adapter.as_raw_ref(), feature_level.as_raw(), &mut inner)
                .map_err(|_| DxError::Dummy)?
        };
        let inner = inner.unwrap();

        Ok(D::new(inner))
    }

    pub fn create_debug<D: DebugInterface>(&self) -> Result<D, DxError> {
        let mut inner: Option<D::Raw> = None;

        unsafe { D3D12GetDebugInterface(&mut inner).map_err(|_| DxError::Dummy)? };
        let inner = inner.unwrap();

        Ok(D::new(inner))
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct WindowAssociationFlags: u32 {
        const NoWindowChanges = DXGI_MWA_NO_WINDOW_CHANGES;
        const NoAltEnter = DXGI_MWA_NO_ALT_ENTER;
        const NoPrintScreen = DXGI_MWA_NO_PRINT_SCREEN;
    }
}

#[cfg(test)]
mod test {
    use crate::device::Device;

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
