use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::ID3D12DeviceChild,
};

use crate::{
    create_type,
    heap::Heap,
    impl_trait,
    resources::Resource,
    sync::{Fence, Fence1},
    HasInterface,
};

pub trait DeviceChildInterface: for<'a> HasInterface<RawRef<'a>: Param<ID3D12DeviceChild>> {}

create_type!(
    DeviceChild wrap ID3D12DeviceChild
);

impl_trait! {
    impl DeviceChildInterface => DeviceChild;
}

impl From<Heap> for DeviceChild {
    fn from(value: Heap) -> Self {
        unsafe { DeviceChild::new(value.0.cast::<ID3D12DeviceChild>().unwrap_unchecked()) }
    }
}

impl From<Resource> for DeviceChild {
    fn from(value: Resource) -> Self {
        unsafe { DeviceChild::new(value.0.cast::<ID3D12DeviceChild>().unwrap_unchecked()) }
    }
}

impl From<Fence> for DeviceChild {
    fn from(value: Fence) -> Self {
        unsafe { DeviceChild::new(value.0.cast::<ID3D12DeviceChild>().unwrap_unchecked()) }
    }
}

impl From<Fence1> for DeviceChild {
    fn from(value: Fence1) -> Self {
        unsafe { DeviceChild::new(value.0.cast::<ID3D12DeviceChild>().unwrap_unchecked()) }
    }
}
