use binrw::binrw;

use super::{NuccChunk, NuccChunkType};

#[binrw]
#[br(import_raw(version: u16))]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NuccChunkLayerSet {
    #[brw(ignore)]
    pub version: u16,

    #[br(count = 466)]
    pub data: Vec<u8>,
}

impl NuccChunk for NuccChunkLayerSet {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkLayerSet
    }

    fn version(&self) -> u16 {
        self.version
    }
}
