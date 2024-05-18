#![feature(associated_type_bounds)]

pub mod adapter;
pub mod command_queue;
pub mod device;
pub mod error;
pub mod factory;
pub mod misc;
pub mod swapchain;
pub mod sync;

mod conv;
mod utils;

pub trait HasInterface {
    type Raw;
    type RawRef<'a> where Self: 'a;

    fn as_raw_ref(&self) -> Self::RawRef<'_>;
}