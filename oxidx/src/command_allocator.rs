use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::ID3D12CommandAllocator,
};

use crate::{create_type, impl_trait, prelude::DxError, HasInterface};

pub trait CommandAllocatorInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12CommandAllocator>>
{
    /// Indicates to re-use the memory that is associated with the command allocator.
    ///
    /// # Errors
    /// This method returns Err(fail) if there is an actively recording command list referencing the command allocator.
    /// The debug layer will also issue an error in this case.
    ///
    /// # Remarks
    /// Apps call [`CommandAllocatorInterface::reset`] to re-use the memory that is associated with a command allocator.
    /// From this call to Reset, the runtime and driver determine that the graphics processing unit (GPU) is no longer executing any command lists that have recorded commands with the command allocator
    ///
    /// Unlike [`CommandAllocatorInterface::reset`], it is not recommended that you call [`CommandAllocatorInterface::reset`] on the command allocator while a command list is still being executed.
    ///
    /// The debug layer will issue a warning if it can't prove that there are no pending GPU references to command lists that have recorded commands in the allocator.
    ///
    /// The debug layer will issue an error if [`CommandAllocatorInterface::reset`] is called concurrently by multiple threads (on the same allocator object).
    fn reset(&self) -> Result<(), DxError>;
}

create_type! { CommandAllocator wrap ID3D12CommandAllocator }

impl_trait! {
    impl CommandAllocatorInterface =>
    CommandAllocator;

    fn reset(&self) -> Result<(), DxError> {
        unsafe {
            self.0.Reset().map_err(|_| DxError::Fail)?;
        }

        Ok(())
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::command_allocator::CommandAllocator;
    use crate::utils::*;

    #[test]
    fn check_send_sync() {
        is_not_send::<CommandAllocator>();
        is_not_sync::<CommandAllocator>();
    }
}
*/
