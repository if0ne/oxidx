use windows::{
    core::{IUnknown, Param},
    Win32::Graphics::Dxgi::IDXGIAdapter3,
};

use crate::{create_type, dx::DxError, impl_trait, types::AdapterDesc1, HasInterface};

/// This interface adds some memory residency methods, for budgeting and reserving physical memory.
///
/// For more information: [`IDXGIAdapter3 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_4/nn-dxgi1_4-idxgiadapter3)
pub trait IAdapter3: for<'a> HasInterface<RawRef<'a>: Param<IUnknown>> {
    /// Gets a DXGI 1.1 description of an adapter (or video card).
    ///
    /// For more information: [`IDXGIAdapter1::GetDesc1 method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter1-getdesc1)
    fn get_desc1(&self) -> Result<AdapterDesc1, DxError>;
}

create_type! {
    /// This interface adds some memory residency methods, for budgeting and reserving physical memory.
    ///
    /// For more information: [`IDXGIAdapter3 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_4/nn-dxgi1_4-idxgiadapter3)
    Adapter3 wrap IDXGIAdapter3
}

impl_trait! {
    impl IAdapter3 =>
    Adapter3;

    fn get_desc1(&self) -> Result<AdapterDesc1, DxError> {
        unsafe {
            let mut desc = Default::default();

            self.0.GetDesc1(&mut desc).map_err(DxError::from)?;

            Ok(AdapterDesc1(desc))
        }
    }
}
