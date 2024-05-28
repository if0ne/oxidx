use windows::{core::Interface, Win32::Graphics::Direct3D12::ID3D12Debug};

use crate::{create_type, impl_trait, HasInterface};

pub trait DebugInterface: HasInterface<Raw: Interface> {
    fn enable_debug_layer(&self);
}

create_type! { Debug wrap ID3D12Debug }

impl_trait! {
    impl DebugInterface =>
    Debug;

    fn enable_debug_layer(&self) {
        unsafe {
            self.0.EnableDebugLayer();
        }
    }
}
