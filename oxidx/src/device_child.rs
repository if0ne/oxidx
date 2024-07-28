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

pub trait IDeviceChild:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12DeviceChild>>
{
}

create_type!(
    DeviceChild wrap ID3D12DeviceChild
);

impl_trait! {
    impl IDeviceChild => DeviceChild;
}

impl_up_down_cast!(Heap inherit DeviceChild);
impl_up_down_cast!(Resource inherit DeviceChild);
impl_up_down_cast!(Fence inherit DeviceChild);
impl_up_down_cast!(Fence1 inherit DeviceChild);
