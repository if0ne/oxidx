#![allow(private_bounds)]
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

pub mod adapter;
pub mod command_allocator;
pub mod command_list;
pub mod command_queue;
pub mod command_signature;
pub mod debug;
pub mod descriptor_heap;
pub mod device;
pub mod error;
pub mod factory;
pub mod heap;
pub mod pso;
pub mod resources;
pub mod swapchain;
pub mod sync;
pub mod types;

pub(crate) mod pix;

mod conv;
mod utils;

pub(crate) trait FeatureObject: Default {
    const TYPE: types::FeatureType;
    type Raw: Default;

    fn as_raw(&self) -> Self::Raw;
    fn from_raw(raw: Self::Raw) -> Self;
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
