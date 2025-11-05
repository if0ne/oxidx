use std::{ffi::CStr, path::Path};

use bytes::Bytes;
use windows::{
    core::{Interface, HSTRING, PCSTR},
    Win32::Graphics::{
        Direct3D::{
            Fxc::{D3DCompileFromFile, D3DReflect},
            ID3DInclude,
        },
        Direct3D12::{ID3D12ShaderReflection, D3D12_CACHED_PIPELINE_STATE, D3D12_SHADER_BYTECODE},
    },
};

use crate::{error::DxError, reflection::ShaderReflection, types::*};

pub type Blob = Bytes;

/// This interface is used to return data of arbitrary length.
///
///  For more information: [`ID3DBlob interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3d10blob)
pub trait Blobby {
    /// Compiles Microsoft High Level Shader Language (HLSL) code into bytecode for a given target.
    ///
    /// For more information: [`D3DCompileFromFile function`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompilefromfile)
    fn compile_from_file(
        filename: impl AsRef<Path>,
        defines: &[ShaderMacro],
        entry_point: impl AsRef<CStr>,
        target: impl AsRef<CStr>,
        flags1: u32,
        flags2: u32,
    ) -> Result<Self, DxError>
    where
        Self: Sized;

    /// Gets a pointer to a reflection interface.
    ///
    /// For more information: [`D3DReflect function`](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dreflect)
    fn reflect(&self) -> Result<ShaderReflection, DxError>;
}

pub(crate) trait BlobbyInternal {
    fn as_shader_bytecode(&self) -> D3D12_SHADER_BYTECODE;
    fn as_cached_pipeline_state(&self) -> D3D12_CACHED_PIPELINE_STATE;
}

impl BlobbyInternal for Blob {
    fn as_shader_bytecode(&self) -> D3D12_SHADER_BYTECODE {
        D3D12_SHADER_BYTECODE {
            pShaderBytecode: self.as_ptr() as *const u8 as *const _,
            BytecodeLength: self.len(),
        }
    }

    fn as_cached_pipeline_state(&self) -> D3D12_CACHED_PIPELINE_STATE {
        D3D12_CACHED_PIPELINE_STATE {
            pCachedBlob: self.as_ptr() as *const _,
            CachedBlobSizeInBytes: self.len(),
        }
    }
}

impl Blobby for Blob {
    fn compile_from_file(
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
                        std::str::from_utf8(slice).unwrap_or_default().to_string(),
                    ));
                } else {
                    return Err(DxError::ShaderCompilationError(err.to_string()));
                }
            }
        }

        let shader = shader.unwrap();
        let bytes = unsafe {
            std::slice::from_raw_parts(
                shader.GetBufferPointer() as *const u8,
                shader.GetBufferSize(),
            )
            .to_vec()
        };

        Ok(bytes.into())
    }

    fn reflect(&self) -> Result<ShaderReflection, DxError> {
        unsafe {
            let mut interface = std::ptr::null_mut();
            D3DReflect(
                self.as_ptr() as *const _,
                self.len(),
                &ID3D12ShaderReflection::IID,
                &mut interface,
            )?;

            let shader_reflection = ID3D12ShaderReflection::from_raw(interface);

            Ok(ShaderReflection(shader_reflection))
        }
    }
}
