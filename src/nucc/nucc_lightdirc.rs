use super::*;

#[derive(Debug, Clone)]
pub struct NuccLightDirc {
    pub struct_info: NuccStructInfo,
    pub version: u16,

    pub data: Vec<u8>,
}

impl_nucc_info!(NuccLightDirc, struct_info);

impl From<NuccStructConverter> for NuccLightDirc {
    fn from(converter: NuccStructConverter) -> Self {
        let NuccStructConverter {
            nucc_chunk,
            struct_infos: _,
            struct_references: _,
        } = converter;

        let chunk = nucc_chunk
            .downcast::<NuccChunkLightDirc>()
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

impl From<NuccChunkConverter> for Box<NuccChunkLightDirc> {
    fn from(converter: NuccChunkConverter) -> Self {
        let NuccChunkConverter {
            nucc_struct,
            struct_info_map: _,
            struct_reference_map: _,
        } = converter;

        let lightdirc = nucc_struct
            .downcast::<NuccLightDirc>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Box::new(NuccChunkLightDirc {
            version: lightdirc.version,
            data: lightdirc.data,
        })
    }
}

impl NuccStruct for NuccLightDirc {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkLightDirc
    }

    fn version(&self) -> u16 {
        self.version
    }
}
