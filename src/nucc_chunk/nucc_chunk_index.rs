use super::super::nucc::NuccStructInfo;
use super::{NuccChunk, NuccChunkType};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NuccChunkIndex;

impl NuccChunk for NuccChunkIndex {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkIndex
    }

    fn version(&self) -> u16 {
        0
    }
}

impl NuccChunkIndex {
    pub fn default_chunk_info() -> NuccStructInfo {
        NuccStructInfo {
            chunk_name: String::from("index"),
            filepath: String::from(""),
            chunk_type: NuccChunkType::NuccChunkIndex.to_string(),
        }
    }
}
