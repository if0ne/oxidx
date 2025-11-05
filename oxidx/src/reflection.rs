use std::ffi::CStr;

use windows::Win32::Graphics::Direct3D12::{
    ID3D12ShaderReflection, ID3D12ShaderReflectionConstantBuffer, ID3D12ShaderReflectionType,
    ID3D12ShaderReflectionVariable,
};

use crate::{create_type, error::DxError, impl_interface, types::*};

create_type! {
    /// A shader-reflection interface accesses shader information.
    ///
    /// For more information: [`ID3D12ShaderReflection interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflection)
    ShaderReflection wrap ID3D12ShaderReflection
}

impl_interface! {
    ShaderReflection;

    /// Gets the number of bitwise instructions.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetBitwiseInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getbitwiseinstructioncount)
    #[inline]
    pub fn get_bitwise_instruction_count(&self) -> u32 {
        unsafe {
            self.0.GetBitwiseInstructionCount()
        }
    }

    /// Gets a constant buffer by index.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetConstantBufferByIndex function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getconstantbufferbyindex)
    #[inline]
    pub fn get_constant_buffer_by_index(&self, index: usize) -> Option<ShaderReflectionConstantBuffer> {
        unsafe {
            self.0.GetConstantBufferByIndex(index as u32)
                .map(ShaderReflectionConstantBuffer)
        }
    }

    /// Gets a constant buffer by name.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetConstantBufferByName function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getconstantbufferbyname)
    #[inline]
    pub fn get_constant_buffer_by_name(&self, name: impl AsRef<CStr>) -> Option<ShaderReflectionConstantBuffer> {
        unsafe {
            let name = windows::core::PCSTR::from_raw(name.as_ref().as_ptr() as *const _);

            self.0.GetConstantBufferByName(name)
                .map(ShaderReflectionConstantBuffer)
        }
    }

    /// Gets the number of conversion instructions.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetConversionInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getconversioninstructioncount)
    #[inline]
    pub fn get_conversion_instruction_count(&self) -> u32 {
        unsafe {
            self.0.GetConversionInstructionCount()
        }
    }

    /// Gets a shader description.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getdesc)
    #[inline]
    pub fn get_desc(&self) -> Result<ShaderDesc, DxError> {
        unsafe {
            let mut raw = Default::default();
            self.0.GetDesc(&mut raw).map_err(DxError::from)?;

            Ok(ShaderDesc(raw))
        }
    }

    /// Gets the geometry-shader input-primitive description.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetGsInputPrimitive function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getgsinputprimitive)
    #[inline]
    pub fn get_gs_input_primitive(&self) -> Primitive {
        unsafe {
            self.0.GetGSInputPrimitive().into()
        }
    }

    /// Gets an input-parameter description for a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetInputParameterDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getinputparameterdesc)
    #[inline]
    pub fn get_input_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError> {
        unsafe {
            let mut raw = Default::default();
            self.0.GetInputParameterDesc(index as u32, &mut raw).map_err(DxError::from)?;

            Ok(SignatureParameterDesc(raw))
        }
    }

    /// Gets the minimum feature level.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetMinFeatureLevel function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getminfeaturelevel)
    #[inline]
    pub fn get_min_feature_level(&self) -> Result<FeatureLevel, DxError> {
        unsafe {
            self.0.GetMinFeatureLevel()
                .map(|v| v.into())
                .map_err(DxError::from)
        }
    }

    /// Gets the number of Movc instructions.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetMovcInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getmovcinstructioncount)
    #[inline]
    pub fn get_movc_instruction_count(&self) -> u32 {
        unsafe {
            self.0.GetMovcInstructionCount()
        }
    }

    /// Gets the number of Mov instructions
    ///
    /// For more information: [`ID3D12ShaderReflection::GetMovInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getmovinstructioncount)
    #[inline]
    pub fn get_mov_instruction_count(&self) -> u32 {
        unsafe {
            self.0.GetMovInstructionCount()
        }
    }

    /// Gets the number of interface slots in a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetNumInterfaceSlots function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getnuminterfaceslots)
    #[inline]
    pub fn get_num_interface_slots(&self) -> u32 {
        unsafe {
            self.0.GetNumInterfaceSlots()
        }
    }

    /// Gets an output-parameter description for a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetOutputParameterDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getoutputparameterdesc)
    #[inline]
    pub fn get_output_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError> {
        unsafe {
            let mut raw = Default::default();
            self.0.GetOutputParameterDesc(index as u32, &mut raw).map_err(DxError::from)?;

            Ok(SignatureParameterDesc(raw))
        }
    }

    /// Gets a patch-constant parameter description for a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetPatchConstantParameterDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getpatchconstantparameterdesc)
    #[inline]
    pub fn get_patch_constant_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError> {
        unsafe {
            let mut raw = Default::default();
            self.0.GetPatchConstantParameterDesc(index as u32, &mut raw).map_err(DxError::from)?;

            Ok(SignatureParameterDesc(raw))
        }
    }

    /// Retrieves a group of flags that indicate the requirements of a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetRequiresFlags function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getrequiresflags)
    #[inline]
    pub fn get_requires_flags(&self) -> ShaderRequirements {
        unsafe {
            self.0.GetRequiresFlags().into()
        }
    }

    /// Gets a description of how a resource is bound to a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetResourceBindingDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getresourcebindingdesc)
    #[inline]
    pub fn get_resource_binding_desc(&self, index: usize) -> Result<ShaderInputBindDesc, DxError> {
        unsafe {
            let mut raw = Default::default();
            self.0.GetResourceBindingDesc(index as u32, &mut raw).map_err(DxError::from)?;

            Ok(ShaderInputBindDesc(raw))
        }
    }

    /// Gets a description of how a resource is bound to a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetResourceBindingDescByName function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getresourcebindingdescbyname)
    #[inline]
    pub fn get_resource_binding_desc_by_name(&self, name: impl AsRef<CStr>) -> Result<ShaderInputBindDesc, DxError> {
        unsafe {
            let name = windows::core::PCSTR::from_raw(name.as_ref().as_ptr() as *const _);
            let mut raw = Default::default();

            self.0.GetResourceBindingDescByName(name, &mut raw).map_err(DxError::from)?;
            Ok(ShaderInputBindDesc(raw))
        }
    }

    /// Retrieves the sizes, in units of threads, of the X, Y, and Z dimensions of the shader's thread-group grid.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetThreadGroupSize function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getthreadgroupsize)
    #[inline]
    pub fn get_thread_group_size(&self) -> (u32, u32, u32, u32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            let mut z = 0;
            let total = self.0.GetThreadGroupSize(Some(&mut x), Some(&mut y), Some(&mut z));

            (x, y, z, total)
        }
    }

    /// Gets a variable by name.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetVariableByName function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getvariablebyname)
    #[inline]
    pub fn get_variable_by_name(&self, name: impl AsRef<CStr>) -> Option<ShaderReflectionVariable> {
        unsafe {
            let name = windows::core::PCSTR::from_raw(name.as_ref().as_ptr() as *const _);

            self.0.GetVariableByName(name)
                .map(ShaderReflectionVariable)
        }
    }

    /// Indicates whether a shader is a sample frequency shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::IsSampleFrequencyShader function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-issamplefrequencyshader)
    #[inline]
    pub fn is_sample_frequency_shader(&self) -> bool {
        unsafe {
            self.0.IsSampleFrequencyShader().into()
        }
    }
}

create_type! {
    /// This shader-reflection interface provides access to a constant buffer.
    ///
    /// For more information: [`ID3D12ShaderReflectionConstantBuffer interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflectionconstantbuffer)
    ShaderReflectionConstantBuffer wrap ID3D12ShaderReflectionConstantBuffer
}

impl_interface! {
    ShaderReflectionConstantBuffer;

    /// Gets a constant-buffer description.
    ///
    /// For more information: [`ID3D12ShaderReflectionConstantBuffer::GetDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectionconstantbuffer-getdesc)
    pub fn get_desc(&self) -> Result<ShaderBufferDesc, DxError> {
        unsafe {
            let mut raw = Default::default();
            self.0.GetDesc(&mut raw).map_err(DxError::from)?;

            Ok(ShaderBufferDesc(raw))
        }
    }

    /// Gets a shader-reflection variable by index.
    ///
    /// For more information: [`ID3D12ShaderReflectionConstantBuffer::GetVariableByIndex function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectionconstantbuffer-getvariablebyindex)
    pub fn get_variable_by_index(&self, index: usize) -> Option<ShaderReflectionVariable> {
        unsafe {
            self.0.GetVariableByIndex(index as u32)
                .map(ShaderReflectionVariable)
        }
    }

    /// Gets a shader-reflection variable by name.
    ///
    /// For more information: [`ID3D12ShaderReflectionConstantBuffer::GetVariableByName function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectionconstantbuffer-getvariablebyname)
    pub fn get_variable_by_name(&self, name: impl AsRef<CStr>) -> Option<ShaderReflectionVariable> {
        unsafe {
            let name = windows::core::PCSTR::from_raw(name.as_ref().as_ptr() as *const _);

            self.0.GetVariableByName(name)
                .map(ShaderReflectionVariable)
        }
    }
}

create_type! {
    /// This shader-reflection interface provides access to a variable.
    ///
    /// For more information: [`ID3D12ShaderReflectionVariable interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflectionvariable)
    ShaderReflectionVariable wrap ID3D12ShaderReflectionVariable
}

impl_interface! {
    ShaderReflectionVariable;

    /// Returns the [`ShaderReflectionConstantBuffer`] of the present [`ShaderReflectionVariable`].
    ///
    /// For more information: [`ID3D12ShaderReflectionVariable::GetBuffer function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectionvariable-getbuffer)
    pub fn get_buffer(&self) -> Option<ShaderReflectionConstantBuffer> {
        unsafe {
            self.0.GetBuffer()
                .map(ShaderReflectionConstantBuffer)
        }
    }

    /// Gets a shader-variable description.
    ///
    /// For more information: [`ID3D12ShaderReflectionVariable::GetDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectionvariable-getdesc)
    pub fn get_desc(&self) -> Result<ShaderVariableDesc, DxError> {
        unsafe {
            let mut raw = Default::default();
            self.0.GetDesc(&mut raw).map_err(DxError::from)?;

            Ok(ShaderVariableDesc(raw))
        }
    }

    /// Gets the corresponding interface slot for a variable that represents an interface pointer.
    ///
    /// For more information: [`ID3D12ShaderReflectionVariable::GetInterfaceSlot function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectionvariable-getinterfaceslot)
    pub fn get_interface_slot(&self, index: u32) -> u32 {
        unsafe {
            self.0.GetInterfaceSlot(index)
        }
    }

    /// Gets a shader-variable type.
    ///
    /// For more information: [`ID3D12ShaderReflectionVariable::GetType function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectionvariable-gettype)
    pub fn get_type(&self) -> Option<ShaderReflectionType> {
        unsafe {
            self.0.GetType()
                .map(ShaderReflectionType)
        }
    }
}

create_type! {
    /// This shader-reflection interface provides access to variable type.
    ///
    /// For more information: [`ID3D12ShaderReflectionType interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflectiontype)
    ShaderReflectionType wrap ID3D12ShaderReflectionType
}

impl_interface! {
    ShaderReflectionType;

    /// Gets an [`ShaderReflectionType`] Interface interface containing the variable base class type.
    ///
    /// For more information: [`ID3D12ShaderReflectionType::GetBaseClass function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectiontype-getbaseclass)
    pub fn get_base_class(&self) -> Option<ShaderReflectionType> {
        unsafe {
            self.0.GetBaseClass()
                .map(ShaderReflectionType)
        }
    }

    /// Gets the description of a shader-reflection-variable type.
    ///
    /// For more information: [`ID3D12ShaderReflectionType::GetDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectiontype-getdesc)
    pub fn get_desc(&self) -> Result<ShaderTypeDesc, DxError> {
        unsafe {
            let mut desc = Default::default();

            self.0.GetDesc(&mut desc)
                .map_err(DxError::from)?;

            Ok(ShaderTypeDesc(desc))
        }
    }

    /// Gets an interface by index.
    ///
    /// For more information: [`ID3D12ShaderReflectionType::GetInterfaceByIndex function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectiontype-getinterfacebyindex)
    pub fn get_interface_by_index(&self, index: u32) -> Option<ShaderReflectionType> {
        unsafe {
            self.0.GetInterfaceByIndex(index)
                .map(ShaderReflectionType)
        }
    }

    /// Gets a shader-reflection-variable type by index.
    ///
    /// For more information: [`ID3D12ShaderReflectionType::GetMemberTypeByIndex function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectiontype-getmembertypebyindex)
    pub fn get_member_type_by_index(&self, index: u32) -> Option<ShaderReflectionType> {
        unsafe {
            self.0.GetMemberTypeByIndex(index)
                .map(ShaderReflectionType)
        }
    }

    /// Gets a shader-reflection-variable type by name.
    ///
    /// For more information: [`ID3D12ShaderReflectionType::GetMemberTypeByName function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectiontype-getmembertypebyname)
    pub fn get_member_type_by_name(&self, name: impl AsRef<CStr>) -> Option<ShaderReflectionType> {
        unsafe {
            let name = windows::core::PCSTR::from_raw(name.as_ref().as_ptr() as *const _);

            self.0.GetMemberTypeByName(name)
                .map(ShaderReflectionType)
        }
    }

    /// Gets a shader-reflection-variable type.
    ///
    /// For more information: [`ID3D12ShaderReflectionType::GetMemberTypeName function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectiontype-getmembertypename)
    pub fn get_member_type_name(&self, index: u32) -> Option<&CStr> {
        unsafe {
            let name = self.0.GetMemberTypeName(index);

            if name.is_null() {
                return None;
            }

            Some(CStr::from_ptr(name.as_ptr() as *const _))
        }
    }

    /// Gets the number of interfaces.
    ///
    /// For more information: [`ID3D12ShaderReflectionType::GetNumInterfaces function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectiontype-getnuminterfaces)
    pub fn get_num_interfaces(&self) -> u32 {
        unsafe {
            self.0.GetNumInterfaces()
        }
    }

    /// Gets the base class of a class.
    ///
    /// For more information: [`ID3D12ShaderReflectionType::GetSubType function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectiontype-getsubtype)
    pub fn get_sub_type(&self) -> Option<ShaderReflectionType> {
        unsafe {
            self.0.GetSubType()
                .map(ShaderReflectionType)
        }
    }

    /// Indicates whether a class type implements an interface.
    ///
    /// For more information: [`ID3D12ShaderReflectionType::ImplementsInterface function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectiontype-implementsinterface)
    pub fn implements_interface(&self, base: &ShaderReflectionType) -> bool {
        unsafe {
            self.0.ImplementsInterface(&base.0).is_ok()
        }
    }

    /// Indicates whether two [`ShaderReflectionType`] Interface pointers have the same underlying type.
    ///
    /// For more information: [`ID3D12ShaderReflectionType::IsEqual function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectiontype-isequal)
    pub fn is_equal(&self, ty: &ShaderReflectionType) -> bool {
        unsafe {
            self.0.IsEqual(&ty.0).is_ok()
        }
    }

    /// Indicates whether a variable is of the specified type.
    ///
    /// For more information: [`ID3D12ShaderReflectionType::IsOfType function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflectiontype-isoftype)
    pub fn is_of_type(&self, ty: &ShaderReflectionType) -> bool {
        unsafe {
            self.0.IsOfType(&ty.0).is_ok()
        }
    }
}
