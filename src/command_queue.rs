use windows::{
    core::{IUnknown, Interface, Param},
    Win32::Graphics::Direct3D12::ID3D12CommandQueue,
};

use crate::{create_type, HasInterface};

#[allow(dead_code)]
pub trait CommandQueueInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<IUnknown>>
{
}

create_type! { CommandQueueInterface => CommandQueue wrap ID3D12CommandQueue; decorator for }
