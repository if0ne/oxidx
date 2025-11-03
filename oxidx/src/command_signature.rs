use windows::Win32::Graphics::Direct3D12::ID3D12CommandSignature;

use crate::{create_type, impl_interface};

create_type! {
    /// A command signature object enables apps to specify indirect drawing, including the buffer format, command type and resource bindings to be used.
    ///
    /// For more information: [`ID3D12CommandSignature interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12commandsignature)
    CommandSignature wrap ID3D12CommandSignature
}

impl_interface! {
    CommandSignature;
}
