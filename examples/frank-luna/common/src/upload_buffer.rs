use std::ptr::NonNull;

use oxidx::dx::*;

#[derive(Debug)]
pub struct UploadBuffer<T: Clone + Copy> {
    buffer: Resource,
    mapped_data: NonNull<T>,
}

impl<T: Clone + Copy> UploadBuffer<T> {
    pub fn new(device: &Device, count: usize) -> Self {
        let element_byte_size = size_of::<T>();
        Self::new_inner(device, count, element_byte_size)
    }
}

impl<T: Clone + Copy> UploadBuffer<T> {
    fn new_inner(device: &Device, count: usize, element_byte_size: usize) -> Self {
        let resource: Resource = device
            .create_committed_resource(
                &HeapProperties::upload(),
                HeapFlags::empty(),
                &ResourceDesc::buffer(count * element_byte_size),
                ResourceStates::GenericRead,
                None,
            )
            .unwrap();

        let mapped_data = resource.map::<T>(0, None).unwrap();

        Self {
            buffer: resource,
            mapped_data,
        }
    }

    pub fn resource(&self) -> &Resource {
        &self.buffer
    }

    pub fn copy_data(&self, index: usize, data: impl ToOwned<Owned = T>) {
        unsafe { std::ptr::write(self.mapped_data.add(index).as_mut(), data.to_owned()) }
    }
}

impl<T: Clone + Copy> Drop for UploadBuffer<T> {
    fn drop(&mut self) {
        self.buffer.unmap(0, None);
    }
}
