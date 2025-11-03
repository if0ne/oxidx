use core::str;
use std::{
    collections::HashMap,
    ffi::c_void,
    sync::{LazyLock, Mutex},
};

use windows::{
    core::PCSTR,
    Win32::Graphics::Direct3D12::{
        ID3D12InfoQueue1, D3D12_MESSAGE_CATEGORY, D3D12_MESSAGE_ID, D3D12_MESSAGE_SEVERITY,
    },
};

use crate::{create_type, dx::DxError, impl_interface, types::*};

static CALLBACK_MAP: LazyLock<Mutex<CallbackMap>> = LazyLock::new(Default::default);

#[derive(Debug, Default)]
struct CallbackMap {
    map: HashMap<u32, *mut c_void>,
}

unsafe impl Send for CallbackMap {}
unsafe impl Sync for CallbackMap {}

impl Drop for CallbackMap {
    fn drop(&mut self) {
        unsafe {
            let map = std::mem::take(&mut self.map);

            let map = map
                .into_iter()
                .map(|(k, v)| (k, std::boxed::Box::from_raw(v.cast::<CallbackData>())))
                .collect::<HashMap<_, _>>();

            drop(map);
        }
    }
}

create_type! {
    /// [`InfoQueue1`] inherits [`InfoQueue`]` and supports message callback with RegisterMessageCallback and UnregisterMessageCallback method.
    ///
    /// For more information: [`ID3D12InfoQueue1 interface`](https://microsoft.github.io/DirectX-Specs/d3d/MessageCallback.html)
    InfoQueue1 wrap ID3D12InfoQueue1
}

impl_interface! {
    InfoQueue1;

    pub fn register_message_callback(
        &self,
        callback: CallbackData,
        flags: CallbackFlags
    ) -> Result<u32, DxError> {
        unsafe{
            let mut id = 0;

            let callback = std::boxed::Box::new(callback);
            let callback = std::boxed::Box::into_raw(callback).cast();

            self.0.RegisterMessageCallback(
                Some(dx_callback),
                flags.as_raw(),
                callback,
                &mut id
            ).map_err(DxError::from)?;

            CALLBACK_MAP
                .lock()
                .unwrap()
                .map
                .insert(id, callback);

            Ok(id)
        }
    }

    pub fn unregister_message_callback(&self, callback_cookie: u32) -> Result<(), DxError> {
        unsafe {
            CALLBACK_MAP
                .lock()
                .unwrap()
                .map
                .remove(&callback_cookie);

            self.0.UnregisterMessageCallback(callback_cookie).map_err(DxError::from)
        }
    }
}

unsafe extern "system" fn dx_callback(
    category: D3D12_MESSAGE_CATEGORY,
    severity: D3D12_MESSAGE_SEVERITY,
    id: D3D12_MESSAGE_ID,
    pdescription: PCSTR,
    pcontext: *mut core::ffi::c_void,
) {
    let message = str::from_utf8(pdescription.as_bytes()).unwrap();
    let callback = pcontext.cast::<CallbackData>();
    (*callback)(category.into(), severity.into(), id.into(), message);
}
