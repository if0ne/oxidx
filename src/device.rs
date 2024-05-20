use windows::{core::Interface, Win32::Graphics::Direct3D12::ID3D12Device};

use crate::{create_type, impl_trait, HasInterface};

pub trait DeviceInterface: HasInterface<Raw: Interface> {}

create_type! { Device wrap ID3D12Device }

impl_trait! {
    impl DeviceInterface =>
    Device;
}
