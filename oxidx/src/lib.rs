#![allow(private_bounds)]
#![allow(dead_code)]

pub mod prelude;

mod adapter;
mod command_allocator;
mod command_list;
mod command_queue;
mod debug;
mod device;
mod error;
mod factory;
mod heap;
mod misc;
mod resources;
mod swapchain;
mod sync;

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
