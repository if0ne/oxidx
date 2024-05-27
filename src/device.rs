use windows::{core::Interface, Win32::Graphics::Direct3D12::ID3D12Device};

use crate::{
    command_allocator::CommandAllocatorInterface,
    command_queue::{CommandQueueDesc, CommandQueueInterface},
    create_type,
    error::DxError,
    heap::{DescriptorHeapDesc, DescriptorHeapInterface, DescriptorHeapType},
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

    fn create_command_queue<CQ: CommandQueueInterface>(
        &self,
        desc: CommandQueueDesc,
    ) -> Result<CQ, DxError>;

    fn create_fence<F: FenceInterface>(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> Result<F, DxError>;

    fn create_descriptor_heap<H: DescriptorHeapInterface>(
        &self,
        desc: DescriptorHeapDesc,
    ) -> Result<H, DxError>;

    fn get_descriptor_handle_increment_size(&self, r#type: DescriptorHeapType) -> u32;
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

    fn create_command_queue<CQ: CommandQueueInterface>(
        &self,
        desc: CommandQueueDesc,
    ) -> Result<CQ, DxError> {
        let res: CQ::Raw  = unsafe {
            self.0.CreateCommandQueue(&desc.as_raw()).map_err(|_| DxError::Dummy)?
        };

        Ok(CQ::new(res))
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

    fn create_descriptor_heap<H: DescriptorHeapInterface>(
        &self,
        desc: DescriptorHeapDesc,
    ) -> Result<H, DxError> {
        let res: H::Raw  = unsafe {
            self.0.CreateDescriptorHeap(&desc.as_raw()).map_err(|_| DxError::Dummy)?
        };

        Ok(H::new(res))
    }

    fn get_descriptor_handle_increment_size(&self, r#type: DescriptorHeapType) -> u32 {
        unsafe {
            self.0.GetDescriptorHandleIncrementSize(r#type.as_raw())
        }
    }
}
