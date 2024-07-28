use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{create_type, impl_trait, HasInterface};

pub trait IPipelineState:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12PipelineState>>
{
}

create_type! { PipelineState wrap ID3D12PipelineState }

impl_trait! {
    impl IPipelineState =>
    PipelineState;
}
