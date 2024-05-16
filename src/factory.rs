use windows::Win32::Graphics::Dxgi::IDXGIFactory;

use crate::{adapter::Adapter, error::DxError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Factory(pub(crate) IDXGIFactory);

impl Factory {
    pub fn enum_adapters(&self, index: usize) -> Result<Adapter, DxError> {
        let adapter = unsafe {
            self.0.EnumAdapters(index as u32).map_err(|_| DxError::NotFoundAdapters)?
        };

        Ok(Adapter(adapter))
    }
}
