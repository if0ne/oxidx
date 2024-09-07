use std::ffi::CStr;

use windows::{
    core::{Interface, Param},
    Win32::Graphics::{Direct3D::WKPDID_D3DDebugObjectName, Direct3D12::ID3D12DeviceChild},
};

use crate::{
    create_type,
    dx::{DxError, IDevice},
    heap::Heap,
    impl_trait, impl_up_down_cast,
    resources::Resource,
    sync::{Fence, Fence1},
    HasInterface,
};

/// An interface from which other core interfaces inherit from, including (but not limited to)
/// ID3D12PipelineLibrary, [`ICommandList`](crate::command_list::ICommandList), [`IPageable`](crate::pageable::IPageable), and [`IRootSignature`](crate::root_signature::IRootSignature).
/// It provides a method to get back to the device object it was created against.
///
/// For more information: [`ID3D12DeviceChild interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12devicechild)
pub trait IDeviceChild:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12DeviceChild>>
{
    /// Gets a pointer to the device that created this interface.
    ///
    /// For more information: [`ID3D12DeviceChild::GetDevice interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12devicechild-getdevice)
    fn get_device<D: IDevice>(&self) -> Result<D, DxError>;
}

pub trait IDeviceChildExt: IDeviceChild {
    fn set_debug_object_name(&self, name: &CStr) -> Result<(), DxError>;
}

create_type!(
    /// An interface from which other core interfaces inherit from, including (but not limited to)
    /// ID3D12PipelineLibrary, [`ICommandList`](crate::command_list::ICommandList), [`IPageable`](crate::pageable::IPageable), and [`IRootSignature`](crate::root_signature::IRootSignature).
    /// It provides a method to get back to the device object it was created against.
    ///
    /// For more information: [`ID3D12DeviceChild interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12devicechild)
    DeviceChild wrap ID3D12DeviceChild
);

impl_trait! {
    impl IDeviceChild =>
    DeviceChild,
    Heap,
    Resource,
    Fence,
    Fence1;

    fn get_device<D: IDevice>(&self) -> Result<D, DxError> {
        unsafe {
            let mut raw: Option<D::Raw> = None;
            self.0.GetDevice(&mut raw).map_err(DxError::from)?;

            Ok(D::new(raw.unwrap()))
        }
    }
}

impl_trait! {
    impl IDeviceChildExt =>
    DeviceChild,
    Heap,
    Resource,
    Fence,
    Fence1;

    fn set_debug_object_name(&self, name: &CStr) -> Result<(), DxError> {
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
