mod enums;
mod flags;
mod structs;

use windows::Win32::{
    Foundation::*,
    Graphics::{
        Direct3D::*,
        Dxgi::{Common::*, *},
    },
};

use crate::{error::DxError, types::*, HasInterface};

impl From<windows::core::Error> for DxError {
    fn from(value: windows::core::Error) -> Self {
        match value.code() {
            D3D12_ERROR_ADAPTER_NOT_FOUND => DxError::AdapterNotFound,
            D3D12_ERROR_DRIVER_VERSION_MISMATCH => DxError::DriverVersionMismatch,
            E_FAIL => DxError::Fail(value.message()),
            E_INVALIDARG => DxError::InvalidArgs,
            E_OUTOFMEMORY => DxError::Oom,
            E_NOTIMPL => DxError::NotImpl,
            _ => DxError::Dxgi(value.message()),
        }
    }
}
