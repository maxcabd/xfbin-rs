//! # nuccTexture
//! nuccTexture is a container for NUT image textures  used in some CyberConnect2 games (e.g. Naruto Shippuden: Ultimate Ninja Storm 4).
//! The extension ".nut" may stand for "NU Texture".
//! 
use binrw::binrw;

use super::{NuccChunk, NuccChunkType};

#[binrw]
#[br(big)]
#[derive(Debug, Clone, PartialEq)]
pub struct NuccTexture {
    pub field00: u16,
    pub width: u16,
    pub height: u16,
    pub field06: u16,

    pub size: u32,

    pub nut: Nut
}

#[binrw]
#[brw(magic = b"NTP3")]
#[derive(Debug, Clone, PartialEq)]
pub struct Nut {
    pub version: u16,

    #[brw(pad_after = 8)]
    pub count: u16,

    #[br(count = count)]
    pub textures: Vec<NutTexture>
}

#[binrw]
#[derive(Debug, Clone, PartialEq)]
pub struct NutTexture {
    #[brw(pad_after = 4)]
    pub total_size: u32,
    pub data_size: u32,

    #[brw(pad_after = 2)]
    pub header_size: u16,

    pub mipmap_count: u16,
    pub pixel_format: u16,

    pub width: u16,
    #[brw(pad_after = 4)]
    pub height: u16,

    pub caps2: u32,

    #[brw(pad_after = 12)]
    pub cubemap_format: u32,

    #[br(count = mipmap_count)]
    pub mipmaps: Vec<u32>,

    #[brw(pad_after = 12)]
    pub ext: [u8; 4],

    #[brw(pad_after = 4)]
    pub gidx: [u8; 4],

    #[brw(pad_after = 4)]
    pub hash: u32,

    #[br(count = data_size)]
    pub texture_data: Vec<u8>
}