use smallvec::SmallVec;
use windows::{
    core::{IUnknown, Interface, Param},
    Win32::Graphics::Direct3D12::{ID3D12CommandList, ID3D12CommandQueue},
};

use crate::{
    command_list::CommandListInterface, create_type, error::DxError, impl_trait,
    misc::CommandListType, sync::Fence, HasInterface,
};

pub trait CommandQueueInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<IUnknown>>
{
    fn execute_command_lists<'cl, I, CL>(&self, command_lists: I)
    where
        I: Iterator<Item = &'cl CL>,
        CL: CommandListInterface + 'cl;
    fn signal(&self, fence: &Fence, value: u64) -> Result<(), DxError>;
}

create_type! { CommandQueue wrap ID3D12CommandQueue }

impl_trait! {
    impl CommandQueueInterface =>
    CommandQueue;

    fn execute_command_lists<'cl, I, CL>(&self, command_lists: I)
    where
        I: Iterator<Item = &'cl CL>,
        CL: CommandListInterface + 'cl,
    {
        let command_lists = command_lists
            .map(|l| {
                Some(
                    l.as_raw()
                        .cast::<ID3D12CommandList>()
                        .expect("Can not cast command list raw into ID3D12CommandList"),
                )
            })
            .collect::<SmallVec<[_; 16]>>();
        unsafe { self.0.ExecuteCommandLists(command_lists.as_slice()) }
    }

    fn signal(&self, fence: &Fence, value: u64) -> Result<(), DxError> {
        unsafe { self.0.Signal(fence.as_raw_ref(), value).map_err(|_| DxError::Dummy) }
    }
}

#[derive(Debug, Clone)]
pub struct CommandQueueDesc {
    pub r#type: CommandListType,
    pub priority: i32,
    pub flags: CommandQueueFlags,
    pub node_mask: u32,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct CommandQueueFlags: i32 {

    }
}
