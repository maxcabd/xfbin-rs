use binrw::binrw;

use super::{NuccChunk, NuccChunkType};

#[binrw]
#[br(import_raw(version: u16))]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NuccChunkLightPoint {
    #[brw(ignore)]
    pub version: u16,

    #[br(count = 64)]
    pub data: Vec<u8>,
}

impl NuccChunk for NuccChunkLightPoint {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkLightPoint
    }

    fn version(&self) -> u16 {
        self.version
    }
}
