use windows::Win32::Graphics::Direct3D12::ID3D12CommandQueue;

use crate::{create_type, HasInterface};

#[allow(dead_code)]
pub(crate) trait CommandQueueInterface: HasInterface {}

create_type! { CommandQueueInterface => CommandQueue wrap ID3D12CommandQueue; decorator for }
