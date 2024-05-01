use super::super::nucc::NuccStructInfo;
use super::{NuccChunk, NuccChunkType};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NuccChunkNull(pub u16);

impl NuccChunk for NuccChunkNull {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkNull
    }

    fn version(&self) -> u16 {
        self.0
    }
}

impl NuccChunkNull {
    pub fn default_chunk_info() -> NuccStructInfo {
        NuccStructInfo {
            chunk_name: String::from(""),
            filepath: String::from(""),
            chunk_type: NuccChunkType::NuccChunkNull.to_string(),
        }
    }
}
