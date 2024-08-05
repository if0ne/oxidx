use std::{ffi::CStr, path::Path};

use windows::{
    core::{Interface, HSTRING, PCSTR},
    Win32::Graphics::{
        Direct3D::{Fxc::D3DCompileFromFile, ID3DBlob},
        Direct3D12::{D3D12_CACHED_PIPELINE_STATE, D3D12_SHADER_BYTECODE},
    },
};

use crate::{create_type, error::DxError, impl_trait, HasInterface};

/// This interface is used to return data of arbitrary length.
///
/// For more information: [`ID3DBlob interface`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ff728743(v=vs.85))
pub trait IBlob: HasInterface<Raw: Interface> {
    /// TODO: type for target
    fn compile_from_file(
        filename: impl AsRef<Path>,
        /*defines, includes,*/
        entry_point: impl AsRef<CStr>,
        target: impl AsRef<CStr>,
        flags1: u32,
        flags2: u32,
    ) -> Result<Self, DxError>
    where
        Self: Sized;

    /// Gets a pointer to the data.
    ///
    /// For more information: [`ID3D10Blob::GetBufferPointer method`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/nf-d3dcommon-id3d10blob-getbufferpointer)
    fn get_buffer_ptr(&self) -> std::ptr::NonNull<()>;

    /// Gets the size.
    ///
    /// For more information: [`ID3D10Blob::GetBufferSize method`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/nf-d3dcommon-id3d10blob-getbuffersize)
    fn get_buffer_size(&self) -> usize;
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

    fn compile_from_file(
        filename: impl AsRef<Path>,
        /*defines, includes,*/
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

        unsafe {
            D3DCompileFromFile(
                &filename,
                None,
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
