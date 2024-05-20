use windows::{
    core::{IUnknown, Interface, Param},
    Win32::Graphics::Direct3D12::ID3D12CommandQueue,
};

use crate::{create_type, impl_trait, HasInterface};

#[allow(dead_code)]
pub trait CommandQueueInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<IUnknown>>
{
}

create_type! { CommandQueue wrap ID3D12CommandQueue }

impl_trait! {
    impl CommandQueueInterface =>
    CommandQueue;
}
