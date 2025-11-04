mod enums;
mod flags;

use windows::Win32::{
    Foundation::*,
    Graphics::{
        Direct3D::*,
        Dxgi::{Common::*, *},
    },
};

use crate::{dx::DxgiError, error::DxError, types::*};

impl From<windows::core::Error> for DxError {
    fn from(value: windows::core::Error) -> Self {
        match value.code() {
            D3D12_ERROR_ADAPTER_NOT_FOUND => DxError::AdapterNotFound,
            D3D12_ERROR_DRIVER_VERSION_MISMATCH => DxError::DriverVersionMismatch,
            E_FAIL => DxError::Fail(value.message()),
            E_INVALIDARG => DxError::InvalidArgs,
            E_OUTOFMEMORY => DxError::Oom,
            E_NOTIMPL => DxError::NotImpl,
            DXGI_ERROR_ACCESS_DENIED => DxError::Dxgi(DxgiError::AccessDenied, value.message()),
            DXGI_ERROR_ACCESS_LOST => DxError::Dxgi(DxgiError::AccessLost, value.message()),
            DXGI_ERROR_ALREADY_EXISTS => DxError::Dxgi(DxgiError::AlreadyExists, value.message()),
            DXGI_ERROR_CANNOT_PROTECT_CONTENT => {
                DxError::Dxgi(DxgiError::CannotProtectContent, value.message())
            }
            DXGI_ERROR_DEVICE_HUNG => DxError::Dxgi(DxgiError::DeviceHung, value.message()),
            DXGI_ERROR_DEVICE_REMOVED => DxError::Dxgi(DxgiError::DeviceRemoved, value.message()),
            DXGI_ERROR_DEVICE_RESET => DxError::Dxgi(DxgiError::DeviceReset, value.message()),
            DXGI_ERROR_DRIVER_INTERNAL_ERROR => {
                DxError::Dxgi(DxgiError::DriverInternalError, value.message())
            }
            DXGI_ERROR_FRAME_STATISTICS_DISJOINT => {
                DxError::Dxgi(DxgiError::FrameStatisticsDisjoint, value.message())
            }
            DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE => {
                DxError::Dxgi(DxgiError::GraphicsVidpnSourceInUse, value.message())
            }
            DXGI_ERROR_INVALID_CALL => DxError::Dxgi(DxgiError::InvalidCall, value.message()),
            DXGI_ERROR_MORE_DATA => DxError::Dxgi(DxgiError::MoreData, value.message()),
            DXGI_ERROR_NAME_ALREADY_EXISTS => {
                DxError::Dxgi(DxgiError::NameAlreadyExists, value.message())
            }
            DXGI_ERROR_NONEXCLUSIVE => DxError::Dxgi(DxgiError::NonExclusive, value.message()),
            DXGI_ERROR_NOT_CURRENTLY_AVAILABLE => {
                DxError::Dxgi(DxgiError::NotCurrentlyAvailable, value.message())
            }
            DXGI_ERROR_NOT_FOUND => DxError::Dxgi(DxgiError::NotFound, value.message()),
            DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED => {
                DxError::Dxgi(DxgiError::RemoteClientDisconnected, value.message())
            }
            DXGI_ERROR_REMOTE_OUTOFMEMORY => DxError::Dxgi(DxgiError::RemoteOom, value.message()),
            DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE => {
                DxError::Dxgi(DxgiError::RestrictToOutputStale, value.message())
            }
            DXGI_ERROR_SDK_COMPONENT_MISSING => {
                DxError::Dxgi(DxgiError::SdkComponentMissing, value.message())
            }
            DXGI_ERROR_SESSION_DISCONNECTED => {
                DxError::Dxgi(DxgiError::SessionDisconnected, value.message())
            }
            DXGI_ERROR_UNSUPPORTED => DxError::Dxgi(DxgiError::Unsupported, value.message()),
            DXGI_ERROR_WAIT_TIMEOUT => DxError::Dxgi(DxgiError::WaitTimeout, value.message()),
            DXGI_ERROR_WAS_STILL_DRAWING => {
                DxError::Dxgi(DxgiError::WasStillDrawing, value.message())
            }
            _ => DxError::Other(value.message()),
        }
    }
}
