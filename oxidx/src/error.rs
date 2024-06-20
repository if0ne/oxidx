#[derive(Debug, Clone, thiserror::Error)]
pub enum DxError {
    #[error("It's not possible cast {0} to {1}")]
    Cast(&'static str, &'static str),

    #[error("Dummy")]
    Dummy,
    #[error("")]
    SwapchainCreation,
    #[error("")]
    SwapchainPresent,

    // DX12
    #[error("The specified cached PSO was created on a different adapter and cannot be reused on the current adapter.")]
    AdapterNotFound,
    #[error("The specified cached PSO was created on a different driver version and cannot be reused on the current adapter.")]
    DriverVersionMismatch,
    #[error("The method call is invalid.")]
    InvalidCall,
    #[error("The previous blit operation that is transferring information to or from this surface is incomplete.")]
    WasStillDrawing,
    #[error("Enable debug layer to get detailed error information.")]
    Fail,
    #[error("An invalid parameter was passed to the returning function.")]
    InvalidArgs,
    #[error("Direct3D could not allocate sufficient memory to complete the call.")]
    Oom,
    #[error("The method call isn't implemented with the passed parameter combination.")]
    NotImpl,
    #[error("Alternate success value, indicating a successful but nonstandard completion (the precise meaning depends on context).")]
    False,

    // DXGI
    #[error("{0}")]
    Dxgi(String),
}
