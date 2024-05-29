use windows::{core::Interface, Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList};

use crate::{create_type, impl_trait, HasInterface};

pub trait CommandListInterface: HasInterface<Raw: Interface> {
    fn close(&self);
}

pub trait GraphicsCommandListInterface: CommandListInterface {}

create_type! { GraphicsCommandList wrap ID3D12GraphicsCommandList }

impl_trait! {
    impl CommandListInterface =>
    GraphicsCommandList;

    fn close(&self) {
        unsafe {
            self.0.Close().unwrap(/*TODO: Error*/);
        }
    }
}

impl_trait! {
    impl GraphicsCommandListInterface =>
    GraphicsCommandList;
}
