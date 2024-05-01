use binrw::binrw;

use super::super::nucc::NuccStructInfo;
use super::{NuccChunk, NuccChunkType};

#[binrw]
#[brw(big)]
#[br(import_raw(version: u16))]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NuccChunkPage {
    #[brw(ignore)]
    #[bw(args_raw = version)]
    pub version: u16,

    pub map_index_count: u32,
    pub reference_count: u32,
}

impl NuccChunk for NuccChunkPage {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkPage
    }

    fn version(&self) -> u16 {
        self.version
    }
}

impl NuccChunkPage {
    pub fn default_chunk_info() -> NuccStructInfo {
        NuccStructInfo {
            chunk_name: String::from("Page0"),
            filepath: String::from(""),
            chunk_type: NuccChunkType::NuccChunkPage.to_string(),
        }
    }
}
