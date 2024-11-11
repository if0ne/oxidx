use std::ffi::CStr;

use windows::Win32::Graphics::Direct3D12::{ID3D12ShaderReflection, ID3D12ShaderReflectionConstantBuffer};

use crate::{create_type, error::DxError, impl_trait, types::*, HasInterface};

/// A shader-reflection interface accesses shader information.
///
/// For more information: [`ID3D12ShaderReflection interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflection)
pub trait IShaderReflection: HasInterface {
    /// Gets the number of bitwise instructions.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetBitwiseInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getbitwiseinstructioncount)
    fn get_bitwise_instruction_count(&self) -> u32;

    /// Gets a constant buffer by index.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetConstantBufferByIndex function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getconstantbufferbyindex)
    fn get_constant_buffer_by_index(&self, index: usize) -> ShaderReflectionConstantBuffer;

    /// Gets a constant buffer by name.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetConstantBufferByName function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getconstantbufferbyname)
    fn get_constant_buffer_by_name(&self, name: impl AsRef<CStr>) -> ShaderReflectionConstantBuffer;

    /// Gets the number of conversion instructions.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetConversionInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getconversioninstructioncount)
    fn get_conversion_instruction_count(&self) -> u32;

    /// Gets a shader description.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getdesc)
    fn get_desc(&self) -> Result<ShaderDesc, DxError>;
}

create_type! {
    /// A shader-reflection interface accesses shader information.
    ///
    /// For more information: [`ID3D12ShaderReflection interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflection)
    ShaderReflection wrap ID3D12ShaderReflection
}

impl_trait! {
    impl IShaderReflection =>
    ShaderReflection;

    fn get_bitwise_instruction_count(&self) -> u32 {
        unsafe {
            self.0.GetBitwiseInstructionCount()
        }
    }

    fn get_constant_buffer_by_index(&self, index: usize) -> ShaderReflectionConstantBuffer {
        todo!()
    }

    fn get_constant_buffer_by_name(&self, name: impl AsRef<CStr>) -> ShaderReflectionConstantBuffer {
        todo!()
    }

    fn get_conversion_instruction_count(&self) -> u32 {
        unsafe {
            self.0.GetConversionInstructionCount()
        }
    }

    fn get_desc(&self) -> Result<ShaderDesc, DxError> {
        todo!()
    }
}

/// This shader-reflection interface provides access to a constant buffer.
///
/// For more information: [`ID3D12ShaderReflectionConstantBuffer interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflectionconstantbuffer)
pub trait IShaderReflectionConstantBuffer: HasInterface {
}

create_type! {
    /// This shader-reflection interface provides access to a constant buffer.
    ///
    /// For more information: [`ID3D12ShaderReflectionConstantBuffer interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflectionconstantbuffer)
    ShaderReflectionConstantBuffer wrap ID3D12ShaderReflectionConstantBuffer 
}

impl_trait! {
    impl IShaderReflectionConstantBuffer =>
    ShaderReflectionConstantBuffer;
}