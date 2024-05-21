use windows::{core::Interface, Win32::Graphics::Direct3D12::ID3D12Device};

use crate::{
    command_allocator::CommandAllocatorInterface,
    create_type,
    error::DxError,
    impl_trait,
    misc::CommandListType,
    sync::{FenceFlags, FenceInterface},
    HasInterface,
};

pub trait DeviceInterface: HasInterface<Raw: Interface> {
    fn create_command_allocator<CA: CommandAllocatorInterface>(
        &self,
        r#type: CommandListType,
    ) -> Result<CA, DxError>;

    fn create_fence<F: FenceInterface>(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> Result<F, DxError>;
}

create_type! { Device wrap ID3D12Device }

impl_trait! {
    impl DeviceInterface =>
    Device;

    fn create_command_allocator<CA: CommandAllocatorInterface>(&self, r#type: CommandListType) -> Result<CA, DxError> {
        let res: CA::Raw  = unsafe {
            self.0.CreateCommandAllocator(r#type.as_raw()).map_err(|_| DxError::Dummy)?
        };

        Ok(CA::new(res))
    }

    fn create_fence<F: FenceInterface>(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> Result<F, DxError> {
        let res: F::Raw  = unsafe {
            self.0.CreateFence(initial_value, flags.as_raw()).map_err(|_| DxError::Dummy)?
        };

        Ok(F::new(res))
    }
}
