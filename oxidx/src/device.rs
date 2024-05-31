use windows::{core::Interface, Win32::Graphics::Direct3D12::ID3D12Device};

use crate::{
    command_allocator::CommandAllocatorInterface,
    command_list::CommandListInterface,
    command_queue::{CommandQueueDesc, CommandQueueInterface},
    create_type,
    error::DxError,
    heap::{CpuDescriptorHandle, DescriptorHeapDesc, DescriptorHeapInterface, DescriptorHeapType},
    impl_trait,
    misc::CommandListType,
    pso::{
        BlobInterface, GraphicsPipelineDesc, PipelineStateInterface, RootSignatureDesc,
        RootSignatureInterface, RootSignatureVersion,
    },
    resources::{RenderTargetViewDesc, ResourceInterface},
    sync::{FenceFlags, FenceInterface},
    HasInterface,
};

pub trait DeviceInterface: HasInterface<Raw: Interface> {
    fn create_command_allocator<CA: CommandAllocatorInterface>(
        &self,
        r#type: CommandListType,
    ) -> Result<CA, DxError>;

    fn create_command_list<
        CL: CommandListInterface,
        CA: CommandAllocatorInterface,
        PSO: PipelineStateInterface,
    >(
        &self,
        nodemask: u32,
        r#type: CommandListType,
        command_allocator: &CA,
        pso: &PSO,
    ) -> Result<CL, DxError>;

    fn create_command_queue<CQ: CommandQueueInterface>(
        &self,
        desc: CommandQueueDesc,
    ) -> Result<CQ, DxError>;

    fn create_fence<F: FenceInterface>(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> Result<F, DxError>;

    fn create_descriptor_heap<H: DescriptorHeapInterface>(
        &self,
        desc: DescriptorHeapDesc,
    ) -> Result<H, DxError>;

    fn get_descriptor_handle_increment_size(&self, r#type: DescriptorHeapType) -> u32;

    fn create_render_target_view(
        &self,
        resource: &impl ResourceInterface,
        view_desc: Option<&RenderTargetViewDesc>,
        handle: CpuDescriptorHandle,
    );

    fn create_root_signature<RS: RootSignatureInterface>(
        &self,
        nodemask: u32,
        blob: &[u8],
    ) -> Result<RS, DxError>;

    fn serialize_create_root_signature<RS: RootSignatureInterface>(
        &self,
        desc: &RootSignatureDesc<'_>,
        version: RootSignatureVersion,
        nodemask: u32,
    ) -> Result<RS, DxError>;

    fn create_graphics_pipeline<'a, const RTV_NUM: usize, G: PipelineStateInterface>(
        &self,
        desc: &GraphicsPipelineDesc<'a, RTV_NUM>,
    ) -> Result<G, DxError>;
}

create_type! { Device wrap ID3D12Device }

impl_trait! {
    impl DeviceInterface =>
    Device;

    fn create_command_allocator<CA: CommandAllocatorInterface>(&self, r#type: CommandListType) -> Result<CA, DxError> {
        let res: CA::Raw  = unsafe {
            self.0.CreateCommandAllocator(r#type.as_raw()).map_err(|_| DxError::Dummy)?
        };

        Ok(CA::new(res))
    }

    fn create_command_queue<CQ: CommandQueueInterface>(
        &self,
        desc: CommandQueueDesc,
    ) -> Result<CQ, DxError> {
        let res: CQ::Raw  = unsafe {
            self.0.CreateCommandQueue(&desc.as_raw()).map_err(|_| DxError::Dummy)?
        };

        Ok(CQ::new(res))
    }

    fn create_fence<F: FenceInterface>(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> Result<F, DxError> {
        let res: F::Raw  = unsafe {
            self.0.CreateFence(initial_value, flags.as_raw()).map_err(|_| DxError::Dummy)?
        };

        Ok(F::new(res))
    }

    fn create_descriptor_heap<H: DescriptorHeapInterface>(
        &self,
        desc: DescriptorHeapDesc,
    ) -> Result<H, DxError> {
        let res: H::Raw  = unsafe {
            self.0.CreateDescriptorHeap(&desc.as_raw()).map_err(|_| DxError::Dummy)?
        };

        Ok(H::new(res))
    }

    fn get_descriptor_handle_increment_size(&self, r#type: DescriptorHeapType) -> u32 {
        unsafe {
            self.0.GetDescriptorHandleIncrementSize(r#type.as_raw())
        }
    }

    fn create_render_target_view(&self, resource: &impl ResourceInterface, view_desc: Option<&RenderTargetViewDesc>, handle: CpuDescriptorHandle) {
        let desc = view_desc.map(|v| v.as_raw());
        let desc = desc.as_ref().map(|f| f as *const _);

        unsafe {
            self.0.CreateRenderTargetView(resource.as_raw_ref(), desc, handle.as_raw());
        }
    }

    fn create_command_list<
        CL: CommandListInterface,
        CA: CommandAllocatorInterface,
        PSO: PipelineStateInterface,
    >(
        &self,
        nodemask: u32,
        r#type: CommandListType,
        command_allocator: &CA,
        pso: &PSO,
    ) -> Result<CL, DxError> {
        let res: CL::Raw = unsafe {
            self.0.CreateCommandList(nodemask, r#type.as_raw(), command_allocator.as_raw_ref(), pso.as_raw_ref()).map_err(|_| DxError::Dummy)?
        };

        Ok(CL::new(res))
    }

    fn create_root_signature<RS: RootSignatureInterface>(&self, nodemask: u32, blob: &[u8]) -> Result<RS, DxError> {
        let res: RS::Raw = unsafe {
            self.0
                .CreateRootSignature(
                    nodemask,
                    blob,
                )
                .map_err(|_| DxError::Dummy)?
        };

        Ok(RS::new(res))
    }

    fn serialize_create_root_signature<RS: RootSignatureInterface>(
        &self,
        desc: &RootSignatureDesc<'_>,
        version: RootSignatureVersion,
        nodemask: u32,
    ) -> Result<RS, DxError> {
        let blob = RS::serialize(desc, version)?;
        unsafe {
        self.create_root_signature(nodemask, std::slice::from_raw_parts(
                blob.get_buffer_ptr() as _,
                blob.get_buffer_size(),
            ))
        }
    }

    fn create_graphics_pipeline<'a, const RTV_NUM: usize, G: PipelineStateInterface>(
        &self,
        desc: &GraphicsPipelineDesc<'a, RTV_NUM>,
    ) -> Result<G, DxError> {
        let res: G::Raw  = unsafe {
            self.0.CreateGraphicsPipelineState(&desc.as_raw()).map_err(|_| DxError::Dummy)?
        };

        Ok(G::new(res))
    }
}
