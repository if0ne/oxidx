use std::{mem::ManuallyDrop, sync::OnceLock};

use windows::{
    core::PCSTR,
    Win32::{
        Graphics::Direct3D12::{ID3D12CommandQueue, ID3D12GraphicsCommandList},
        System::LibraryLoader::{GetProcAddress, LoadLibraryA},
    },
};

pub(crate) static WIN_PIX_EVENT_RUNTIME: OnceLock<WinPixEventRuntime> = OnceLock::new();

type BeginEventOnCommandList = fn(ManuallyDrop<ID3D12GraphicsCommandList>, u64, PCSTR);
type EndEventOnCommandList = fn(ManuallyDrop<ID3D12GraphicsCommandList>);
type SetMarkerOnCommandList = fn(ManuallyDrop<ID3D12GraphicsCommandList>, u64, PCSTR);

type BeginEventOnCommandQueue = fn(ManuallyDrop<ID3D12CommandQueue>, u64, PCSTR);
type EndEventOnCommandQueue = fn(ManuallyDrop<ID3D12CommandQueue>);
type SetMarkerOnCommandQueue = fn(ManuallyDrop<ID3D12CommandQueue>, u64, PCSTR);

#[derive(Debug)]
pub(crate) struct WinPixEventRuntime {
    pub(crate) begin_event_cmd_list: BeginEventOnCommandList,
    pub(crate) end_event_cmd_list: EndEventOnCommandList,
    pub(crate) set_marker_cmd_list: SetMarkerOnCommandList,

    pub(crate) begin_event_cmd_queue: BeginEventOnCommandQueue,
    pub(crate) end_event_cmd_queue: EndEventOnCommandQueue,
    pub(crate) set_marker_cmd_queue: SetMarkerOnCommandQueue,
}

impl WinPixEventRuntime {
    pub(crate) fn new() -> WinPixEventRuntime {
        unsafe {
            let module = LoadLibraryA(PCSTR::from_raw(
                c"WinPixEventRuntime.dll".as_ptr() as *const _
            ))
            .expect("Could not found WinPixEventRuntime.dll");

            let p_begin_event_list = GetProcAddress(
                module,
                PCSTR::from_raw(c"PIXBeginEventOnCommandList".as_ptr() as *const _),
            )
            .expect("Could not found PIXBeginEventOnCommandList");
            let p_end_event_list = GetProcAddress(
                module,
                PCSTR::from_raw(c"PIXEndEventOnCommandList".as_ptr() as *const _),
            )
            .expect("Could not found PIXEndEventOnCommandList");
            let p_set_marker_list = GetProcAddress(
                module,
                PCSTR::from_raw(c"PIXSetMarkerOnCommandList".as_ptr() as *const _),
            )
            .expect("Could not found PIXSetMarkerOnCommandList");

            let p_begin_event_queue = GetProcAddress(
                module,
                PCSTR::from_raw(c"PIXBeginEventOnCommandQueue".as_ptr() as *const _),
            )
            .expect("Could not found PIXBeginEventOnCommandQueue");
            let p_end_event_queue = GetProcAddress(
                module,
                PCSTR::from_raw(c"PIXEndEventOnCommandQueue".as_ptr() as *const _),
            )
            .expect("Could not found PIXEndEventOnCommandQueue");
            let p_set_marker_queue = GetProcAddress(
                module,
                PCSTR::from_raw(c"PIXSetMarkerOnCommandQueue".as_ptr() as *const _),
            )
            .expect("Could not found PIXSetMarkerOnCommandQueue");

            WinPixEventRuntime {
                begin_event_cmd_list: std::mem::transmute::<*const usize, BeginEventOnCommandList>(
                    p_begin_event_list as *const usize,
                ),
                end_event_cmd_list: std::mem::transmute::<*const usize, EndEventOnCommandList>(
                    p_end_event_list as *const usize,
                ),
                set_marker_cmd_list: std::mem::transmute::<*const usize, SetMarkerOnCommandList>(
                    p_set_marker_list as *const usize,
                ),

                begin_event_cmd_queue: std::mem::transmute::<*const usize, BeginEventOnCommandQueue>(
                    p_begin_event_queue as *const usize,
                ),
                end_event_cmd_queue: std::mem::transmute::<*const usize, EndEventOnCommandQueue>(
                    p_end_event_queue as *const usize,
                ),
                set_marker_cmd_queue: std::mem::transmute::<*const usize, SetMarkerOnCommandQueue>(
                    p_set_marker_queue as *const usize,
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn load_test() {
        use super::WIN_PIX_EVENT_RUNTIME;
        use crate::pix::WinPixEventRuntime;

        let _object = WIN_PIX_EVENT_RUNTIME.get_or_init(WinPixEventRuntime::new);

        assert!(true);
    }
}
