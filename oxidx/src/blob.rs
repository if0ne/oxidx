use std::{ffi::CStr, path::Path};

use windows::{
    core::{Interface, HSTRING, PCSTR},
    Win32::Graphics::{
        Direct3D::{
            Fxc::{D3DCompileFromFile, D3DCreateBlob, D3DReflect},
            ID3DBlob, ID3DInclude,
        },
        Direct3D12::{ID3D12ShaderReflection, D3D12_CACHED_PIPELINE_STATE, D3D12_SHADER_BYTECODE},
    },
};

use crate::{create_type, error::DxError, impl_interface, reflection::ShaderReflection, types::*};

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

impl_interface! {
    Blob;

    /// Gets a pointer to the data.
    ///
    /// For more information: [`ID3D10Blob::GetBufferPointer method`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/nf-d3dcommon-id3d10blob-getbufferpointer)
    pub fn get_buffer_ptr<T>(&self) -> std::ptr::NonNull<T> {
        unsafe {
            std::ptr::NonNull::new(self.0.GetBufferPointer() as *mut T).expect("Expected valid pointer")
        }
    }

    /// Gets the size.
    ///
    /// For more information: [`ID3D10Blob::GetBufferSize method`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/nf-d3dcommon-id3d10blob-getbuffersize)
    pub fn get_buffer_size(&self) -> usize {
        unsafe {
            self.0.GetBufferSize()
        }
    }
}

impl_interface! {
    Blob;

    /// Compiles Microsoft High Level Shader Language (HLSL) code into bytecode for a given target.
    ///
    /// For more information: [`D3DCompileFromFile function`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompilefromfile)
    pub fn compile_from_file(
        filename: impl AsRef<Path>,
        defines: &[ShaderMacro],
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

        let defines = if !defines.is_empty() {
            Some(defines.as_ptr() as *const _)
        } else {
            None
        };

        let mut error_msg = None;

        unsafe {
            let res = D3DCompileFromFile(
                &filename,
                defines,
                Some(&std::mem::transmute::<isize, ID3DInclude>(1isize)),
                entry_point,
                target,
                flags1,
                flags2,
                &mut shader,
                Some(&mut error_msg),
            )
            .map_err(DxError::from);

            if let Err(err) = res {
                if let Some(error_msg) = error_msg {
                    let pointer = error_msg.GetBufferPointer() as *mut u8;
                    let size = error_msg.GetBufferSize();

                    let slice = std::slice::from_raw_parts(pointer, size);

                    return Err(DxError::ShaderCompilationError(
                        std::str::from_utf8(slice)
                            .unwrap_or_default()
                            .to_string())
                    );
                } else {
                    return Err(DxError::ShaderCompilationError(err.to_string()));
                }
            }
        }

        Ok(Blob(shader.unwrap()))
    }

    /// Create buffer
    ///
    /// For more information: [`D3DCreateBlob function`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcreateblob)
    pub fn create_blob(size: usize) -> Result<Self, DxError>
    where
        Self: Sized
    {
        unsafe {
            D3DCreateBlob(size)
                .map(Self)
                .map_err(DxError::from)
        }
    }

    /// Create blob from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DxError>
    where
        Self: Sized
    {
        let blob = Self::create_blob(bytes.len())?;

        let slice = unsafe {
            std::slice::from_raw_parts_mut(blob.get_buffer_ptr().as_ptr(), blob.get_buffer_size())
        };

        slice.clone_from_slice(bytes);

        Ok(blob)
    }

    /// Gets a pointer to a reflection interface.
    ///
    /// For more information: [`D3DReflect function`]https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dreflect
    pub fn reflect(&self) -> Result<ShaderReflection, DxError> {
        unsafe {
            let mut interface = std::ptr::null_mut();
            D3DReflect(
                self.get_buffer_ptr().as_ptr(),
                self.get_buffer_size(),
                &ID3D12ShaderReflection::IID,
                &mut interface
            )?;

            let shader_reflection = ID3D12ShaderReflection::from_raw(interface);

            Ok(ShaderReflection(shader_reflection))
        }
    }
}
