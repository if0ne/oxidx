use windows::Win32::Graphics::Direct3D12::ID3D12Resource;

use crate::{create_type, impl_trait, HasInterface};

pub trait ResourceInterface: HasInterface {}

create_type! { Resource wrap ID3D12Resource }

impl_trait! {
    impl ResourceInterface =>
    Resource;
}
