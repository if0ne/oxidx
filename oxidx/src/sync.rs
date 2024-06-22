use windows::{
    core::{Interface, Param},
    Win32::{
        Foundation::HANDLE,
        Graphics::Direct3D12::ID3D12Fence,
        System::Threading::{CreateEventA, WaitForSingleObject},
    },
};

use crate::{create_type, error::DxError, impl_trait, HasInterface};

pub trait FenceInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12Fence>>
{
    fn set_event_on_completion(&self, event: Event, value: u64) -> Result<(), DxError>;
    fn get_completed_value(&self) -> u64;
    fn signal(&self, value: u64) -> Result<(), DxError>;
}

create_type! { Fence wrap ID3D12Fence }

impl_trait! {
    impl FenceInterface =>
    Fence;

    fn set_event_on_completion(&self, event: Event, value: u64) -> Result<(), DxError> {
        unsafe { self.0.SetEventOnCompletion(value, event.0).map_err(|_| DxError::Dummy)? }

        Ok(())
    }

    fn get_completed_value(&self) -> u64 {
        unsafe { self.0.GetCompletedValue() }
    }

    fn signal(&self, value: u64) -> Result<(), DxError> {
        unsafe { self.0.Signal(value).map_err(|_| DxError::Dummy)? }

        Ok(())
    }
}

bitflags::bitflags! {
    pub struct FenceFlags: i32 {

    }
}

#[derive(Copy, Clone)]
pub struct Event(pub(crate) HANDLE);
impl Event {
    pub fn create(manual_reset: bool, initial_state: bool) -> Result<Self, DxError> {
        Ok(Event(unsafe {
            CreateEventA(None, manual_reset, initial_state, None).map_err(|_| DxError::Dummy)?
        }))
    }

    pub fn wait(&self, timeout_ms: u32) -> u32 {
        unsafe { WaitForSingleObject(self.0, timeout_ms).0 }
    }
}
