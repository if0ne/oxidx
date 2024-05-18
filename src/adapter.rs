use windows::Win32::Graphics::Dxgi::IDXGIAdapter3;

use crate::{create_type, HasInterface};

#[allow(dead_code)]
pub(crate) trait AdapterInterface: HasInterface {}

create_type! { AdapterInterface => Adapter3 wrap IDXGIAdapter3; decorator for }
