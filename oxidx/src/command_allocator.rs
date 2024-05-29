use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::ID3D12CommandAllocator,
};

use crate::{create_type, impl_trait, HasInterface};

pub trait CommandAllocatorInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12CommandAllocator>>
{
    fn reset(&self);
}

create_type! { CommandAllocator wrap ID3D12CommandAllocator }

impl_trait! {
    impl CommandAllocatorInterface =>
    CommandAllocator;

    fn reset(&self) {
        unsafe {
            self.0.Reset().unwrap(/*TODO: Error*/);
        }
    }
}
