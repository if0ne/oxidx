use std::ptr::NonNull;

use oxidx::dx::*;

use crate::utils::ConstantBufferData;

#[derive(Debug)]
pub struct UploadBuffer<const IS_CONSTANT: bool, T: Clone + Copy> {
    buffer: Resource,
    mapped_data: NonNull<T>,
    element_byte_size: usize,
}

impl<T: Clone + Copy> UploadBuffer<true, T> {
    pub fn new(device: &Device, count: usize) -> Self {
        let element_byte_size = size_of::<ConstantBufferData<T>>();
        Self::new_inner(device, count, element_byte_size)
    }
}

impl<T: Clone + Copy> UploadBuffer<false, T> {
    pub fn new(device: &Device, count: usize) -> Self {
        let element_byte_size = size_of::<T>();
        Self::new_inner(device, count, element_byte_size)
    }
}

impl<const IS_CONSTANT: bool, T: Clone + Copy> UploadBuffer<IS_CONSTANT, T> {
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
            element_byte_size,
        }
    }

    pub fn resource(&self) -> &Resource {
        &self.buffer
    }

    pub fn copy_data(&self, index: usize, data: &T) {
        unsafe {
            std::ptr::write(
                self.mapped_data
                    .add(index * self.element_byte_size)
                    .as_mut(),
                *data,
            )
        }
    }
}

impl<const IS_CONSTANT: bool, T: Clone + Copy> Drop for UploadBuffer<IS_CONSTANT, T> {
    fn drop(&mut self) {
        self.buffer.unmap(0, None);
    }
}
