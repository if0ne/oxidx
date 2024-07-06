use smallvec::SmallVec;
use windows::{
    core::{Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{blob::Blob, create_type, error::DxError, impl_trait, types::*, HasInterface};

/// The root signature defines what resources are bound to the graphics pipeline.
/// A root signature is configured by the app and links command lists to the resources the shaders require.
/// Currently, there is one graphics and one compute root signature per app.
///
/// For more information: [`ID3D12RootSignature interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12rootsignature)
pub trait RootSignatureInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<ID3D12RootSignature>>
{
    fn serialize(
        desc: &RootSignatureDesc<'_>,
        version: RootSignatureVersion,
    ) -> Result<Blob, DxError>;
}

create_type! {
    /// The root signature defines what resources are bound to the graphics pipeline.
    /// A root signature is configured by the app and links command lists to the resources the shaders require.
    /// Currently, there is one graphics and one compute root signature per app.
    ///
    /// For more information: [`ID3D12RootSignature interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12rootsignature)
    RootSignature wrap ID3D12RootSignature
}

impl_trait! {
    impl RootSignatureInterface =>
    RootSignature;

    fn serialize(desc: &RootSignatureDesc<'_>, version: RootSignatureVersion) -> Result<Blob, DxError> {
        let mut signature = None;

        let parameters = desc.parameters.iter().map(|param| param.as_raw()).collect::<SmallVec<[_; 16]>>();
        let sampler = desc.samplers.iter().map(|sampler| sampler.as_raw()).collect::<SmallVec<[_; 16]>>();

        let desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: desc.parameters.len() as u32,
            pParameters: parameters.as_ptr(),
            NumStaticSamplers: desc.samplers.len() as u32,
            pStaticSamplers: sampler.as_ptr(),
            Flags: desc.flags.as_raw(),
        };

        let signature = unsafe {
            D3D12SerializeRootSignature(
                &desc,
                version.as_raw(),
                &mut signature,
                None,
            )
        }
        .map(|()| signature.unwrap())
        .map_err(|_| DxError::Dummy)?;

        Ok(Blob::new(signature))
    }
}
