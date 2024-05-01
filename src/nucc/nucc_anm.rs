use super::*;

use crate::nucc_chunk::nucc_chunk_anm::{AnmClump, AnmEntry, CoordParent};

#[derive(Debug, Default, Clone)]
pub struct NuccAnm {
    pub struct_info: NuccStructInfo,

    pub version: u16,

    pub frame_count: u32,
    pub is_looped: bool,

    pub clumps: Vec<AnmClump>,

    pub other_entries_indices: Vec<u32>,
    pub unk_entry_indices: Vec<u32>,

    pub coord_parents: Vec<CoordParent>,

    pub entries: Vec<AnmEntry>,
}

impl_nucc_info!(NuccAnm, struct_info);

impl From<NuccStructConverter> for NuccAnm {
    fn from(converter: NuccStructConverter) -> Self {
        let NuccStructConverter {
            nucc_chunk,
            struct_infos: _,
            struct_references: _,
        } = converter;

        let chunk = nucc_chunk
            .downcast::<NuccChunkAnm>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Self {
            struct_info: Default::default(),
            version: chunk.version,
            frame_count: chunk.frame_count,
            is_looped: chunk.is_looped == 1,
            clumps: chunk.clumps,
            other_entries_indices: chunk.other_entries_indices,
            unk_entry_indices: chunk.unk_entry_indices,
            coord_parents: chunk.coord_parents,
            entries: chunk.entries,
        }
    }
}

impl From<NuccChunkConverter> for Box<NuccChunkAnm> {
    fn from(converter: NuccChunkConverter) -> Self {
        let NuccChunkConverter {
            nucc_struct,
            struct_info_map: _,
            struct_reference_map: _,
        } = converter;

        let mut anm = nucc_struct.downcast::<NuccAnm>().map(|s| *s).ok().unwrap();


        // Apply the pad values method to the entries
        for entry in anm.entries.iter_mut() {
            for (curve, curve_header) in entry
            .curves
            .iter_mut()
            .zip(&mut entry.curve_headers)
            {
                
                if curve.get_curve_format() == nucc_chunk_anm::AnmCurveFormat::SHORT3 as u16
                    || curve.get_curve_format() == nucc_chunk_anm::AnmCurveFormat::SHORT1_ALT as u16
                    || curve.get_curve_format() == nucc_chunk_anm::AnmCurveFormat::SHORT1 as u16
                    || curve.get_curve_format() == nucc_chunk_anm::AnmCurveFormat::BYTE3 as u16
                {
                    curve.pad_values();
                }

                curve_header.frame_count = curve.get_frame_count() as u16;
            }
            
        }
        
        let mut chunk = NuccChunkAnm::default();
        chunk.version = anm.version;
        chunk.frame_count = anm.frame_count;
        chunk.is_looped = if anm.is_looped { 1 } else { 0 };
        chunk.clumps = anm.clumps;
        chunk.other_entries_indices = anm.other_entries_indices;
        chunk.unk_entry_indices = anm.unk_entry_indices;
        chunk.coord_parents = anm.coord_parents;
        chunk.entries = anm.entries;

        Box::new(chunk)
    }
}

impl NuccStruct for NuccAnm {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkAnm
    }

    fn version(&self) -> u16 {
        self.version
    }
}
