use crate::dx::*;

pub fn memcpy_subresource<T>(
    dst: &MemcpyDest<'_, T>,
    src: &SubresourceData<'_, T>,
    row_size: usize,
    num_rows: u32,
    num_slices: u32,
) {
    unsafe {
        for z in 0..num_slices {
            let dst_slice = (dst.0.pData as *mut u8).add(dst.0.SlicePitch * z as usize);
            let src_slice = (src.0.pData as *const u8).add(src.0.SlicePitch as usize * z as usize);

            for y in 0..num_rows {
                std::ptr::copy_nonoverlapping(
                    src_slice.add(src.0.RowPitch as usize * y as usize),
                    dst_slice.add(dst.0.RowPitch * y as usize),
                    row_size,
                );
            }
        }
    }
}
