use windows::{
    core::{IUnknown, Param},
    Win32::Graphics::Dxgi::IDXGIAdapter3,
};

use crate::{create_type, impl_trait, types::AdapterDesc1, HasInterface};

pub trait IAdapter3: for<'a> HasInterface<RawRef<'a>: Param<IUnknown>> {
    fn get_desc1(&self) -> AdapterDesc1;
}

create_type! { Adapter3 wrap IDXGIAdapter3 }

impl_trait! {
    impl IAdapter3 =>
    Adapter3;

    fn get_desc1(&self) -> AdapterDesc1 {
        let mut desc = Default::default();

        unsafe {
            self.0.GetDesc1(&mut desc).unwrap(/*TODO: Error*/);
        }

        desc.into()
    }
}
