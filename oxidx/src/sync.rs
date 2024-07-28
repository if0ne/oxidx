use windows::{
    core::{Interface, Param},
    Win32::{
        Foundation::HANDLE,
        Graphics::Direct3D12::{ID3D12Fence, ID3D12Fence1},
        System::Threading::{CreateEventA, ResetEvent, WaitForSingleObject},
    },
};

use crate::{create_type, error::DxError, impl_trait, types::FenceFlags, HasInterface};

/// Represents a fence, an object used for synchronization of the CPU and one or more GPUs.
///
/// For more information: [`ID3D12Fence interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12fence)
pub trait IFence:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12Fence>>
{
    /// Gets the current value of the fence.
    ///
    /// # Returns
    /// Returns the current value of the fence. If the device has been removed, the return value will be [`u64::MAX`].
    ///
    /// For more information: [`ID3D12Fence::GetCompletedValue method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12fence-getcompletedvalue)
    fn get_completed_value(&self) -> u64;

    /// Specifies an event that's raised when the fence reaches a certain value.
    ///
    /// # Arguments
    /// * `value` - The fence value when the event is to be signaled.
    /// * `event` - A handle to the event object.
    ///
    /// For more information: [`ID3D12Fence::SetEventOnCompletion method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12fence-seteventoncompletion)
    fn set_event_on_completion(&self, value: u64, event: Event) -> Result<(), DxError>;

    /// Sets the fence to the specified value.
    ///
    /// # Arguments
    /// * `value` - The value to set the fence to.
    ///
    /// For more information: [`ID3D12Fence::Signal method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12fence-signal)
    fn signal(&self, value: u64) -> Result<(), DxError>;
}

/// Represents a fence. This interface extends [`FenceInterface1`], and supports the retrieval of the flags used to create the original fence.
/// This new feature is useful primarily for opening shared fences.
///
/// For more information: [`ID3D12Fence1 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12fence1)
pub trait IFence1: IFence {
    /// Gets the flags used to create the fence represented by the current instance.
    ///
    /// # Returns
    /// The flags used to create the fence.
    ///
    /// For more information: [`ID3D12Fence1::GetCreationFlags method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12fence1-getcreationflags)
    fn get_creation_flags(&self) -> FenceFlags;
}

create_type! {
    /// Represents a fence, an object used for synchronization of the CPU and one or more GPUs.
    ///
    /// For more information: [`ID3D12Fence interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12fence)
    Fence wrap ID3D12Fence
}

create_type! {
    /// Represents a fence. This interface extends [`FenceInterface1`], and supports the retrieval of the flags used to create the original fence.
    /// This new feature is useful primarily for opening shared fences.
    ///
    /// For more information: [`ID3D12Fence1 interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12fence1)
    Fence1 wrap ID3D12Fence1; decorator for Fence
}

impl_trait! {
    impl IFence =>
    Fence,
    Fence1;

    fn get_completed_value(&self) -> u64 {
        unsafe { self.0.GetCompletedValue() }
    }

    fn set_event_on_completion(&self, value: u64, event: Event) -> Result<(), DxError> {
        unsafe {
            self.0.SetEventOnCompletion(value, event.0).map_err(DxError::from)
        }
    }

    fn signal(&self, value: u64) -> Result<(), DxError> {
        unsafe { self.0.Signal(value).map_err(DxError::from) }
    }
}

impl_trait! {
    impl IFence1 =>
    Fence1;

    fn get_creation_flags(&self) -> FenceFlags {
        unsafe {
            self.0.GetCreationFlags().into()
        }
    }
}

/// A handle to the object of event.
#[derive(Clone, Copy, Debug)]
pub struct Event(pub(crate) HANDLE);
impl Event {
    /// Creates or opens a named or unnamed event object.
    ///
    /// # Arguments
    /// * `manual_reset` - If this parameter is true, the function creates a manual-reset event object, which requires the use of the [`Event::reset`]
    ///   function to set the event state to nonsignaled. If this parameter is FALSE, the function creates an auto-reset event object, and the
    ///   system automatically resets the event state to nonsignaled after a single waiting thread has been released.
    /// * `initial_state` - If this parameter is true, the initial state of the event object is signaled; otherwise, it is nonsignaled.
    ///
    /// # Returns
    /// The return value is a handle to the event object.
    ///
    /// For more information: [`CreateEventA`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventa)
    pub fn create(manual_reset: bool, initial_state: bool) -> Result<Self, DxError> {
        Ok(Event(unsafe {
            CreateEventA(None, manual_reset, initial_state, None).map_err(DxError::from)?
        }))
    }

    /// Sets the specified event object to the nonsignaled state.
    ///
    /// For more information: [`ResetEvent`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-resetevent)
    pub fn reset(&self) -> Result<(), DxError> {
        unsafe { ResetEvent(self.0).map_err(DxError::from) }
    }

    /// Waits until the specified object is in the signaled state or the time-out interval elapses.
    ///
    /// # Arguments
    /// * `timeout_ms` - The time-out interval, in milliseconds. If a nonzero value is specified, the function waits until the object is signaled
    ///   or the interval elapses. If timeout_ms is zero, the function does not enter a wait state if the object is not signaled;
    ///   it always returns immediately. If timeout_ms is [`u32::MAX`], the function will return only when the object is signaled.
    ///
    /// For more information: [`WaitForSingleObject`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)
    pub fn wait(&self, timeout_ms: u32) -> u32 {
        unsafe { WaitForSingleObject(self.0, timeout_ms).0 }
    }
}
