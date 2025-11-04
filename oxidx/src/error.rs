/// Error values of `HRESULT`.
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
    Fail(String),

    /// An invalid parameter was passed to the returning function
    #[error("An invalid parameter was passed to the returning function.")]
    InvalidArgs,

    /// Direct3D could not allocate sufficient memory to complete the call
    #[error("Direct3D could not allocate sufficient memory to complete the call.")]
    Oom,

    /// The method call isn't implemented with the passed parameter combination
    #[error("The method call isn't implemented with the passed parameter combination.")]
    NotImpl,

    // DXGI
    /// Generic DXGI error
    #[error("{0} {1}")]
    Dxgi(DxgiError, String),

    /// Shader compilation error
    #[error("{0}")]
    ShaderCompilationError(String),

    /// Unknown type of error
    #[error("{0}")]
    Other(String),
}

/// DXGI Errors
///
/// For more information: [DXGI Error](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dxgi-error)
#[derive(Debug, Clone, thiserror::Error)]
pub enum DxgiError {
    #[error(
        "You tried to use a resource to which you did not have the required access privileges."
    )]
    AccessDenied,
    #[error("The desktop duplication interface is invalid.")]
    AccessLost,
    #[error("The desired element already exists.")]
    AlreadyExists,
    #[error("DXGI can't provide content protection on the swap chain.")]
    CannotProtectContent,
    #[error(
        "The application's device failed due to badly formed commands sent by the application."
    )]
    DeviceHung,
    #[error(
        "The video card has been physically removed from the system, or a driver upgrade for the video card has occurred."
    )]
    DeviceRemoved,
    #[error("The device failed due to a badly formed command.")]
    DeviceReset,
    #[error("The driver encountered a problem and was put into the device removed state.")]
    DriverInternalError,
    #[error("An event (for example, a power cycle) interrupted the gathering of presentation statistics.")]
    FrameStatisticsDisjoint,
    #[error("The application attempted to acquire exclusive ownership of an output, but failed because some other application (or device within the application) already acquired ownership.")]
    GraphicsVidpnSourceInUse,
    #[error("The application provided invalid parameter data; this must be debugged and fixed before the application is released.")]
    InvalidCall,
    #[error(
        "The buffer supplied by the application is not big enough to hold the requested data."
    )]
    MoreData,
    #[error(
        "The supplied name of a resource in a call to create_shared_handle is already associated with some other resource."
    )]
    NameAlreadyExists,
    #[error(
        "A global counter resource is in use, and the Direct3D device can't currently use the counter resource."
    )]
    NonExclusive,
    #[error(
        "The resource or request is not currently available, but it might become available later."
    )]
    NotCurrentlyAvailable,
    #[error("Requested object not found")]
    NotFound,
    #[error("TBD")]
    RemoteClientDisconnected,
    #[error("TBD")]
    RemoteOom,
    #[error("The DXGI output (monitor) to which the swap chain content was restricted is now disconnected or changed.")]
    RestrictToOutputStale,
    #[error("The operation depends on an SDK component that is missing or mismatched.")]
    SdkComponentMissing,
    #[error("The Remote Desktop Services session is currently disconnected.")]
    SessionDisconnected,
    #[error("The requested functionality is not supported by the device or the driver.")]
    Unsupported,
    #[error("The time-out interval elapsed before the next desktop frame was available.")]
    WaitTimeout,
    #[error("The GPU was busy at the moment when a call was made to perform an operation, and did not execute or schedule the operation.")]
    WasStillDrawing,
}
