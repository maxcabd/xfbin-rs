use super::*;

#[derive(Debug, Clone)]
pub struct NuccMorphModel {
    pub struct_info: NuccStructInfo,
    pub version: u16,

    pub data: Vec<u8>,
}

impl_nucc_info!(NuccMorphModel, struct_info);

impl From<NuccStructConverter> for NuccMorphModel {
    fn from(converter: NuccStructConverter) -> Self {
        let NuccStructConverter {
            nucc_chunk,
            struct_infos: _,
            struct_references: _,
        } = converter;

        let chunk = nucc_chunk
            .downcast::<NuccChunkMorphModel>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Self {
            struct_info: Default::default(),
            version: chunk.version,
            data: chunk.data,
        }
    }
}

impl From<NuccChunkConverter> for Box<NuccChunkMorphModel> {
    fn from(converter: NuccChunkConverter) -> Self {
        let NuccChunkConverter {
            nucc_struct,
            struct_info_map: _,
            struct_reference_map: _,
        } = converter;

        let morphmodel = nucc_struct
            .downcast::<NuccMorphModel>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Box::new(NuccChunkMorphModel {
            version: morphmodel.version,
            data: morphmodel.data,
        })
    }
}

impl NuccStruct for NuccMorphModel {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkMorphModel
    }

    fn version(&self) -> u16 {
        self.version
    }
}
