use std::{
    fs::File,
    io::{BufReader, Read, Seek},
    path::Path,
};

use oxidx::dx::*;

#[repr(align(256))]
pub struct ConstantBufferData<T>(pub T);

pub fn create_default_buffer<T>(
    device: &Device,
    cmd_list: &GraphicsCommandList,
    data: &[T],
) -> (Resource, Resource) {
    let default_buffer = device
        .create_committed_resource(
            &HeapProperties::default(),
            HeapFlags::empty(),
            &ResourceDesc::buffer(data.len()),
            ResourceStates::Common,
            None,
        )
        .unwrap();

    let upload_buffer = device
        .create_committed_resource(
            &HeapProperties::upload(),
            HeapFlags::empty(),
            &ResourceDesc::buffer(data.len()),
            ResourceStates::GenericRead,
            None,
        )
        .unwrap();

    let subresource_data = SubresourceData::new(data);

    cmd_list.resource_barrier(&[ResourceBarrier::transition(
        &default_buffer,
        ResourceStates::Common,
        ResourceStates::CopyDest,
    )]);
    cmd_list.update_subresources_fixed::<1, _, _>(
        &default_buffer,
        &upload_buffer,
        0,
        0..1,
        &[subresource_data],
    );
    cmd_list.resource_barrier(&[ResourceBarrier::transition(
        &default_buffer,
        ResourceStates::CopyDest,
        ResourceStates::GenericRead,
    )]);

    (default_buffer, upload_buffer)
}

pub fn load_binary(filename: impl AsRef<Path>) -> Blob {
    let mut file = File::open(filename).unwrap();
    let _ = file.seek(std::io::SeekFrom::Start(0));
    let size = file.seek(std::io::SeekFrom::End(0)).unwrap() as usize;

    let mut reader = BufReader::new(file);
    let _ = reader.seek(std::io::SeekFrom::Start(0));

    let blob = Blob::create_blob(size).unwrap();

    let buffer = unsafe {
        std::slice::from_raw_parts_mut(blob.get_buffer_ptr().as_mut() as *mut () as *mut u8, size)
    };

    let _ = reader.read(buffer);

    blob
}
