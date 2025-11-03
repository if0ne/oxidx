use std::ffi::CStr;

use windows::{
    core::Interface,
    Win32::Graphics::{Direct3D::WKPDID_D3DDebugObjectName, Direct3D12::ID3D12DeviceChild},
};

use crate::{
    create_type,
    dx::{Device, DxError},
    heap::Heap,
    impl_interface, impl_up_down_cast,
    resources::Resource,
    sync::{Fence, Fence1},
};

create_type!(
    /// An interface from which other core interfaces inherit from, including (but not limited to)
    /// ID3D12PipelineLibrary, [`ICommandList`](crate::command_list::ICommandList), [`IPageable`](crate::pageable::IPageable), and [`IRootSignature`](crate::root_signature::IRootSignature).
    /// It provides a method to get back to the device object it was created against.
    ///
    /// For more information: [`ID3D12DeviceChild interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12devicechild)
    DeviceChild wrap ID3D12DeviceChild
);

impl_interface! {
    DeviceChild,
    Heap,
    Resource,
    Fence,
    Fence1;

    /// Gets a pointer to the device that created this interface.
    ///
    /// For more information: [`ID3D12DeviceChild::GetDevice interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12devicechild-getdevice)
    pub fn get_device(&self) -> Result<Device, DxError> {
        unsafe {
            let mut raw = None;
            self.0.GetDevice(&mut raw).map_err(DxError::from)?;

            Ok(Device(raw.unwrap()))
        }
    }
}

impl_interface! {
    DeviceChild,
    Heap,
    Resource,
    Fence,
    Fence1;

    pub fn set_debug_object_name(&self, name: &CStr) -> Result<(), DxError> {
        unsafe {
            self.0.SetPrivateData(&WKPDID_D3DDebugObjectName, name.to_bytes().len() as u32, Some(name.as_ptr() as *const _))
                .map_err(DxError::from)
        }
    }
}

impl_up_down_cast!(Heap inherit DeviceChild);
impl_up_down_cast!(Resource inherit DeviceChild);
impl_up_down_cast!(Fence inherit DeviceChild);
impl_up_down_cast!(Fence1 inherit DeviceChild);
