use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::ID3D12DeviceChild,
};

use crate::{
    create_type,
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
    impl IDeviceChild => DeviceChild;
}

impl_up_down_cast!(Heap inherit DeviceChild);
impl_up_down_cast!(Resource inherit DeviceChild);
impl_up_down_cast!(Fence inherit DeviceChild);
impl_up_down_cast!(Fence1 inherit DeviceChild);
