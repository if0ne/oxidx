use windows::Win32::Graphics::Direct3D12::ID3D12Device;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device(pub(crate) ID3D12Device);

impl Device {}
