use windows::{core::Interface, Win32::Graphics::Direct3D12::*};

use crate::{create_type, impl_trait, types::GpuBasedValidationFlags, HasInterface};

/// An interface used to turn on the debug layer.
///
/// For more information: [`ID3D12Debug interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug)
pub trait DebugInterface: HasInterface<Raw: Interface> {
    /// Enables the debug layer.
    ///
    /// For more information: [`ID3D12Debug::EnableDebugLayer method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug-enabledebuglayer)
    fn enable_debug_layer(&self);
}

/// Adds GPU-Based Validation and Dependent Command Queue Synchronization to the debug layer.
///
/// For more information: [`ID3D12Debug1 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug1)
pub trait Debug1Interface: HasInterface<Raw: Interface> {
    /// This method enables or disables GPU-Based Validation (GBV) before creating a device with the debug layer enabled.
    ///
    /// # Arguments
    /// * `enable` - TRUE to enable GPU-Based Validation, otherwise FALSE.
    ///
    /// For more information: [`ID3D12Debug1::SetEnableGPUBasedValidation method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug1-setenablegpubasedvalidation)
    fn set_enable_gpu_based_validation(&self, enable: bool);

    /// Enables or disables dependent command queue synchronization when using a D3D12 device with the debug layer enabled.
    ///
    /// # Arguments
    /// * `enable` - TRUE to enable Dependent Command Queue Synchronization, otherwise FALSE.
    ///
    /// For more information: [`ID3D12Debug1::SetEnableSynchronizedCommandQueueValidation method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug1-setenablesynchronizedcommandqueuevalidation)
    fn set_enable_synchronized_command_queue_validation(&self, enable: bool);
}

/// Adds configurable levels of GPU-based validation to the debug layer.
///
/// For more information: [`ID3D12Debug2 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug2)
pub trait Debug2Interface: HasInterface<Raw: Interface> {
    /// This method configures the level of GPU-based validation that the debug device is to perform at runtime.
    ///
    /// # Arguments
    /// * `flags` - Specifies the level of GPU-based validation to perform at runtime.
    ///
    /// For more information: [`ID3D12Debug2::SetGPUBasedValidationFlags method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug2-setgpubasedvalidationflags)
    fn set_gpu_based_validation_flags(&self, flags: GpuBasedValidationFlags);
}

create_type! {
    /// An interface used to turn on the debug layer.
    ///
    /// For more information: [`ID3D12Debug interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug)
    Debug wrap ID3D12Debug
}

create_type! {
    /// An interface used to turn on the debug layer.
    ///
    /// For more information: [`ID3D12Debug1 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug1)
    Debug1 wrap ID3D12Debug1; decorator for Debug
}

create_type! {
    /// Adds configurable levels of GPU-based validation to the debug layer.
    ///
    /// For more information: [`ID3D12Debug2 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug2)
    Debug2 wrap ID3D12Debug2; decorator for Debug1, Debug
}

impl_trait! {
    impl DebugInterface =>
    Debug,
    Debug1;

    fn enable_debug_layer(&self) {
        unsafe {
            self.0.EnableDebugLayer();
        }
    }
}

impl_trait! {
    impl Debug1Interface =>
    Debug1;

    fn set_enable_gpu_based_validation(&self, enable: bool) {
        unsafe {
            self.0.SetEnableGPUBasedValidation(enable);
        }
    }

    fn set_enable_synchronized_command_queue_validation(&self, enable: bool) {
        unsafe {
            self.0.SetEnableSynchronizedCommandQueueValidation(enable);
        }
    }
}
