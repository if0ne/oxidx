#[derive(Debug, Clone, thiserror::Error)]
pub enum DxError {
    /// Occurs when trying to cast an interface to a higher version interface
    #[error("It's not possible cast {0} to {1}")]
    Cast(&'static str, &'static str),

    /// Dummy error
    #[error("Dummy")]
    Dummy,

    // DX12
    /// The specified cached PSO was created on a different adapter and cannot be reused on the current adapter
    #[error("The specified cached PSO was created on a different adapter and cannot be reused on the current adapter.")]
    AdapterNotFound,

    /// The specified cached PSO was created on a different driver version and cannot be reused on the current adapter
    #[error("The specified cached PSO was created on a different driver version and cannot be reused on the current adapter.")]
    DriverVersionMismatch,

    /// The method call is invalid
    #[error("The method call is invalid.")]
    InvalidCall,

    /// The previous blit operation that is transferring information to or from this surface is incomplete
    #[error("The previous blit operation that is transferring information to or from this surface is incomplete.")]
    WasStillDrawing,

    /// Generic error. Enable debug layer to get detailed error information
    #[error("Enable debug layer to get detailed error information.")]
    Fail,

    /// An invalid parameter was passed to the returning function
    #[error("An invalid parameter was passed to the returning function.")]
    InvalidArgs,

    /// Direct3D could not allocate sufficient memory to complete the call
    #[error("Direct3D could not allocate sufficient memory to complete the call.")]
    Oom,

    /// The method call isn't implemented with the passed parameter combination
    #[error("The method call isn't implemented with the passed parameter combination.")]
    NotImpl,

    /// Alternate success value, indicating a successful but nonstandard completion (the precise meaning depends on context)
    #[error("Alternate success value, indicating a successful but nonstandard completion (the precise meaning depends on context).")]
    False,

    // DXGI
    /// Generic DXGI error
    #[error("{0}")]
    Dxgi(String),
}
