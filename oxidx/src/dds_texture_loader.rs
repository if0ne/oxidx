use std::path::Path;

use crate::dx::Format;

const DDS_MAGIC: u32 = 0x20534444;

const DDS_FOURCC: u32 = 0x00000004; // DDPF_FOURCC
const DDS_RGB: u32 = 0x00000040; // DDPF_RGB
const DDS_LUMINANCE: u32 = 0x00020000; // DDPF_LUMINANCE
const DDS_ALPHA: u32 = 0x00000002; // DDPF_ALPHA
const DDS_BUMPDUDV: u32 = 0x00080000; // DDPF_BUMPDUDV

const DDS_HEADER_FLAGS_VOLUME: u32 = 0x00800000; // DDSD_DEPTH

const DDS_HEIGHT: u32 = 0x00000002; // DDSD_HEIGHT

const DDS_CUBEMAP_POSITIVEX: u32 = 0x00000600; // DDSCAPS2_CUBEMAP | DDSCAPS2_CUBEMAP_POSITIVEX
const DDS_CUBEMAP_NEGATIVEX: u32 = 0x00000a00; // DDSCAPS2_CUBEMAP | DDSCAPS2_CUBEMAP_NEGATIVEX
const DDS_CUBEMAP_POSITIVEY: u32 = 0x00001200; // DDSCAPS2_CUBEMAP | DDSCAPS2_CUBEMAP_POSITIVEY
const DDS_CUBEMAP_NEGATIVEY: u32 = 0x00002200; // DDSCAPS2_CUBEMAP | DDSCAPS2_CUBEMAP_NEGATIVEY
const DDS_CUBEMAP_POSITIVEZ: u32 = 0x00004200; // DDSCAPS2_CUBEMAP | DDSCAPS2_CUBEMAP_POSITIVEZ
const DDS_CUBEMAP_NEGATIVEZ: u32 = 0x00008200; // DDSCAPS2_CUBEMAP | DDSCAPS2_CUBEMAP_NEGATIVEZ

const DDS_CUBEMAP: u32 = 0x00000200; // DDSCAPS2_CUBEMAP

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct DdsMiscFlags2: i32 {
        const AlphaModeMask = 0x7;
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct DdsPixelFormat {
    size: u32,
    flags: u32,
    four_cc: u32,
    rgb_bit_count: u32,
    r_bit_mask: u32,
    g_bit_mask: u32,
    b_bit_mask: u32,
    a_bit_mask: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct DdsHeader {
    size: u32,
    flags: u32,
    height: u32,
    width: u32,
    pitch_or_linear_size: u32,
    depth: u32,
    mip_map_count: u32,
    _reserved1: [u32; 11],
    ddspf: DdsPixelFormat,
    caps: u32,
    caps2: u32,
    caps3: u32,
    caps4: u32,
    _reserved2: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct DdsHeaderDxt10 {
    format: Format,
    resource_dimension: u32,
    misc_flags: u32,
    array_size: u32,
    misc_flags2: u32,
}
