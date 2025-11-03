use windows::Win32::Graphics::Direct3D12::{D3D12CreateDevice, D3D12GetDebugInterface};
use windows::Win32::Graphics::Dxgi::CreateDXGIFactory2;

use crate::dx::{Adapter3, Debug, Device, Factory4};
use crate::error::DxError;
use crate::types::{FactoryCreationFlags, FeatureLevel};

/// Creates a DXGI 1.3 factory that you can use to generate other DXGI objects.
///
/// For more information: [`CreateDXGIFactory2 function`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-createdxgifactory2)
pub fn create_factory4(flags: FactoryCreationFlags) -> Result<Factory4, DxError> {
    unsafe {
        let inner = CreateDXGIFactory2(flags.as_raw()).map_err(DxError::from)?;

        Ok(Factory4(inner))
    }
}

/// Creates a device that represents the display adapter.
///
/// For more information: [`D3D12CreateDevice function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-d3d12createdevice)
pub fn create_device<'a>(
    adapter: impl Into<Option<&'a Adapter3>>,
    feature_level: FeatureLevel,
) -> Result<Device, DxError> {
    unsafe {
        let mut inner = None;

        if let Some(adapter) = adapter.into() {
            D3D12CreateDevice(&adapter.0, feature_level.as_raw(), &mut inner)
                .map_err(DxError::from)?;
        } else {
            D3D12CreateDevice(None, feature_level.as_raw(), &mut inner).map_err(DxError::from)?;
        }

        let inner = inner.unwrap();

        Ok(Device(inner))
    }
}

/// Gets a debug interface.
///
/// For more information: [`D3D12GetDebugInterface function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-d3d12getdebuginterface)
pub fn create_debug() -> Result<Debug, DxError> {
    unsafe {
        let mut inner = None;

        D3D12GetDebugInterface(&mut inner).map_err(DxError::from)?;
        let inner = inner.unwrap();

        Ok(Debug(inner))
    }
}

#[cfg(test)]
mod test {
    use crate::{dx::Factory7, types::FactoryCreationFlags};

    use super::*;

    #[test]
    fn create_factory4_test() {
        let factory = create_factory4(FactoryCreationFlags::Debug);

        assert!(factory.is_ok())
    }

    #[test]
    fn create_device_test() {
        let device = create_device(None, FeatureLevel::Level11);
        assert!(device.is_ok());
    }

    #[test]
    fn as_ref_factory_test() {
        fn test(factory: impl AsRef<Factory4>) {
            let _ = factory.as_ref().enum_adapters(0).unwrap();
        }

        let factory = create_factory4(FactoryCreationFlags::empty()).unwrap();
        let factory: Factory7 = factory.try_into().unwrap();

        test(&factory);
    }
}
