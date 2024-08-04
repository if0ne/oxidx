use windows::Win32::Graphics::Direct3D12::{D3D12CreateDevice, D3D12GetDebugInterface};
use windows::Win32::Graphics::Dxgi::CreateDXGIFactory2;

use crate::adapter::IAdapter3;
use crate::debug::IDebug;
use crate::device::IDevice;
use crate::error::DxError;
use crate::factory::IFactory4;
use crate::types::{FactoryCreationFlags, FeatureLevel};

/// Unit-strucutre that has two methods: [`create_factory`](crate::entry::Entry::create_factory), [`create_device`](crate::entry::Entry::create_device)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry;

impl Entry {
    /// Creates a DXGI 1.3 factory that you can use to generate other DXGI objects.
    ///
    /// For more information: [`CreateDXGIFactory2 function`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-createdxgifactory2)
    pub fn create_factory<F: IFactory4>(&self, flags: FactoryCreationFlags) -> Result<F, DxError> {
        unsafe {
            let inner: F::Raw = CreateDXGIFactory2(flags.bits()).map_err(DxError::from)?;

            Ok(F::new(inner))
        }
    }

    /// Creates a device that represents the display adapter.
    ///
    /// For more information: [`D3D12CreateDevice function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-d3d12createdevice)
    pub fn create_device<A: IAdapter3, D: IDevice>(
        &self,
        adapter: &A,
        feature_level: FeatureLevel,
    ) -> Result<D, DxError> {
        unsafe {
            let mut inner: Option<D::Raw> = None;

            D3D12CreateDevice(adapter.as_raw_ref(), feature_level.as_raw(), &mut inner)
                .map_err(DxError::from)?;

            let inner = inner.unwrap();

            Ok(D::new(inner))
        }
    }

    /// Gets a debug interface.
    ///
    /// For more information: [`D3D12GetDebugInterface function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-d3d12getdebuginterface)
    pub fn create_debug<D: IDebug>(&self) -> Result<D, DxError> {
        unsafe {
            let mut inner: Option<D::Raw> = None;

            D3D12GetDebugInterface(&mut inner).map_err(DxError::from)?;
            let inner = inner.unwrap();

            Ok(D::new(inner))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{device::Device, factory::*, types::FactoryCreationFlags};

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
