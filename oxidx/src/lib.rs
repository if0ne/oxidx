#![allow(private_bounds)]
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

pub mod dx;

pub(crate) mod adapter;
pub(crate) mod blob;
pub(crate) mod command_allocator;
pub(crate) mod command_list;
pub(crate) mod command_queue;
pub(crate) mod command_signature;
pub(crate) mod debug;
pub(crate) mod descriptor_heap;
pub(crate) mod device;
pub(crate) mod device_child;
pub(crate) mod error;
pub(crate) mod factory;
pub(crate) mod heap;
pub(crate) mod pageable;
pub(crate) mod pix;
pub(crate) mod pso;
pub(crate) mod query_heap;
pub(crate) mod resources;
pub(crate) mod root_signature;
pub(crate) mod swapchain;
pub(crate) mod sync;
pub(crate) mod types;

mod conv;
mod utils;

pub trait FeatureObject: __Sealed {
    const TYPE: types::FeatureType;

    type Raw: Default;
    type Input<'a>;
    type Output;

    fn into_raw(input: Self::Input<'_>) -> Self::Raw;
    fn from_raw(raw: Self::Raw) -> Self::Output;
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
