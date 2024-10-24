use std::sync::Mutex;

use windows::{core::Interface, Win32::Graphics::Direct3D12::*};

use crate::{
    create_type, dx::CallbackData, impl_trait, types::GpuBasedValidationFlags, HasInterface,
};

static CALLBACK_HANDLER: Mutex<Option<CallbackData>> = Mutex::new(None);

/// An interface used to turn on the debug layer.
///
/// For more information: [`ID3D12Debug interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug)
pub trait IDebug: HasInterface<Raw: Interface> {
    /// Enables the debug layer.
    ///
    /// For more information: [`ID3D12Debug::EnableDebugLayer method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug-enabledebuglayer)
    fn enable_debug_layer(&self);
}

pub trait IDebugExt: IDebug {
    fn set_callback(&self, callback: CallbackData);
    fn take_callback(&self);
}

/// Adds GPU-Based Validation and Dependent Command Queue Synchronization to the debug layer.
///
/// For more information: [`ID3D12Debug1 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug1)
pub trait IDebug1: IDebug {
    /// This method enables or disables GPU-Based Validation (GBV) before creating a device with the debug layer enabled.
    ///
    /// For more information: [`ID3D12Debug1::SetEnableGPUBasedValidation method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug1-setenablegpubasedvalidation)
    fn set_enable_gpu_based_validation(&self, enable: bool);

    /// Enables or disables dependent command queue synchronization when using a D3D12 device with the debug layer enabled.
    ///
    /// For more information: [`ID3D12Debug1::SetEnableSynchronizedCommandQueueValidation method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug1-setenablesynchronizedcommandqueuevalidation)
    fn set_enable_synchronized_command_queue_validation(&self, enable: bool);
}

/// Adds configurable levels of GPU-based validation to the debug layer.
///
/// For more information: [`ID3D12Debug2 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug2)
pub trait IDebug2: IDebug1 {
    /// This method configures the level of GPU-based validation that the debug device is to perform at runtime.
    ///
    /// For more information: [`ID3D12Debug2::SetGPUBasedValidationFlags method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug2-setgpubasedvalidationflags)
    fn set_gpu_based_validation_flags(&self, flags: GpuBasedValidationFlags);
}

/// Adds the ability to disable the debug layer.
///
/// For more information: [`ID3D12Debug4 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug4)
pub trait IDebug4: IDebug2 {
    /// Disables the debug layer.
    ///
    /// For more information: [`ID3D12Debug4::DisableDebugLayer method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug4-disabledebuglayer)
    fn disable_debug_layer(&self);
}

/// Adds to the debug layer the ability to configure the auto-naming of objects.
///
/// For more information: [`ID3D12Debug5 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug5)
pub trait IDebug5: IDebug4 {
    /// Configures the auto-naming of objects.
    ///
    /// For more information: [`ID3D12Debug5::SetEnableAutoName method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug5-setenableautoname)
    fn set_enable_auto_name(&self, enable: bool);
}

/// Adds to the debug layer the ability to configure the auto-naming of objects.
///
/// For more information: [`ID3D12Debug5 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug5)
pub trait IDebug6: IDebug5 {
    /// TBD
    ///
    /// For more information: [`ID3D12Debug6::SetForceLegacyBarrierValidation method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug6-setforcelegacybarriervalidation)
    fn set_force_legacy_barrier_validation(&self, enable: bool);
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
    /// For more information: [`ID3D12Debug3 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug3)
    Debug3 wrap ID3D12Debug3; decorator for Debug1, Debug
}

create_type! {
    /// Adds the ability to disable the debug layer.
    ///
    /// For more information: [`ID3D12Debug4 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug4)
    Debug4 wrap ID3D12Debug4; decorator for Debug3, Debug1, Debug
}

create_type! {
    /// Adds to the debug layer the ability to configure the auto-naming of objects.
    ///
    /// For more information: [`ID3D12Debug5 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug5)
    Debug5 wrap ID3D12Debug5; decorator for Debug4, Debug3, Debug1, Debug
}

create_type! {
   /// A debug interface controls debug settings.
   ///
   /// For more information: [`ID3D12Debug6 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nn-d3d12sdklayers-id3d12debug6)
   Debug6 wrap ID3D12Debug6; decorator for Debug5, Debug4, Debug3, Debug1, Debug
}

impl_trait! {
    impl IDebug =>
    Debug,
    Debug1,
    Debug3,
    Debug4,
    Debug5,
    Debug6;

    fn enable_debug_layer(&self) {
        unsafe {
            self.0.EnableDebugLayer();
        }
    }
}

impl_trait! {
    impl IDebug1 =>
    Debug1,
    Debug3,
    Debug4,
    Debug5,
    Debug6;

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

impl_trait! {
    impl IDebug2 =>
    Debug3,
    Debug4,
    Debug5,
    Debug6;

    fn set_gpu_based_validation_flags(&self, flags: GpuBasedValidationFlags) {
        unsafe {
            self.0.SetGPUBasedValidationFlags(flags.as_raw());
        }
    }
}

impl_trait! {
    impl IDebug4 =>
    Debug4,
    Debug5,
    Debug6;

    fn disable_debug_layer(&self) {
        unsafe {
            self.0.DisableDebugLayer();
        }
    }
}

impl_trait! {
    impl IDebug5 =>
    Debug5,
    Debug6;

    fn set_enable_auto_name(&self, enable: bool) {
        unsafe {
            self.0.SetEnableAutoName(enable);
        }
    }
}

impl_trait! {
    impl IDebug6 =>
    Debug6;

    fn set_force_legacy_barrier_validation(&self, enable: bool) {
        unsafe {
            self.0.SetForceLegacyBarrierValidation(enable);
        }
    }
}

impl_trait! {
    impl IDebugExt =>
    Debug,
    Debug1,
    Debug3,
    Debug4,
    Debug5,
    Debug6;

    fn set_callback(&self, callback: CallbackData) {
        let mut guard = CALLBACK_HANDLER.lock().unwrap();

        if guard.is_some() {
            return;
        }

        AddVectoredExceptionHandler(0, Some(debug_callback));
        std::mem::replace(&mut *guard, Some(callback));
    }

    fn take_callback(&self) {
        let mut guard = CALLBACK_HANDLER.lock().unwrap();
        std::mem::take(&mut *guard);
    }
}

unsafe extern "system" fn dx_callback(
    category: D3D12_MESSAGE_CATEGORY,
    severity: D3D12_MESSAGE_SEVERITY,
    id: D3D12_MESSAGE_ID,
    pdescription: PCSTR,
    pcontext: *mut core::ffi::c_void,
) {
    let message = str::from_utf8(pdescription.as_bytes()).unwrap();
    let callback = pcontext.cast::<CallbackData>();
    (*callback)(category.into(), severity.into(), id.into(), message);
}

unsafe extern "system" fn debug_callback(exception_info: *mut EXCEPTION_POINTERS) -> i32 {
    Debug::EXCEPTION_CONTINUE_EXECUTION
}
