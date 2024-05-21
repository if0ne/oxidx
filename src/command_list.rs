use windows::{core::Interface, Win32::Graphics::Direct3D12::ID3D12GraphicsCommandList};

use crate::{create_type, impl_trait, HasInterface};

pub trait CommandListInterface: HasInterface<Raw: Interface> {}

pub trait GraphicsCommandListInterface: CommandListInterface {}

create_type! { GraphicsCommandList wrap ID3D12GraphicsCommandList }

impl_trait! {
    impl CommandListInterface =>
    GraphicsCommandList;
}

impl_trait! {
    impl GraphicsCommandListInterface =>
    GraphicsCommandList;
}
