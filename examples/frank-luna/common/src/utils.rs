use std::{
    fs::File,
    io::{BufReader, Read, Seek},
    ops::Deref,
    path::Path,
};

use oxidx::dx::*;

use crate::texture::Texture;

#[derive(Clone, Copy, Debug)]
#[repr(align(256))]
pub struct ConstantBufferData<T>(pub T);

impl<T> Deref for ConstantBufferData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn create_default_buffer<T: Copy>(
    device: &Device,
    cmd_list: &GraphicsCommandList,
    data: &[T],
) -> (Resource, Resource) {
    let default_buffer = device
        .create_committed_resource(
            &HeapProperties::default(),
            HeapFlags::empty(),
            &ResourceDesc::buffer(size_of_val(data)),
            ResourceStates::Common,
            None,
        )
        .unwrap();

    let upload_buffer = device
        .create_committed_resource(
            &HeapProperties::upload(),
            HeapFlags::empty(),
            &ResourceDesc::buffer(size_of_val(data)),
            ResourceStates::GenericRead,
            None,
        )
        .unwrap();

    let subresource_data = SubresourceData::new(data);

    cmd_list.resource_barrier(&[ResourceBarrier::transition(
        &default_buffer,
        ResourceStates::Common,
        ResourceStates::CopyDest,
        None,
    )]);

    assert!(
        cmd_list.update_subresources_fixed::<1, _, _>(
            &default_buffer,
            &upload_buffer,
            0,
            0..1,
            &[subresource_data],
        ) > 0
    );

    cmd_list.resource_barrier(&[ResourceBarrier::transition(
        &default_buffer,
        ResourceStates::CopyDest,
        ResourceStates::GenericRead,
        None,
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

pub fn load_texture_from_file(
    device: &Device,
    cmd_list: &GraphicsCommandList,
    name: impl Into<String>,
    filename: impl AsRef<Path>,
) -> Result<Texture, DxError> {
    let filename = filename.as_ref().to_path_buf();
    let img = image::open(&filename)
        .map_err(|e| DxError::Fail(e.to_string()))?
        .to_rgba8();

    let texture_bytes = img.as_raw();

    let desc = ResourceDesc::texture_2d(img.width(), img.height()).with_format(Format::Rgba8Unorm);

    let resource = device.create_committed_resource(
        &HeapProperties::default(),
        HeapFlags::empty(),
        &desc,
        ResourceStates::CopyDest,
        None,
    )?;

    let total_size = device.get_copyable_footprints(&desc, 0..1, 0, None, None, None);

    let upload_buffer = device.create_committed_resource(
        &HeapProperties::upload(),
        HeapFlags::empty(),
        &ResourceDesc::buffer(total_size as usize).with_layout(TextureLayout::RowMajor),
        ResourceStates::GenericRead,
        None,
    )?;

    let subresource_data = SubresourceData::new(texture_bytes)
        .with_row_pitch(4 * img.width() as usize)
        .with_slice_pitch(4 * (img.width() * img.height()) as usize);

    assert!(
        cmd_list.update_subresources_fixed::<1, _, _>(
            &resource,
            &upload_buffer,
            0,
            0..1,
            &[subresource_data],
        ) > 0
    );

    cmd_list.resource_barrier(&[ResourceBarrier::transition(
        &resource,
        ResourceStates::CopyDest,
        ResourceStates::PixelShaderResource,
        None,
    )]);

    Ok(Texture {
        name: name.into(),
        filename,
        image: resource,
        upload_buffer,
    })
}
