use binrw::binrw;

use super::{NuccChunk, NuccChunkType};

#[binrw]
#[br(import_raw(version: u16))]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NuccChunkMorphModel {
    #[brw(ignore)]
    pub version: u16,

    #[br(count = 28)]
    pub data: Vec<u8>,
}

impl NuccChunk for NuccChunkMorphModel {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkMorphModel
    }

    fn version(&self) -> u16 {
        self.version
    }
}
