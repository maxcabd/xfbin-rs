use super::*;

use crate::nucc_chunk::nucc_chunk_anmstrmframe::AnmStrmEntry;

#[derive(Debug, Default)]
pub struct NuccAnmStrmFrame {
    pub struct_info: NuccStructInfo,
    pub version: u16,
    pub frame_number: u32,
    pub unknown: u16,
    pub entries: Vec<AnmStrmEntry>,
}

impl_nucc_info!(NuccAnmStrmFrame, struct_info);

impl From<NuccStructConverter> for NuccAnmStrmFrame {
    fn from(converter: NuccStructConverter) -> Self {
        let NuccStructConverter {
            nucc_chunk,
            struct_infos: _,
            struct_references: _,
        } = converter;

        let chunk = nucc_chunk
            .downcast::<NuccChunkAnmStrmFrame>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Self {
            struct_info: Default::default(),
            version: chunk.version,
            frame_number: chunk.frame_number,
            unknown: chunk.unknown,
            entries: chunk.entries,
        }
    }
}

impl From<NuccChunkConverter> for Box<NuccChunkAnmStrmFrame> {
    fn from(converter: NuccChunkConverter) -> Self {
        let NuccChunkConverter {
            nucc_struct,
            struct_info_map: _,
            struct_reference_map: _,
        } = converter;

        let anmstrmframe = nucc_struct
            .downcast::<NuccAnmStrmFrame>()
            .map(|a| *a)
            .ok()
            .unwrap();

        let chunk = NuccChunkAnmStrmFrame {
            version: anmstrmframe.version,
            frame_number: anmstrmframe.frame_number,
            unknown: anmstrmframe.unknown,
            entries: anmstrmframe.entries,
        };

        Box::new(chunk)
    }
}

impl NuccStruct for NuccAnmStrmFrame {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkAnmStrmFrame
    }

    fn version(&self) -> u16 {
        self.version
    }
}
