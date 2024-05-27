#![allow(private_bounds)]
#![allow(dead_code)]

pub mod adapter;
pub mod command_allocator;
pub mod command_list;
pub mod command_queue;
pub mod debug;
pub mod device;
pub mod error;
pub mod factory;
pub mod misc;
pub mod resources;
pub mod swapchain;
pub mod sync;

mod conv;
mod utils;

pub(crate) trait HasInterface {
    type Raw;
    type RawRef<'a>
    where
        Self: 'a;

    fn new(raw: Self::Raw) -> Self;
    fn as_raw(&self) -> &Self::Raw;
    fn as_raw_ref(&self) -> Self::RawRef<'_>;
}
