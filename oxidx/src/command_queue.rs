use windows::Win32::Graphics::Direct3D12::*;

use crate::{
    create_type,
    dx::{Fence, GraphicsCommandList, Heap, Resource},
    error::DxError,
    impl_interface,
    types::{CommandQueueDesc, TileRangeFlags, TileRegionSize, TiledResourceCoordinate},
};

create_type! {
    /// Provides methods for submitting command lists, synchronizing command list execution, instrumenting the command queue, and updating resource tile mappings.
    ///
    /// For more information: [`ID3D12CommandQueue interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12commandqueue)
    CommandQueue wrap ID3D12CommandQueue
}

impl_interface! {
    CommandQueue;

    /// Marks the start of a user-defined region of work.
     #[cfg(feature = "pix")]
    pub fn begin_event(&self, color: impl Into<u64>, label: impl AsRef<std::ffi::CStr>) {
        unsafe {
            let color = color.into();
            let label = windows::core::PCSTR::from_raw(label.as_ref().as_ptr() as *const _);

            (crate::pix::WIN_PIX_EVENT_RUNTIME.begin_event_cmd_queue)(std::mem::transmute_copy(&self.0), color, label);
        }
    }

    /// Copies mappings from a source reserved resource to a destination reserved resource.
    ///
    /// For more information: [`ID3D12CommandQueue::CopyTileMappings method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-copytilemappings)
    pub fn copy_tile_mappings(
        &self,
        dst_resource: impl AsRef<Resource>,
        dst_region_start_coordinate: &TiledResourceCoordinate,
        src_resource: impl AsRef<Resource>,
        src_region_start_coordinate: &TiledResourceCoordinate,
        region_size: &TileRegionSize,
    ) {
        unsafe {
            self.0.CopyTileMappings(
                &dst_resource.as_ref().0,
                &dst_region_start_coordinate.0,
                &src_resource.as_ref().0,
                &src_region_start_coordinate.0,
                &region_size.0,
                D3D12_TILE_MAPPING_FLAG_NONE,
            );
        }
    }

    /// Marks the end of a user-defined region of work.
    #[cfg(feature = "pix")]
    pub fn end_event(&self) {
        unsafe {
            (crate::pix::WIN_PIX_EVENT_RUNTIME.end_event_cmd_queue)(std::mem::transmute_copy(&self.0));
        }
    }

    /// Submits an iterator of command lists for execution.
    ///
    /// For more information: [`ID3D12CommandQueue::ExecuteCommandLists method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-executecommandlists)
    pub fn execute_command_lists(
        &self,
        command_lists: &[Option<GraphicsCommandList>],
    ) {
        unsafe {
            let command_lists = std::slice::from_raw_parts(command_lists.as_ptr() as *const _, command_lists.len());
            self.0.ExecuteCommandLists(command_lists)
        }
    }

    /// This method samples the CPU and GPU timestamp counters at the same moment in time.
    ///
    /// For more information: [`ID3D12CommandQueue::GetClockCalibration method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-getclockcalibration)
    pub fn get_clock_calibration(&self) -> Result<(u64, u64), DxError> {
        unsafe {
            let mut gpu = 0;
            let mut cpu = 0;

            self.0
                .GetClockCalibration(&mut gpu, &mut cpu)
                .map_err(DxError::from)?;

            Ok((gpu, cpu))
        }
    }

    /// Gets the description of the command queue.
    ///
    /// For more information: [`ID3D12CommandQueue::GetDesc method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-getdesc)
    pub fn get_desc(&self) -> CommandQueueDesc {
        unsafe { CommandQueueDesc(self.0.GetDesc()) }
    }

    /// This method is used to determine the rate at which the GPU timestamp counter increments.
    pub fn get_timestamp_frequency(&self) -> Result<u64, DxError> {
        unsafe { self.0.GetTimestampFrequency().map_err(DxError::from) }
    }

    /// Inserts a user-defined marker into timeline.
    #[cfg(feature = "pix")]
    pub fn set_marker(&self, color: impl Into<u64>, label: impl AsRef<std::ffi::CStr>) {
        unsafe {
            let color = color.into();
            let label = windows::core::PCSTR::from_raw(label.as_ref().as_ptr() as *const _);

            (crate::pix::WIN_PIX_EVENT_RUNTIME.set_marker_cmd_queue)(std::mem::transmute_copy(&self.0), color, label);
        }
    }

    /// Updates a fence to a specified value.
    ///
    /// For more information: [`ID3D12CommandQueue::Signal method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-signal)
    pub fn signal(&self, fence: impl AsRef<Fence>, value: u64) -> Result<(), DxError> {
        unsafe {
            self.0
                .Signal(&fence.as_ref().0, value)
                .map_err(DxError::from)
        }
    }

    /// Updates mappings of tile locations in reserved resources to memory locations in a resource heap.
    ///
    /// For more information: [`ID3D12CommandQueue::UpdateTileMappings method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-updatetilemappings)
    pub fn update_tile_mappings(
        &self,
        resource: impl AsRef<Resource>,
        resource_region_start_coordinates: Option<&[TiledResourceCoordinate]>,
        resource_region_sizes: Option<&[TileRegionSize]>,
        heap: impl AsRef<Heap>,
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
                &resource.as_ref().0,
                regions_size as u32,
                resource_region_start_coordinates,
                resource_region_sizes,
                &heap.as_ref().0,
                range_size as u32,
                range_flags,
                heap_range_start_offsets,
                range_tile_counts,
                D3D12_TILE_MAPPING_FLAG_NONE,
            );
        }
    }

    /// Queues a GPU-side wait, and returns immediately. A GPU-side wait is where the GPU waits until the specified fence reaches or exceeds the specified value.
    ///
    /// For more information: [`ID3D12CommandQueue::Wait method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-wait)
    pub fn wait(&self, fence: impl AsRef<Fence>, value: u64) -> Result<(), DxError> {
        unsafe {
            self.0
                .Wait(&fence.as_ref().0, value)
                .map_err(DxError::from)
        }
    }
}
