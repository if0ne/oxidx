use windows::{
    core::{IUnknown, Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{
    command_list::ICommandList,
    create_type,
    error::DxError,
    heap::IHeap,
    impl_trait,
    resources::IResource,
    sync::IFence,
    types::{CommandQueueDesc, TileRangeFlags, TileRegionSize, TiledResourceCoordinate},
    HasInterface,
};

/// Provides methods for submitting command lists, synchronizing command list execution, instrumenting the command queue, and updating resource tile mappings.
///
/// For more information: [`ID3D12CommandQueue interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12commandqueue)
pub trait ICommandQueue: for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<IUnknown>> {
    /// Marks the start of a user-defined region of work.
    #[cfg(feature = "pix")]
    fn begin_event(&self, color: impl Into<u64>, label: impl AsRef<std::ffi::CStr>);

    /// Copies mappings from a source reserved resource to a destination reserved resource.
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

    /// Marks the end of a user-defined region of work.
    #[cfg(feature = "pix")]
    fn end_event(&self);

    /// Submits an iterator of command lists for execution.
    ///
    /// For more information: [`ID3D12CommandQueue::ExecuteCommandLists method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-executecommandlists)
    fn execute_command_lists<CL: ICommandList>(&self, command_lists: &[Option<CL>]);

    /// This method samples the CPU and GPU timestamp counters at the same moment in time.
    ///
    /// For more information: [`ID3D12CommandQueue::GetClockCalibration method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-getclockcalibration)
    fn get_clock_calibration(&self) -> Result<(u64, u64), DxError>;

    /// Gets the description of the command queue.
    ///
    /// For more information: [`ID3D12CommandQueue::GetDesc method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-getdesc)
    fn get_desc(&self) -> CommandQueueDesc;

    /// This method is used to determine the rate at which the GPU timestamp counter increments.
    fn get_timestamp_frequency(&self) -> Result<u64, DxError>;

    /// Inserts a user-defined marker into timeline.
    #[cfg(feature = "pix")]
    fn set_marker(&self, color: impl Into<u64>, label: impl AsRef<std::ffi::CStr>);

    /// Updates a fence to a specified value.
    ///
    /// For more information: [`ID3D12CommandQueue::Signal method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-signal)
    fn signal(&self, fence: &impl IFence, value: u64) -> Result<(), DxError>;

    /// Updates mappings of tile locations in reserved resources to memory locations in a resource heap.
    ///
    /// For more information: [`ID3D12CommandQueue::UpdateTileMappings method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-updatetilemappings)
    fn update_tile_mappings(
        &self,
        resource: &impl IResource,
        resource_region_start_coordinates: Option<&[TiledResourceCoordinate]>,
        resource_region_sizes: Option<&[TileRegionSize]>,
        heap: &impl IHeap,
        range_flags: Option<&[TileRangeFlags]>,
        heap_range_start_offsets: Option<&[u32]>,
        range_tile_counts: Option<&[u32]>,
    );

    /// Queues a GPU-side wait, and returns immediately. A GPU-side wait is where the GPU waits until the specified fence reaches or exceeds the specified value.
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

     #[cfg(feature = "pix")]
    fn begin_event(&self, color: impl Into<u64>, label: impl AsRef<std::ffi::CStr>) {
        unsafe {
            let color = color.into();
            let label = windows::core::PCSTR::from_raw(label.as_ref().as_ptr() as *const _);

            (crate::pix::WIN_PIX_EVENT_RUNTIME.begin_event_cmd_queue)(std::mem::transmute_copy(&self.0), color, label);
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
        unsafe {
            self.0.CopyTileMappings(
                dst_resource.as_raw_ref(),
                &dst_region_start_coordinate.0,
                src_resource.as_raw_ref(),
                &src_region_start_coordinate.0,
                &region_size.0,
                D3D12_TILE_MAPPING_FLAG_NONE,
            );
        }
    }

     #[cfg(feature = "pix")]
    fn end_event(&self) {
        unsafe {
            (crate::pix::WIN_PIX_EVENT_RUNTIME.end_event_cmd_queue)(std::mem::transmute_copy(&self.0));
        }
    }

    fn execute_command_lists<CL: ICommandList>(
        &self,
        command_lists: &[Option<CL>],
    ) {
        unsafe {
            let command_lists = std::slice::from_raw_parts(command_lists.as_ptr() as *const _, command_lists.len());
            self.0.ExecuteCommandLists(command_lists)
        }
    }

    fn get_clock_calibration(&self) -> Result<(u64, u64), DxError> {
        unsafe {
            let mut gpu = 0;
            let mut cpu = 0;

            self.0
                .GetClockCalibration(&mut gpu, &mut cpu)
                .map_err(DxError::from)?;

            Ok((gpu, cpu))
        }
    }

    fn get_desc(&self) -> CommandQueueDesc {
        unsafe { CommandQueueDesc(self.0.GetDesc()) }
    }

    fn get_timestamp_frequency(&self) -> Result<u64, DxError> {
        unsafe { self.0.GetTimestampFrequency().map_err(DxError::from) }
    }

     #[cfg(feature = "pix")]
    fn set_marker(&self, color: impl Into<u64>, label: impl AsRef<std::ffi::CStr>) {
        unsafe {
            let color = color.into();
            let label = windows::core::PCSTR::from_raw(label.as_ref().as_ptr() as *const _);

            (crate::pix::WIN_PIX_EVENT_RUNTIME.set_marker_cmd_queue)(std::mem::transmute_copy(&self.0), color, label);
        }
    }

    fn signal(&self, fence: &impl IFence, value: u64) -> Result<(), DxError> {
        unsafe {
            self.0
                .Signal(fence.as_raw_ref(), value)
                .map_err(DxError::from)
        }
    }

    fn update_tile_mappings(
        &self,
        resource: &impl IResource,
        resource_region_start_coordinates: Option<&[TiledResourceCoordinate]>,
        resource_region_sizes: Option<&[TileRegionSize]>,
        heap: &impl IHeap,
        range_flags: Option<&[TileRangeFlags]>,
        heap_range_start_offsets: Option<&[u32]>,
        range_tile_counts: Option<&[u32]>,
    ) {
        unsafe {
            let regions_size = resource_region_start_coordinates.map(|r| r.len())
            .or_else(|| resource_region_sizes.map(|r| r.len()))
            .unwrap_or_default();

            let range_size = range_flags.map(|r| r.len())
                .or_else(|| heap_range_start_offsets.map(|r| r.len()))
                .or_else(|| range_tile_counts.map(|r| r.len()))
                .unwrap_or_default();

            let resource_region_start_coordinates = resource_region_start_coordinates.map(|r| r.as_ptr() as *const _);
            let resource_region_sizes = resource_region_sizes.map(|r| r.as_ptr() as *const _);
            let range_flags = range_flags.map(|r| r.as_ptr() as *const _);
            let heap_range_start_offsets = heap_range_start_offsets.map(|r| r.as_ptr());
            let range_tile_counts = range_tile_counts.map(|r| r.as_ptr());

            self.0.UpdateTileMappings(
                resource.as_raw_ref(),
                regions_size as u32,
                resource_region_start_coordinates,
                resource_region_sizes,
                heap.as_raw_ref(),
                range_size as u32,
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
