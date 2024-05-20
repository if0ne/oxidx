#[derive(Debug, Clone)]
pub enum DxError {
    CastError,
    NotFoundAdaptersError,
    FactoryCreationError,
    SwapchainCreationError,
    SwapchainPresentError,
    Dummy,
}
