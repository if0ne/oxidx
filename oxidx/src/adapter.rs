use compact_str::CompactString;
use windows::{
    core::{IUnknown, Param},
    Win32::Graphics::Dxgi::IDXGIAdapter3,
};

use crate::{create_type, impl_trait, HasInterface};

pub trait AdapterInterface3: for<'a> HasInterface<RawRef<'a>: Param<IUnknown>> {
    fn get_desc1(&self) -> AdapterDesc;
}

create_type! { Adapter3 wrap IDXGIAdapter3 }

impl_trait! {
    impl AdapterInterface3 =>
    Adapter3;

    fn get_desc1(&self) -> AdapterDesc {
        let mut desc = Default::default();

        unsafe {
            self.0.GetDesc1(&mut desc).unwrap(/*TODO: Error*/);
        }

        desc.into()
    }
}

#[derive(Debug, Clone)]
pub struct AdapterDesc {
    pub description: CompactString,
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: Luid,
    pub flags: AdapterFlags,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct AdapterFlags: u32 {
        const Remote = 1;
        const Sofware = 2;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Luid {
    pub low_part: u32,
    pub high_part: i32,
}
