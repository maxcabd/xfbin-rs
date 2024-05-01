use binrw::{binrw, until_eof, NullString};
use std::mem::size_of;

use super::nucc_chunk::{NuccChunk, NuccChunkType};

#[binrw]
#[derive(Debug, Clone, Default)]
pub struct XfbinFile {
    pub header: XfbinHeader,
    pub index: XfbinIndex,

    #[br(parse_with = until_eof)]
    pub chunks: Vec<XfbinChunk>,
}

#[binrw]
#[brw(magic = b"NUCC")]
#[derive(Debug, Clone, Default)]
pub struct XfbinHeader {
    pub version: u32,
    #[brw(pad_after = 6)]
    pub encrypted: u16,
}

#[binrw]
#[derive(Debug, Clone, Default)]
pub struct XfbinIndex {
    #[bw(calc = self.get_chunk_table_size())]
    pub chunk_table_size: u32,

    pub min_page_size: u32,

    #[brw(pad_after = 2)]
    pub version: u16,

    #[bw(calc = self.chunk_types.len() as u32)]
    pub chunk_type_count: u32,

    #[bw(calc = self.chunk_types.iter().map(|x| (x.len() + 1) as u32).sum())]
    pub chunk_type_size: u32,

    #[bw(calc = self.filepaths.len() as u32)]
    pub filepath_count: u32,

    #[bw(calc = self.filepaths.iter().map(|x| (x.len() + 1) as u32).sum())]
    pub filepath_size: u32,

    #[bw(calc = self.chunk_names.len() as u32)]
    pub chunk_name_count: u32,

    #[bw(calc = self.chunk_names.iter().map(|x| (x.len() + 1) as u32).sum())]
    pub chunk_name_size: u32,

    #[bw(calc = self.chunk_maps.len() as u32)]
    pub chunk_map_count: u32,

    #[bw(calc = (self.chunk_maps.len() * size_of::<XfbinChunkMap>()) as u32)]
    pub chunk_map_size: u32,

    #[bw(calc = self.chunk_map_indices.len() as u32)]
    pub chunk_map_indices_count: u32,

    #[bw(calc = self.chunk_references.len() as u32)]
    pub references_count: u32,

    #[br(count = chunk_type_count)]
    pub chunk_types: Vec<NullString>,

    #[br(count = filepath_count)]
    pub filepaths: Vec<NullString>,

    #[br(count = chunk_name_count)]
    pub chunk_names: Vec<NullString>,

    #[brw(align_before = 4)]
    #[br(count = chunk_map_count)]
    pub chunk_maps: Vec<XfbinChunkMap>,

    #[br(count = references_count)]
    pub chunk_references: Vec<XfbinChunkReference>,

    #[br(count = chunk_map_indices_count)]
    pub chunk_map_indices: Vec<u32>,
}

impl XfbinIndex {
    /// Calculate the size of the chunk table for writing
    pub fn get_chunk_table_size(&self) -> u32 {
        let chunk_types_size: u32 = self.chunk_types.iter().map(|x| (x.len() + 1) as u32).sum();
        let filepaths_size: u32 = self.filepaths.iter().map(|x| (x.len() + 1) as u32).sum();
        let chunk_names_size: u32 = self.chunk_names.iter().map(|x| (x.len() + 1) as u32).sum();

        let string_sizes = chunk_types_size + filepaths_size + chunk_names_size;

        0x28 + string_sizes + (4 - (string_sizes % 4)) // Add the header size, size of the strings buffer, and the aligned size of the strings
        + (self.chunk_maps.len() as u32 * size_of::<XfbinChunkMap>() as u32)
        + (self.chunk_map_indices.len() as u32 * size_of::<u32>() as u32)
    }
}

#[binrw]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XfbinChunkReference {
    pub chunk_name_index: u32,
    pub chunk_map_index: u32,
}

#[binrw]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XfbinChunkMap {
    pub chunk_type_index: u32,
    pub filepath_index: u32,
    pub chunk_name_index: u32,
}

#[binrw]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct XfbinChunk {
    #[bw(calc = data.len() as u32)]
    pub size: u32,

    pub chunk_map_index: u32,

    #[brw(pad_after = 2)]
    pub version: u16,

    #[br(count = size)]
    pub data: Vec<u8>,
}

impl XfbinChunk {
    pub fn unpack(self, chunk_type: &str) -> Box<dyn NuccChunk> {
        NuccChunkType::read_data(self.data, chunk_type, self.version)
            .map(|(_, chunk)| chunk)
            .unwrap()
    }

    pub fn repack(boxed: Box<dyn NuccChunk>) -> Self {
        let mut result = Self::default();
        result.version = 121;
        result.data = NuccChunkType::write_data(boxed, result.version).unwrap();

        result
    }
}
