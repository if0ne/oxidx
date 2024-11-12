use std::ffi::CStr;

use windows::Win32::Graphics::Direct3D12::{
    ID3D12ShaderReflection, ID3D12ShaderReflectionConstantBuffer,
};

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
    fn get_constant_buffer_by_name(&self, name: impl AsRef<CStr>)
        -> ShaderReflectionConstantBuffer;

    /// Gets the number of conversion instructions.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetConversionInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getconversioninstructioncount)
    fn get_conversion_instruction_count(&self) -> u32;

    /// Gets a shader description.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getdesc)
    fn get_desc(&self) -> Result<ShaderDesc, DxError>;

    /// Gets the geometry-shader input-primitive description.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetGsInputPrimitive function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getgsinputprimitive)
    fn get_gs_input_primitive(&self) -> Primitive;

    /// Gets an input-parameter description for a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetInputParameterDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getinputparameterdesc)
    fn get_input_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError>;

    /// Gets the minimum feature level.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetMinFeatureLevel function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getminfeaturelevel)
    fn get_min_feature_level(&self, index: usize) -> Result<FeatureLevel, DxError>;

    /// Gets the number of Movc instructions.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetMovcInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getmovcinstructioncount)
    fn get_movc_instruction_count(&self) -> u32;

    /// Gets the number of Mov instructions
    ///
    /// For more information: [`ID3D12ShaderReflection::GetMovInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getmovinstructioncount)
    fn get_mov_instruction_count(&self) -> u32;

    /// Gets the number of interface slots in a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetNumInterfaceSlots function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getnuminterfaceslots)
    fn get_num_interface_slots(&self) -> u32;

    /// Gets an output-parameter description for a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetOutputParameterDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getoutputparameterdesc)
    fn get_output_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError>;

    /// Gets a patch-constant parameter description for a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetPatchConstantParameterDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getpatchconstantparameterdesc)
    fn get_patch_constant_parameter_desc(
        &self,
        index: usize,
    ) -> Result<SignatureParameterDesc, DxError>;

    /// Retrieves a group of flags that indicate the requirements of a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetRequiresFlags function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getrequiresflags)
    fn get_requires_flags(&self, index: usize) -> ShaderRequirements;

    /// Gets a description of how a resource is bound to a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetResourceBindingDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getresourcebindingdesc)
    fn get_resource_binding_desc(&self, index: usize) -> Result<ShaderInputBindDesc, DxError>;

    /// Gets a description of how a resource is bound to a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetResourceBindingDescByName function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getresourcebindingdescbyname)
    fn get_resource_binding_desc_by_name(
        &self,
        name: impl AsRef<CStr>,
    ) -> Result<ShaderInputBindDesc, DxError>;

    /// Retrieves the sizes, in units of threads, of the X, Y, and Z dimensions of the shader's thread-group grid.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetThreadGroupSize function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getthreadgroupsize)
    fn get_thread_group_size(&self) -> (u32, u32, u32, u32);

    /// Gets a variable by name.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetVariableByName function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getvariablebyname)
    fn get_variable_by_name(
        &self,
        name: impl AsRef<CStr>,
    ) -> Result<ShaderReflectionVariable, DxError>;

    /// Indicates whether a shader is a sample frequency shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::IsSampleFrequencyShader function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-issamplefrequencyshader)
    fn is_sample_frequency_shader(&self) -> bool;
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

    fn get_gs_input_primitive(&self) -> Primitive {
        todo!()
    }

    fn get_input_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError> {
        todo!()
    }

    fn get_min_feature_level(&self, index: usize) -> Result<FeatureLevel, DxError> {
        todo!()
    }

    fn get_movc_instruction_count(&self) -> u32 {
        todo!()
    }

    fn get_mov_instruction_count(&self) -> u32 {
        todo!()
    }

    fn get_num_interface_slots(&self) -> u32 {
        todo!()
    }

    fn get_output_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError> {
        todo!()
    }

    fn get_patch_constant_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError> {
        todo!()
    }

    fn get_requires_flags(&self, index: usize) -> ShaderRequirements {
        todo!()
    }

    fn get_resource_binding_desc(&self, index: usize) -> Result<ShaderInputBindDesc, DxError> {
        todo!()
    }

    fn get_resource_binding_desc_by_name(&self, name: impl AsRef<CStr>) -> Result<ShaderInputBindDesc, DxError> {
        todo!()
    }

    fn get_thread_group_size(&self) -> (u32, u32, u32, u32) {
        todo!()
    }

    fn get_variable_by_name(&self, name: impl AsRef<CStr>) -> Result<ShaderReflectionVariable, DxError> {
        todo!()
    }

    fn is_sample_frequency_shader(&self) -> bool {
        todo!()
    }
}

/// This shader-reflection interface provides access to a constant buffer.
///
/// For more information: [`ID3D12ShaderReflectionConstantBuffer interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflectionconstantbuffer)
pub trait IShaderReflectionConstantBuffer: HasInterface {}

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

/// This shader-reflection interface provides access to a variable.
///
/// For more information: [`ID3D12ShaderReflectionVariable interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflectionvariable)
pub trait IShaderReflectionVariable: HasInterface {}

create_type! {
    /// This shader-reflection interface provides access to a variable.
    ///
    /// For more information: [`ID3D12ShaderReflectionVariable interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflectionvariable)
    ShaderReflectionVariable wrap ID3D12ShaderReflectionVariable
}

impl_trait! {
    impl IShaderReflectionVariable =>
    ShaderReflectionVariable;
}
