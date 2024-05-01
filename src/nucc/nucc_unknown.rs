use super::*;

pub struct NuccUnknown {
    pub struct_info: NuccStructInfo,
    pub version: u16,

    pub data: Vec<u8>,
    pub chunk_type: String,
}

impl_nucc_info!(NuccUnknown, struct_info);

impl From<NuccStructConverter> for NuccUnknown {
    fn from(converter: NuccStructConverter) -> Self {
        let NuccStructConverter {
            nucc_chunk,
            struct_infos: _,
            struct_references: _,
        } = converter;

        let chunk = nucc_chunk
            .downcast::<NuccChunkUnknown>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Self {
            struct_info: Default::default(),
            data: chunk.data,
            chunk_type: chunk.chunk_type,
            version: chunk.version,
        }
    }
}

impl From<NuccChunkConverter> for Box<NuccChunkUnknown> {
    fn from(converter: NuccChunkConverter) -> Self {
        let NuccChunkConverter {
            nucc_struct,
            struct_info_map: _,
            struct_reference_map: _,
        } = converter;

        let unknown = nucc_struct
            .downcast::<NuccUnknown>()
            .map(|s| *s)
            .ok()
            .unwrap();

        Box::new(NuccChunkUnknown {
            version: unknown.version,
            data: unknown.data,
            chunk_type: unknown.chunk_type,
        })
    }
}

impl NuccStruct for NuccUnknown {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkUnknown
    }

    fn version(&self) -> u16 {
        self.version
    }
}
