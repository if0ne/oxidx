use smallvec::SmallVec;
use windows::{
    core::{IUnknown, Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{
    command_list::CommandListInterface,
    create_type,
    error::DxError,
    impl_trait,
    resources::ResourceInterface,
    sync::Fence,
    types::{CommandListType, TileRegionSize, TiledResourceCoordinate},
    HasInterface,
};

/// Provides methods for submitting command lists, synchronizing command list execution, instrumenting the command queue, and updating resource tile mappings.
pub trait CommandQueueInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<IUnknown>>
{
    // TODO: PIX FUNCTIONS
    // fn begin_event<'a>(&self, color: impl Into<u64>, label: &'a str);
    // fn end_event(&self);
    // fn set_marker<'a>(&self, color: impl Into<u64>, label: &'a str)

    /// Copies mappings from a source reserved resource to a destination reserved resource.
    ///
    /// # Arguments
    /// * `dst_resource` - A reference to the destination reserved resource.
    /// * `dst_region_start_coordinate` - A reference to a [`TiledResourceCoordinate`] structure that describes the starting coordinates of the destination reserved resource.
    /// * `src_resource` - A reference to the source reserved resource.
    /// * `src_region_start_coordinate` - A reference to a [`TiledResourceCoordinate`] structure that describes the starting coordinates of the source reserved resource.
    /// * `region_size` - A reference to a [`TileRegionSize`] structure that describes the size of the reserved region.
    ///
    /// # Remarks
    /// Use [`copy_tile_mappings`] to copy the tile mappings from one reserved resource to another, either to duplicate a resource mapping,
    /// or to initialize a new mapping before modifying it using [`CommandQueueInterface::update_tile_mappings`].
    ///
    /// [`copy_tile_mappings`] helps with tasks such as shifting mappings around within and across reserved resources, for example, scrolling tiles.
    /// The source and destination regions can overlap; the result of the copy in this situation is as
    /// if the source was saved to a temporary location and from there written to the destination.
    ///
    /// The destination and the source regions must each entirely fit in their resource or behavior is undefined and the debug layer will emit an error.
    fn copy_tile_mappings(
        &self,
        dst_resource: &impl ResourceInterface,
        dst_region_start_coordinate: &TiledResourceCoordinate,
        src_resource: &impl ResourceInterface,
        src_region_start_coordinate: &TiledResourceCoordinate,
        region_size: &TileRegionSize,
    );

    /// Submits an iterator of command lists for execution.
    ///
    /// # Arguments
    /// * `command_lists` - The iterator of [`CommandListInterface`] command lists to be executed.
    ///
    /// # Remarks
    /// Calling [`execute_command_lists`]  twice in succession (from the same thread, or different threads) guarantees that
    /// the first workload (A) finishes before the second workload (B).
    /// Calling [`execute_command_lists`]  with two command lists allows the driver to merge the two command lists such that the second command list (D)
    /// may begin executing work before all work from the first (C) has finished.
    /// Specifically, your application is allowed to insert a fence signal or wait between A and B, and the driver has no visibility into this,
    /// so the driver must ensure that everything in A is complete before the fence operation.
    /// There is no such opportunity in a single call to the API, so the driver is able to optimize that scenario.
    ///
    /// The driver is free to patch the submitted command lists.
    /// It is the calling application’s responsibility to ensure that the graphics processing unit (GPU) is not currently reading the any of the submitted command lists from a previous execution.
    ///
    /// Applications are encouraged to batch together command list executions to reduce fixed costs associated with submitted commands to the GPU.
    ///
    /// # Runtime validation
    /// Bundles can't be submitted to a command queue directly. If a bundle is passed to this method, the runtime will drop the call.
    /// The runtime will also drop the call if the [`GraphicsCommandListInterface::close`](crate::command_list::GraphicsCommandListInterface::close) function has not been called on one or more of the command lists.
    ///
    /// The runtime will detect if the command allocators associated with the command lists have been reset after Close was called.
    /// The runtime will drop the call and remove the device in this situation.
    ///
    /// The runtime will drop the call and remove the device if the command queue fence indicates that a previous execution of any of the command lists has not yet completed.
    ///
    /// The runtime will validate the "before" and "after" states of resource transition barriers inside of [`execute_command_lists`].
    /// If the “before” state of a transition does not match up with the “after” state of a previous transition, then the runtime will drop the call and remove the device.
    ///
    /// The runtime will validate the “before” and “after” states of queries used by the command lists. If an error is detected, then the runtime will drop the call and remove the device.
    ///
    /// # Debug Layer
    /// The debug layer issues errors for all cases where the runtime would drop the call.
    ///
    /// The debug layer issues an error if it detects that any resource referenced by the command lists, including queries, has been destroyed.
    fn execute_command_lists<'cl, I, CL>(&self, command_lists: I)
    where
        I: IntoIterator<Item = &'cl CL>,
        CL: CommandListInterface + 'cl;

    /// This method samples the CPU and GPU timestamp counters at the same moment in time.
    ///
    /// # Returns
    /// * `(u64, u64)` - (The value of the GPU timestamp counter, the value of the CPU timestamp counter)
    ///
    /// # Remarks
    /// For more information, refer to [`Timing`](https://learn.microsoft.com/en-us/windows/win32/direct3d12/timing).
    fn get_clock_calibration(&self) -> Result<(u64, u64), DxError>;

    fn signal(&self, fence: &Fence, value: u64) -> Result<(), DxError>;
}

create_type! {
    /// Provides methods for submitting command lists, synchronizing command list execution, instrumenting the command queue, and updating resource tile mappings.
    ///
    /// # Remarks
    /// Use [DeviceInterface::create_command_queue](`crate::device::DeviceInterface::create_command_queue`) to create a command queue object.
    CommandQueue wrap ID3D12CommandQueue
}

impl_trait! {
    impl CommandQueueInterface =>
    CommandQueue;

    fn copy_tile_mappings(
        &self,
        dst_resource: &impl ResourceInterface,
        dst_region_start_coordinate: &TiledResourceCoordinate,
        src_resource: &impl ResourceInterface,
        src_region_start_coordinate: &TiledResourceCoordinate,
        region_size: &TileRegionSize,
    ) {
        let dst_region_start_coordinate = dst_region_start_coordinate.to_raw();
        let src_region_start_coordinate = src_region_start_coordinate.to_raw();
        let region_size = region_size.to_raw();

        unsafe {
            self.0.CopyTileMappings(
                dst_resource.as_raw_ref(),
                &dst_region_start_coordinate,
                src_resource.as_raw_ref(),
                &src_region_start_coordinate,
                &region_size,
                D3D12_TILE_MAPPING_FLAG_NONE
            );
        }
    }

    fn execute_command_lists<'cl, I, CL>(&self, command_lists: I)
    where
        I: IntoIterator<Item = &'cl CL>,
        CL: CommandListInterface + 'cl,
    {
        let command_lists = command_lists
            .into_iter()
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

    fn get_clock_calibration(&self) -> Result<(u64, u64), DxError> {
        let mut gpu = 0;
        let mut cpu = 0;

        unsafe {
            self.0.GetClockCalibration(&mut gpu, &mut cpu).map_err(DxError::from)?;
        }

        Ok((gpu, cpu))
    }

    fn signal(&self, fence: &Fence, value: u64) -> Result<(), DxError> {
        unsafe { self.0.Signal(fence.as_raw_ref(), value).map_err(|_| DxError::Dummy) }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CommandQueueDesc {
    pub r#type: CommandListType,
    pub priority: i32,
    pub flags: CommandQueueFlags,
    pub node_mask: u32,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct CommandQueueFlags: i32 {
        const DisableGpuTimeout = D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT.0;
    }
}
