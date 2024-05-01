use super::*;

use crate::nucc_chunk::nucc_chunk_anm::CoordParent;
use crate::nucc_chunk::nucc_chunk_anmstrm::{AnmStrmClump, AnmStrmFrameInfo};

#[derive(Default)]
pub struct NuccAnmStrm {
    pub struct_info: NuccStructInfo,

    pub version: u16,

    pub frame_count: u32,
    pub is_looped: bool,

    pub clumps: Vec<AnmStrmClump>,

    pub other_entry_indices: Vec<u32>,

    pub coord_parents: Vec<CoordParent>,

    pub entries: Vec<AnmStrmFrameInfo>,
}

impl_nucc_info!(NuccAnmStrm, struct_info);

impl From<NuccStructConverter> for NuccAnmStrm {
    fn from(converter: NuccStructConverter) -> Self {
        let NuccStructConverter {
            nucc_chunk,
            struct_infos: _,
            struct_references: _,
        } = converter;

        let chunk = nucc_chunk
            .downcast::<NuccChunkAnmStrm>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Self {
            struct_info: Default::default(),
            version: chunk.version,
            frame_count: chunk.frame_count,
            is_looped: chunk.is_looped == 1,
            clumps: chunk.clumps,
            other_entry_indices: chunk.other_entry_indices,
            coord_parents: chunk.coord_parents,
            entries: chunk.entries,
        }
    }
}

impl From<NuccChunkConverter> for Box<NuccChunkAnmStrm> {
    fn from(converter: NuccChunkConverter) -> Self {
        let NuccChunkConverter {
            nucc_struct,
            struct_info_map: _,
            struct_reference_map: _,
        } = converter;

        let anmstrm = nucc_struct
            .downcast::<NuccAnmStrm>()
            .map(|a| *a)
            .ok()
            .unwrap();

        let chunk = NuccChunkAnmStrm {
            version: anmstrm.version,
            frame_count: anmstrm.frame_count,
            is_looped: if anmstrm.is_looped { 1 } else { 0 },
            clumps: anmstrm.clumps,
            unk_entry_count: 0,
            other_entry_indices: anmstrm.other_entry_indices,
            coord_parents: anmstrm.coord_parents,
            entries: anmstrm.entries,
        };

        Box::new(chunk)
    }
}

impl NuccStruct for NuccAnmStrm {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkAnmStrm
    }

    fn version(&self) -> u16 {
        self.version
    }
}
