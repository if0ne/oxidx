use windows::{
    core::{IUnknown, Interface, Param},
    Win32::Graphics::Dxgi::{IDXGIAdapter3, IDXGIOutput1},
};

use crate::{
    create_type,
    dx::{DxError, Output1},
    impl_trait,
    types::AdapterDesc1,
    HasInterface,
};

/// This interface adds some memory residency methods, for budgeting and reserving physical memory.
///
/// For more information: [`IDXGIAdapter3 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_4/nn-dxgi1_4-idxgiadapter3)
pub trait IAdapter3: for<'a> HasInterface<RawRef<'a>: Param<IUnknown>> {
    ///Enumerate adapter (video card) outputs.
    ///
    /// For more information: [`IDXGIAdapter1::EnumOutputs method`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiadapter1-enumoutputs)
    fn enum_outputs(&self, index: usize) -> Result<Output1, DxError>;

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

    fn enum_outputs(&self, index: usize) -> Result<Output1, DxError> {
        unsafe {
            let raw = self.0.EnumOutputs(index as u32).map_err(DxError::from)?;
            let raw = raw.cast::<IDXGIOutput1>().map_err(|_| DxError::Cast("IDXGIOutput", "IDXGIOutput1"))?;

            Ok(Output1::new(raw))
        }
    }

    fn get_desc1(&self) -> Result<AdapterDesc1, DxError> {
        unsafe {
            self.0.GetDesc1()
                .map(|d| AdapterDesc1(d))
                .map_err(DxError::from)
        }
    }
}
