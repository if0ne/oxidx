#![allow(private_bounds)]
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

pub mod dx;

pub mod adapter;
pub mod blob;
pub mod command_allocator;
pub mod command_list;
pub mod command_queue;
pub mod command_signature;
pub mod debug;
pub mod descriptor_heap;
pub mod device;
pub mod device_child;
pub mod entry;
pub mod error;
pub mod factory;
pub mod heap;
pub mod pageable;
pub mod pso;
pub mod query_heap;
pub mod resources;
pub mod root_signature;
pub mod swapchain;
pub mod sync;
pub mod types;

pub(crate) mod pix;

mod conv;
mod utils;

pub trait FeatureObject: __Sealed {
    const TYPE: types::FeatureType;
}

pub(crate) trait HasInterface {
    type Raw;
    type RawRef<'a>
    where
        Self: 'a;

    fn new(raw: Self::Raw) -> Self;
    fn as_raw(&self) -> &Self::Raw;
    fn as_raw_ref(&self) -> Self::RawRef<'_>;
}

pub(crate) trait __Sealed {}
