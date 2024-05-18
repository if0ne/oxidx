use std::num::NonZeroIsize;

use windows::core::Interface;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory2, IDXGIAdapter, IDXGIAdapter3, IDXGIFactory4, IDXGIFactory6, IDXGIFactory7,
    DXGI_CREATE_FACTORY_DEBUG,
};

use crate::command_queue::CommandQueueInterface;
use crate::swapchain::{OutputInterface, Swapchain1, SwapchainDesc, SwapchainFullscreenDesc};
use crate::{adapter::Adapter3, error::DxError};
use crate::{create_type, implement_fns, HasInterface};

#[allow(dead_code)]
pub trait FactoryInterface: HasInterface {}

create_type! { FactoryInterface => Factory4 wrap IDXGIFactory4; decorator for }
create_type! { FactoryInterface => Factory6 wrap IDXGIFactory6; decorator for Factory4 }
create_type! { FactoryInterface => Factory7 wrap IDXGIFactory7; decorator for Factory4, Factory6 }

implement_fns! {
    Factory4,
    Factory6,
    Factory7;

    pub fn enum_adapters(&self, index: usize) -> Result<Adapter3, DxError> {
        let adapter = unsafe {
            self.0
                .EnumAdapters1(index as u32)
                .map_err(|_| DxError::NotFoundAdaptersError)?
        }
        .cast::<IDXGIAdapter3>()
        .expect("IDXGIFactory4 should support IDXGIAdapter3");
    
        Ok(Adapter3::new(adapter))
    }
    
    pub fn enum_warp_adapters(&self) -> Result<Adapter3, DxError> {
        let adapter = unsafe {
            self.0
                .EnumWarpAdapter::<IDXGIAdapter>()
                .map_err(|_| DxError::NotFoundAdaptersError)?
        }
        .cast::<IDXGIAdapter3>()
        .expect("IDXGIFactory4 should support IDXGIAdapter3");
    
        Ok(Adapter3::new(adapter))
    }
    
    pub fn create_swapchain_for_hwnd<CQ, RTO, O>(
        &self,
        command_queue: &CQ,
        hwnd: NonZeroIsize,
        desc: &SwapchainDesc,
        fullscreen_desc: Option<&SwapchainFullscreenDesc>,
        restrict_to_output: Option<O>,
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
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct FactoryCreationFlags: u32 {
        const Debug = DXGI_CREATE_FACTORY_DEBUG;
    }
}

pub struct Entry;

impl Entry {
    pub fn create_factory4(&self, flags: FactoryCreationFlags) -> Result<Factory4, DxError> {
        let inner = unsafe { CreateDXGIFactory2(flags.bits()) }
            .map_err(|_| DxError::FactoryCreationError)?;

        Ok(Factory4::new(inner))
    }

    pub fn create_factory6(&self, flags: FactoryCreationFlags) -> Result<Factory6, DxError> {
        let inner = unsafe { CreateDXGIFactory2(flags.bits()) }
            .map_err(|_| DxError::FactoryCreationError)?;

        Ok(Factory6::new(inner))
    }

    pub fn create_factory7(&self, flags: FactoryCreationFlags) -> Result<Factory7, DxError> {
        let inner = unsafe { CreateDXGIFactory2(flags.bits()) }
            .map_err(|_| DxError::FactoryCreationError)?;

        Ok(Factory7::new(inner))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_factory4_test() {
        let entry = Entry;
        let factory = entry.create_factory4(FactoryCreationFlags::Debug);

        assert!(factory.is_ok())
    }

    #[test]
    fn create_factory6_test() {
        let entry = Entry;
        let factory = entry.create_factory6(FactoryCreationFlags::Debug);

        assert!(factory.is_ok())
    }

    #[test]
    fn create_factory7_test() {
        let entry = Entry;
        let factory = entry.create_factory7(FactoryCreationFlags::Debug);

        assert!(factory.is_ok())
    }
}
