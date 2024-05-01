use super::*;

#[derive(Debug, Clone)]
pub struct NuccAmbient {
    pub struct_info: NuccStructInfo,
    pub version: u16,

    pub data: Vec<u8>,
}

impl_nucc_info!(NuccAmbient, struct_info);

impl From<NuccStructConverter> for NuccAmbient {
    fn from(converter: NuccStructConverter) -> Self {
        let NuccStructConverter {
            nucc_chunk,
            struct_infos: _,
            struct_references: _,
        } = converter;

        let chunk = nucc_chunk
            .downcast::<NuccChunkAmbient>()
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

impl From<NuccChunkConverter> for Box<NuccChunkAmbient> {
    fn from(converter: NuccChunkConverter) -> Self {
        let NuccChunkConverter {
            nucc_struct,
            struct_info_map: _,
            struct_reference_map: _,
        } = converter;

        let ambient = nucc_struct
            .downcast::<NuccAmbient>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Box::new(NuccChunkAmbient {
            version: ambient.version,
            data: ambient.data,
        })
    }
}

impl NuccStruct for NuccAmbient {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkAmbient
    }

    fn version(&self) -> u16 {
        self.version
    }
}
