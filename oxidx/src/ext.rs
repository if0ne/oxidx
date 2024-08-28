use crate::dx::*;

pub fn memcpy_subresource<T: Copy>(
    dst: &mut MemcpyDest<'_, T>,
    src: &SubresourceData<'_, T>,
    row_size: usize,
    num_rows: usize,
    num_slices: usize,
) {
    let dst_slice = dst.as_slice_mut(num_slices);
    let src_slice = src.as_slice(num_slices);

    for z in 0..num_slices {
        let dst_slice = &mut dst_slice[(z * dst.slice_pitch())..((z + 1) * dst.slice_pitch())];
        let src_slice = &src_slice[(z * src.slice_pitch())..((z + 1) * src.slice_pitch())];

        for y in 0..num_rows {
            let dst_slice = &mut dst_slice[(y * dst.row_pitch())..((y + 1) * dst.row_pitch())];
            let src_slice = &src_slice[(y * src.row_pitch())..((y + 1) * src.row_pitch())];

            let dst_slice = &mut dst_slice[..row_size];
            let src_slice = &src_slice[..row_size];

            dst_slice.copy_from_slice(src_slice);
        }
    }
}
