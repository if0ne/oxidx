use std::ops::Deref;

use windows::Win32::Graphics::Dxgi::{IDXGIAdapter, IDXGIAdapter3, IDXGIFactory4, IDXGIFactory6, IDXGIFactory7};
use windows::core::Interface;

use crate::{adapter::Adapter, error::DxError};

#[derive(Clone, Debug, PartialEq, Eq)]
struct FactoryInner<T: Interface>(T);

impl<T: Interface> Deref for FactoryInner<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Factory4(FactoryInner<IDXGIFactory4>);

impl Factory4 {
    pub fn enum_adapters(&self, index: usize) -> Result<Adapter, DxError> {
        let adapter = unsafe {
            self.0.EnumAdapters1(index as u32).map_err(|_| DxError::NotFoundAdapters)?
        }.cast::<IDXGIAdapter3>().expect("IDXGIFactory4 should support IDXGIAdapter3");

        Ok(Adapter(adapter))
    }

    pub fn enum_warp_adapters(&self) -> Result<Adapter, DxError> {
        let adapter = unsafe {
            self.0.EnumWarpAdapter::<IDXGIAdapter>().map_err(|_| DxError::NotFoundAdapters)?
        }.cast::<IDXGIAdapter3>().expect("IDXGIFactory4 should support IDXGIAdapter3");

        Ok(Adapter(adapter))
    }
}

impl TryInto<Factory6> for Factory4 {
    type Error = DxError;

    fn try_into(self) -> Result<Factory6, Self::Error> {
        let temp = self.0.cast::<_>().map_err(|_| DxError::CastError)?;

        Ok(Factory6(FactoryInner(temp)))
    }
}

impl TryInto<Factory7> for Factory4 {
    type Error = DxError;

    fn try_into(self) -> Result<Factory7, Self::Error> {
        let temp = self.0.cast::<_>().map_err(|_| DxError::CastError)?;

        Ok(Factory7(FactoryInner(temp)))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Factory6(FactoryInner<IDXGIFactory6>);

impl TryInto<Factory7> for Factory6 {
    type Error = DxError;

    fn try_into(self) -> Result<Factory7, Self::Error> {
        let temp = self.0.cast::<_>().map_err(|_| DxError::CastError)?;

        Ok(Factory7(FactoryInner(temp)))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Factory7(FactoryInner<IDXGIFactory7>);
