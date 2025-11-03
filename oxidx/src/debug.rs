use core::str;

use windows::{core::Interface, Win32::Graphics::Direct3D12::*};

#[cfg(feature = "callback")]
use windows::Win32::System::Diagnostics::Debug::{
    AddVectoredExceptionHandler, EXCEPTION_CONTINUE_EXECUTION, EXCEPTION_CONTINUE_SEARCH,
    EXCEPTION_POINTERS,
};

use crate::{create_type, dx::MessageSeverity, impl_interface, types::GpuBasedValidationFlags};

const MESSAGE_PREFIXES: &[(&str, MessageSeverity)] = &[
    ("CORRUPTION", MessageSeverity::Corruption),
    ("ERROR", MessageSeverity::Error),
    ("WARNING", MessageSeverity::Warning),
    ("INFO", MessageSeverity::Info),
    ("MESSAGE", MessageSeverity::Message),
];

#[cfg(feature = "callback")]
static CALLBACK_HANDLER: std::sync::Mutex<Option<crate::types::CallbackData>> =
    std::sync::Mutex::new(None);

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

impl_interface! {
    Debug,
    Debug1,
    Debug3,
    Debug4,
    Debug5,
    Debug6;

    /// Enables the debug layer.
    ///
    /// For more information: [`ID3D12Debug::EnableDebugLayer method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug-enabledebuglayer)
    pub fn enable_debug_layer(&self) {
        unsafe {
            self.0.EnableDebugLayer();
        }
    }
}

impl_interface! {
    Debug1,
    Debug3,
    Debug4,
    Debug5,
    Debug6;

    /// This method enables or disables GPU-Based Validation (GBV) before creating a device with the debug layer enabled.
    ///
    /// For more information: [`ID3D12Debug1::SetEnableGPUBasedValidation method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug1-setenablegpubasedvalidation)
    pub fn set_enable_gpu_based_validation(&self, enable: bool) {
        unsafe {
            self.0.SetEnableGPUBasedValidation(enable);
        }
    }

    /// Enables or disables dependent command queue synchronization when using a D3D12 device with the debug layer enabled.
    ///
    /// For more information: [`ID3D12Debug1::SetEnableSynchronizedCommandQueueValidation method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug1-setenablesynchronizedcommandqueuevalidation)
    pub fn set_enable_synchronized_command_queue_validation(&self, enable: bool) {
        unsafe {
            self.0.SetEnableSynchronizedCommandQueueValidation(enable);
        }
    }
}

impl_interface! {
    Debug3,
    Debug4,
    Debug5,
    Debug6;

    /// This method configures the level of GPU-based validation that the debug device is to perform at runtime.
    ///
    /// For more information: [`ID3D12Debug2::SetGPUBasedValidationFlags method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug2-setgpubasedvalidationflags)
    pub fn set_gpu_based_validation_flags(&self, flags: GpuBasedValidationFlags) {
        unsafe {
            self.0.SetGPUBasedValidationFlags(flags.as_raw());
        }
    }
}

impl_interface! {
    Debug4,
    Debug5,
    Debug6;

    /// Disables the debug layer.
    ///
    /// For more information: [`ID3D12Debug4::DisableDebugLayer method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug4-disabledebuglayer)
    pub fn disable_debug_layer(&self) {
        unsafe {
            self.0.DisableDebugLayer();
        }
    }
}

impl_interface! {
    Debug5,
    Debug6;

    /// Configures the auto-naming of objects.
    ///
    /// For more information: [`ID3D12Debug5::SetEnableAutoName method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug5-setenableautoname)
    pub fn set_enable_auto_name(&self, enable: bool) {
        unsafe {
            self.0.SetEnableAutoName(enable);
        }
    }
}

impl_interface! {
    Debug6;

    /// TBD
    ///
    /// For more information: [`ID3D12Debug6::SetForceLegacyBarrierValidation method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12sdklayers/nf-d3d12sdklayers-id3d12debug6-setforcelegacybarriervalidation)
    pub fn set_force_legacy_barrier_validation(&self, enable: bool) {
        unsafe {
            self.0.SetForceLegacyBarrierValidation(enable);
        }
    }
}

#[cfg(feature = "callback")]
impl_interface! {
    Debug,
    Debug1,
    Debug3,
    Debug4,
    Debug5,
    Debug6;

    pub fn set_callback(&self, callback: crate::types::CallbackData) {
        unsafe {
            let mut guard = CALLBACK_HANDLER.lock().unwrap();

            if guard.is_some() {
                return;
            }

            AddVectoredExceptionHandler(0, Some(debug_callback));
            *guard = Some(callback);
        }
    }

    pub fn take_callback(&self) {
        unsafe {
            let mut guard = CALLBACK_HANDLER.lock().unwrap();
            AddVectoredExceptionHandler(0, None);
            std::mem::take(&mut *guard);
        }
    }
}

#[cfg(feature = "callback")]
unsafe extern "system" fn dx_callback(
    category: D3D12_MESSAGE_CATEGORY,
    severity: D3D12_MESSAGE_SEVERITY,
    id: D3D12_MESSAGE_ID,
    pdescription: windows::core::PCSTR,
    pcontext: *mut core::ffi::c_void,
) {
    let message = str::from_utf8(pdescription.as_bytes()).unwrap();
    let callback = pcontext.cast::<crate::types::CallbackData>();
    (*callback)(category.into(), severity.into(), id.into(), message);
}

#[cfg(feature = "callback")]
unsafe extern "system" fn debug_callback(exception_info: *mut EXCEPTION_POINTERS) -> i32 {
    // See https://stackoverflow.com/a/41480827
    let record = unsafe { &*(*exception_info).ExceptionRecord };
    if record.NumberParameters != 2 {
        return EXCEPTION_CONTINUE_SEARCH;
    }
    let message = match record.ExceptionCode {
        windows::Win32::Foundation::DBG_PRINTEXCEPTION_C => String::from_utf8_lossy(unsafe {
            std::slice::from_raw_parts(
                record.ExceptionInformation[1] as *const u8,
                record.ExceptionInformation[0],
            )
        }),
        windows::Win32::Foundation::DBG_PRINTEXCEPTION_WIDE_C => {
            std::borrow::Cow::Owned(String::from_utf16_lossy(unsafe {
                std::slice::from_raw_parts(
                    record.ExceptionInformation[1] as *const u16,
                    record.ExceptionInformation[0],
                )
            }))
        }
        _ => return EXCEPTION_CONTINUE_SEARCH,
    };

    let message = match message.strip_prefix("D3D12 ") {
        Some(msg) => msg
            .trim_end_matches("\n\0")
            .trim_end_matches("[ STATE_CREATION WARNING #0: UNKNOWN]"),
        None => return EXCEPTION_CONTINUE_SEARCH,
    };

    let (message, level) = match MESSAGE_PREFIXES
        .iter()
        .find(|&&(prefix, _)| message.starts_with(prefix))
    {
        Some(&(prefix, level)) => (&message[prefix.len() + 2..], level),
        None => (message, MessageSeverity::Message),
    };

    if let Some(callback) = &*CALLBACK_HANDLER.lock().unwrap() {
        callback(
            crate::types::MessageCategory::Execution,
            level,
            crate::types::MessageId::Unknown,
            message,
        );
    }

    EXCEPTION_CONTINUE_EXECUTION
}
