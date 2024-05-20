use windows::Win32::Graphics::Dxgi::IDXGIAdapter3;

use crate::{create_type, impl_trait, HasInterface};

pub trait AdapterInterface3: HasInterface {}

create_type! { Adapter3 wrap IDXGIAdapter3 }

impl_trait! {
    impl AdapterInterface3 =>
    Adapter3;
}
