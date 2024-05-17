use windows::core::Interface;
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory2, IDXGIAdapter, IDXGIAdapter3, IDXGIFactory4, IDXGIFactory6, IDXGIFactory7,
    DXGI_CREATE_FACTORY_DEBUG,
};

use crate::command_queue::CommandQueue;
use crate::swapchain::{Swapchain1, SwapchainDesc};
use crate::{adapter::Adapter3, error::DxError};
use crate::{create_type, implement_fns};

create_type! { Factory4, IDXGIFactory4; }
create_type! { Factory6, IDXGIFactory6; Factory4 }
create_type! { Factory7, IDXGIFactory7; Factory4, Factory6 }

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

    pub fn create_swapchain_for_hwnd<CQ>(&self, command_queue: &CQ, hwnd: isize, desc: SwapchainDesc) -> Result<Swapchain1, DxError>
    where
        CQ: Into<CommandQueue>
    {
        todo!()
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
