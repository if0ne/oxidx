use std::{ffi::CStr, path::Path};

use smallvec::SmallVec;
use windows::{
    core::{Interface, HSTRING, PCSTR},
    Win32::Graphics::{
        Direct3D::{Fxc::D3DCompileFromFile, ID3DBlob, D3D_SHADER_MACRO},
        Direct3D12::{D3D12_CACHED_PIPELINE_STATE, D3D12_SHADER_BYTECODE},
    },
};

use crate::{create_type, error::DxError, impl_trait, HasInterface};

/// This interface is used to return data of arbitrary length.
///
/// For more information: [`ID3DBlob interface`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ff728743(v=vs.85))
pub trait IBlob: HasInterface<Raw: Interface> {
    /// Gets a pointer to the data.
    ///
    /// For more information: [`ID3D10Blob::GetBufferPointer method`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/nf-d3dcommon-id3d10blob-getbufferpointer)
    fn get_buffer_ptr(&self) -> std::ptr::NonNull<()>;

    /// Gets the size.
    ///
    /// For more information: [`ID3D10Blob::GetBufferSize method`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/nf-d3dcommon-id3d10blob-getbuffersize)
    fn get_buffer_size(&self) -> usize;
}

/// Additional methods
pub trait IBlobExt: IBlob {
    /// Compiles Microsoft High Level Shader Language (HLSL) code into bytecode for a given target.
    ///
    /// For more information: [`D3DCompileFromFile function`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompilefromfile)
    fn compile_from_file<'a>(
        filename: impl AsRef<Path>,
        defines: impl IntoIterator<Item = (&'a CStr, &'a CStr)>,
        entry_point: impl AsRef<CStr>,
        target: impl AsRef<CStr>,
        flags1: u32,
        flags2: u32,
    ) -> Result<Self, DxError>
    where
        Self: Sized;
}

create_type! {
    /// This interface is used to return data of arbitrary length.
    ///
    /// For more information: [`ID3DBlob interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3d10blob)
    Blob wrap ID3DBlob
}

impl Blob {
    pub(crate) fn as_shader_bytecode(&self) -> D3D12_SHADER_BYTECODE {
        D3D12_SHADER_BYTECODE {
            pShaderBytecode: self.get_buffer_ptr().as_ptr() as *const _,
            BytecodeLength: self.get_buffer_size(),
        }
    }

    pub(crate) fn as_cached_pipeline_state(&self) -> D3D12_CACHED_PIPELINE_STATE {
        D3D12_CACHED_PIPELINE_STATE {
            pCachedBlob: self.get_buffer_ptr().as_ptr() as *const _,
            CachedBlobSizeInBytes: self.get_buffer_size(),
        }
    }
}

impl_trait! {
    impl IBlob =>
    Blob;

    fn get_buffer_ptr(&self) -> std::ptr::NonNull<()> {
        unsafe {
            std::ptr::NonNull::new(self.0.GetBufferPointer() as *mut _).expect("Expected valid pointer")
        }
    }

    fn get_buffer_size(&self) -> usize {
        unsafe {
            self.0.GetBufferSize()
        }
    }
}

impl_trait! {
    impl IBlobExt =>
    Blob;

    fn compile_from_file<'a>(
        filename: impl AsRef<Path>,
        defines: impl IntoIterator<Item = (&'a CStr, &'a CStr)>,
        entry_point: impl AsRef<CStr>,
        target: impl AsRef<CStr>,
        flags1: u32,
        flags2: u32,
    ) -> Result<Self, DxError>
    where
        Self: Sized,
    {
        let filename: HSTRING = filename.as_ref().to_str().unwrap_or("").into();
        let entry_point = PCSTR::from_raw(entry_point.as_ref().as_ptr() as *const _);
        let target = PCSTR::from_raw(target.as_ref().as_ptr() as *const _);

        let mut shader = None;

        let defines = defines.into_iter()
            .map(|(k, v)| {
                let k = PCSTR::from_raw(k.as_ref().as_ptr() as *const _);
                let v = PCSTR::from_raw(v.as_ref().as_ptr() as *const _);

                D3D_SHADER_MACRO {
                    Name: k,
                    Definition: v
                }
            })
            .chain(std::iter::once(D3D_SHADER_MACRO {
                Name: PCSTR::null(),
                Definition: PCSTR::null(),
            }))
            .collect::<SmallVec<[_; 8]>>();

        unsafe {
            D3DCompileFromFile(
                &filename,
                Some(defines.as_ptr()),
                None,
                entry_point,
                target,
                flags1,
                flags2,
                &mut shader,
                None,
            )
            .map_err(DxError::from)?;
        }

        Ok(Blob::new(shader.unwrap()))
    }
}
