use windows::core::Interface;
use windows::Win32::Graphics::Dxgi::{
    IDXGIAdapter, IDXGIAdapter3, IDXGIFactory4, IDXGIFactory6, IDXGIFactory7,
};

use crate::create_type;
use crate::{adapter::Adapter3, error::DxError};

create_type!(Factory4, IDXGIFactory4);
create_type!(Factory6, IDXGIFactory6, Factory4);
create_type!(Factory7, IDXGIFactory7, Factory6);

impl Factory4 {
    pub fn enum_adapters(&self, index: usize) -> Result<Adapter3, DxError> {
        let adapter = unsafe {
            self.0
                .EnumAdapters1(index as u32)
                .map_err(|_| DxError::NotFoundAdapters)?
        }
        .cast::<IDXGIAdapter3>()
        .expect("IDXGIFactory4 should support IDXGIAdapter3");

        Ok(Adapter3::new(adapter))
    }

    pub fn enum_warp_adapters(&self) -> Result<Adapter3, DxError> {
        let adapter = unsafe {
            self.0
                .EnumWarpAdapter::<IDXGIAdapter>()
                .map_err(|_| DxError::NotFoundAdapters)?
        }
        .cast::<IDXGIAdapter3>()
        .expect("IDXGIFactory4 should support IDXGIAdapter3");

        Ok(Adapter3::new(adapter))
    }
}
