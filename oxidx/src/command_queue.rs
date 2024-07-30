use std::ffi::CStr;

use smallvec::SmallVec;
use windows::{
    core::{IUnknown, Interface, Param, PCSTR},
    Win32::Graphics::Direct3D12::*,
};

use crate::{
    command_list::ICommandList,
    create_type,
    error::DxError,
    heap::IHeap,
    impl_trait,
    pix::WIN_PIX_EVENT_RUNTIME,
    resources::IResource,
    sync::IFence,
    types::{CommandQueueDesc, TileRangeFlags, TileRegionSize, TiledResourceCoordinate},
    HasInterface,
};

/// Provides methods for submitting command lists, synchronizing command list execution, instrumenting the command queue, and updating resource tile mappings.
///
/// For more information: [`ID3D12CommandQueue interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12commandqueue)
pub trait ICommandQueue: for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<IUnknown>> {
    /// Marks the start of a user-defined region of work
    ///
    /// # Arguments
    /// * `color` - label's color
    /// * `label` - label's text
    fn begin_event(&self, color: impl Into<u64>, label: impl AsRef<CStr>);

    /// Copies mappings from a source reserved resource to a destination reserved resource.
    ///
    /// # Arguments
    /// * `dst_resource` - A reference to the destination reserved resource.
    /// * `dst_region_start_coordinate` - A reference to a [`TiledResourceCoordinate`] structure that describes the starting coordinates of the destination reserved resource.
    /// * `src_resource` - A reference to the source reserved resource.
    /// * `src_region_start_coordinate` - A reference to a [`TiledResourceCoordinate`] structure that describes the starting coordinates of the source reserved resource.
    /// * `region_size` - A reference to a [`TileRegionSize`] structure that describes the size of the reserved region.
    ///
    /// For more information: [`ID3D12CommandQueue::CopyTileMappings method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-copytilemappings)
    fn copy_tile_mappings(
        &self,
        dst_resource: &impl IResource,
        dst_region_start_coordinate: &TiledResourceCoordinate,
        src_resource: &impl IResource,
        src_region_start_coordinate: &TiledResourceCoordinate,
        region_size: &TileRegionSize,
    );

    /// Marks the end of a user-defined region of work
    fn end_event(&self);

    /// Submits an iterator of command lists for execution.
    ///
    /// # Arguments
    /// * `command_lists` - The iterator of [`CommandListInterface`] command lists to be executed.
    ///
    /// For more information: [`ID3D12CommandQueue::ExecuteCommandLists method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-executecommandlists)
    fn execute_command_lists<'cl>(
        &self,
        command_lists: impl IntoIterator<Item = &'cl (impl ICommandList + 'cl)>,
    );

    /// This method samples the CPU and GPU timestamp counters at the same moment in time.
    ///
    /// # Returns
    /// The first value in tuple is the GPU timestamp counter, the second value is the CPU timestamp counter.
    ///
    /// For more information: [`ID3D12CommandQueue::GetClockCalibration method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-getclockcalibration)
    fn get_clock_calibration(&self) -> Result<(u64, u64), DxError>;

    /// Gets the description of the command queue.
    ///
    /// # Returns
    /// The description of the command queue.
    ///
    /// For more information: [`ID3D12CommandQueue::GetDesc method `](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-getdesc)
    fn get_desc(&self) -> CommandQueueDesc;

    /// This method is used to determine the rate at which the GPU timestamp counter increments.
    ///
    /// # Returns
    /// The GPU timestamp counter frequency (in ticks/second).
    fn get_timestamp_frequency(&self) -> Result<u64, DxError>;

    /// Inserts a user-defined marker into timeline
    ///
    /// # Arguments
    /// * `color` - label's color
    /// * `label` - label's text
    fn set_marker(&self, color: impl Into<u64>, label: impl AsRef<CStr>);

    /// Updates a fence to a specified value.
    ///
    /// # Arguments
    /// * `fence` - A reference to the [`FenceInterface`] object.
    /// * `value` - The value to set the fence to.
    ///
    /// For more information: [`ID3D12CommandQueue::Signal method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-signal)
    fn signal(&self, fence: &impl IFence, value: u64) -> Result<(), DxError>;

    /// Updates mappings of tile locations in reserved resources to memory locations in a resource heap.
    ///
    /// # Arguments
    /// * `resource` - A reference to the reserved resource.
    /// * `resource_region_start_coordinates` - An array of [`TiledResourceCoordinate`] structures that describe the starting coordinates of the reserved resource regions.
    /// * `resource_region_sizes` - An array of [`TileRegionSize`] structures that describe the sizes of the reserved resource regions.
    /// * `heap` - A reference to the resource heap.
    /// * `range_flags` - A pointer to an array of [`TileRangeFlags`] values that describes each tile range.
    /// * `heap_range_start_offsets` - An array of offsets into the resource heap. These are 0-based tile offsets, counting in tiles (not bytes).
    /// * `range_tile_counts` - An array of tiles. An array of values that specify the number of tiles in each tile range.
    ///
    /// For more information: [`ID3D12CommandQueue::UpdateTileMappings method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-updatetilemappings)
    fn update_tile_mappings<const REGIONS: usize, const RANGES: usize>(
        &self,
        resource: &impl IResource,
        resource_region_start_coordinates: Option<[impl AsRef<TiledResourceCoordinate>; REGIONS]>,
        resource_region_sizes: Option<[impl AsRef<TileRegionSize>; REGIONS]>,
        heap: &impl IHeap,
        range_flags: Option<[impl AsRef<TileRangeFlags>; RANGES]>,
        heap_range_start_offsets: Option<[u32; RANGES]>,
        range_tile_counts: Option<[u32; RANGES]>,
    );

    /// Queues a GPU-side wait, and returns immediately. A GPU-side wait is where the GPU waits until the specified fence reaches or exceeds the specified value.
    ///
    /// # Arguments
    /// * `fence` - A reference to the [`FenceInterface`] object.
    /// * `value` - The value that the command queue is waiting for the fence to reach or exceed. So when [`FenceInterface::get_completed_value`] is greater than or equal to Value, the wait is terminated.
    ///
    /// For more information: [`ID3D12CommandQueue::Wait method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-wait)
    fn wait(&self, fence: &impl IFence, value: u64) -> Result<(), DxError>;
}

create_type! {
    /// Provides methods for submitting command lists, synchronizing command list execution, instrumenting the command queue, and updating resource tile mappings.
    ///
    /// For more information: [`ID3D12CommandQueue interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12commandqueue)
    CommandQueue wrap ID3D12CommandQueue
}

impl_trait! {
    impl ICommandQueue =>
    CommandQueue;

    fn begin_event(&self, color: impl Into<u64>, label: impl AsRef<CStr>) {
        unsafe {
            let color = color.into();
            let label = PCSTR::from_raw(label.as_ref().as_ptr() as *const _);

            (WIN_PIX_EVENT_RUNTIME.begin_event_cmd_queue)(std::mem::transmute_copy(&self.0), color, label);
        }
    }

    fn copy_tile_mappings(
        &self,
        dst_resource: &impl IResource,
        dst_region_start_coordinate: &TiledResourceCoordinate,
        src_resource: &impl IResource,
        src_region_start_coordinate: &TiledResourceCoordinate,
        region_size: &TileRegionSize,
    ) {
        let dst_region_start_coordinate = dst_region_start_coordinate.as_raw();
        let src_region_start_coordinate = src_region_start_coordinate.as_raw();
        let region_size = region_size.as_raw();

        unsafe {
            self.0.CopyTileMappings(
                dst_resource.as_raw_ref(),
                &dst_region_start_coordinate,
                src_resource.as_raw_ref(),
                &src_region_start_coordinate,
                &region_size,
                D3D12_TILE_MAPPING_FLAG_NONE,
            );
        }
    }

    fn end_event(&self) {
        unsafe {
            (WIN_PIX_EVENT_RUNTIME.end_event_cmd_queue)(std::mem::transmute_copy(&self.0));
        }
    }

    fn execute_command_lists<'cl>(
        &self,
        command_lists: impl IntoIterator<Item = &'cl (impl ICommandList + 'cl)>,
    ) {
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
            self.0
                .GetClockCalibration(&mut gpu, &mut cpu)
                .map_err(DxError::from)?;
        }

        Ok((gpu, cpu))
    }

    fn get_desc(&self) -> CommandQueueDesc {
        unsafe { self.0.GetDesc().into() }
    }

    fn get_timestamp_frequency(&self) -> Result<u64, DxError> {
        unsafe { self.0.GetTimestampFrequency().map_err(DxError::from) }
    }

    fn set_marker(&self, color: impl Into<u64>, label: impl AsRef<CStr>) {
        unsafe {
            let color = color.into();
            let label = PCSTR::from_raw(label.as_ref().as_ptr() as *const _);

            (WIN_PIX_EVENT_RUNTIME.set_marker_cmd_queue)(std::mem::transmute_copy(&self.0), color, label);
        }
    }

    fn signal(&self, fence: &impl IFence, value: u64) -> Result<(), DxError> {
        unsafe {
            self.0
                .Signal(fence.as_raw_ref(), value)
                .map_err(DxError::from)
        }
    }

    fn update_tile_mappings<const REGIONS: usize, const RANGES: usize>(
        &self,
        resource: &impl IResource,
        resource_region_start_coordinates: Option<[impl AsRef<TiledResourceCoordinate>; REGIONS]>,
        resource_region_sizes: Option<[impl AsRef<TileRegionSize>; REGIONS]>,
        heap: &impl IHeap,
        range_flags: Option<[impl AsRef<TileRangeFlags>; RANGES]>,
        heap_range_start_offsets: Option<[u32; RANGES]>,
        range_tile_counts: Option<[u32; RANGES]>,
    ) {
        let resource_region_start_coordinates = resource_region_start_coordinates.map(|r| {
            r.into_iter()
                .map(|r| r.as_ref().as_raw())
                .collect::<SmallVec<[_; REGIONS]>>()
        });
        let resource_region_start_coordinates = resource_region_start_coordinates.map(|r| r.as_ptr());

        let resource_region_sizes = resource_region_sizes.map(|r| {
            r.into_iter()
                .map(|r| r.as_ref().as_raw())
                .collect::<SmallVec<[_; REGIONS]>>()
        });
        let resource_region_sizes = resource_region_sizes.map(|r| r.as_ptr());

        let range_flags = range_flags.map(|r| {
            r.into_iter()
                .map(|r| r.as_ref().as_raw())
                .collect::<SmallVec<[_; REGIONS]>>()
        });
        let range_flags = range_flags.map(|r| r.as_ptr());

        let heap_range_start_offsets = heap_range_start_offsets.map(|r| r.as_ptr());
        let range_tile_counts = range_tile_counts.map(|r| r.as_ptr());

        unsafe {
            self.0.UpdateTileMappings(
                resource.as_raw_ref(),
                REGIONS as u32,
                resource_region_start_coordinates,
                resource_region_sizes,
                heap.as_raw_ref(),
                RANGES as u32,
                range_flags,
                heap_range_start_offsets,
                range_tile_counts,
                D3D12_TILE_MAPPING_FLAG_NONE,
            );
        }
    }

    fn wait(&self, fence: &impl IFence, value: u64) -> Result<(), DxError> {
        unsafe {
            self.0
                .Wait(fence.as_raw_ref(), value)
                .map_err(DxError::from)
        }
    }
}
