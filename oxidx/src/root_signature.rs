use windows::Win32::Graphics::Direct3D12::*;

use crate::{blob::Blob, create_type, error::DxError, impl_interface, types::*};

create_type! {
    /// The root signature defines what resources are bound to the graphics pipeline.
    /// A root signature is configured by the app and links command lists to the resources the shaders require.
    /// Currently, there is one graphics and one compute root signature per app.
    ///
    /// For more information: [`ID3D12RootSignature interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12rootsignature)
    RootSignature wrap ID3D12RootSignature
}

impl_interface! {
    RootSignature;

    /// Serializes a root signature version 1.0
    ///
    /// For more information: [`D3D12SerializeRootSignature function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-d3d12serializerootsignature)
    pub fn serialize(desc: &RootSignatureDesc<'_>, version: RootSignatureVersion) -> Result<Blob, DxError> {
        let mut signature = None;

        let signature = unsafe {
            D3D12SerializeRootSignature(
                &desc.0,
                version.as_raw(),
                &mut signature,
                None,
            )
        }
        .map(|()| signature.unwrap())
        .map_err(DxError::from)?;

        let bytes = unsafe {
            std::slice::from_raw_parts(
                signature.GetBufferPointer() as *const _,
                signature.GetBufferSize()
            ).to_vec()
        };

        Ok(bytes.into())
    }

    /// Serializes a root signature of any version, so that it can be passed to [`Device::create_root_signature`](crate::device::Device::create_root_signature).
    ///
    /// For more information: [`D3D12SerializeVersionedRootSignature function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-d3d12serializeversionedrootsignature)
    pub fn serialize_versioned(desc: &VersionedRootSignatureDesc<'_>) -> Result<Blob, DxError> {
        let mut signature = None;

        let signature = unsafe {
            D3D12SerializeVersionedRootSignature(
                &desc.0,
                &mut signature,
                None,
            )
        }
        .map(|()| signature.unwrap())
        .map_err(DxError::from)?;

        let bytes = unsafe {
            std::slice::from_raw_parts(
                signature.GetBufferPointer() as *const _,
                signature.GetBufferSize()
            ).to_vec()
        };

        Ok(bytes.into())
    }
}
