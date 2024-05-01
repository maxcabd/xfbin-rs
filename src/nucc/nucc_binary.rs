use super::*;

pub struct NuccBinary {
    pub struct_info: NuccStructInfo,
    pub version: u16,

    pub data: Vec<u8>,
}

impl_nucc_info!(NuccBinary, struct_info);

impl From<NuccStructConverter> for NuccBinary {
    fn from(converter: NuccStructConverter) -> Self {
        let NuccStructConverter {
            nucc_chunk,
            struct_infos: _,
            struct_references: _,
        } = converter;

        let chunk = nucc_chunk
            .downcast::<NuccChunkBinary>()
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

impl From<NuccChunkConverter> for Box<NuccChunkBinary> {
    fn from(converter: NuccChunkConverter) -> Self {
        let NuccChunkConverter {
            nucc_struct,
            struct_info_map: _,
            struct_reference_map: _,
        } = converter;

        let binary = nucc_struct
            .downcast::<NuccBinary>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Box::new(NuccChunkBinary {
            version: binary.version,
            data: binary.data,
        })
    }
}

impl NuccStruct for NuccBinary {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkBinary
    }

    fn version(&self) -> u16 {
        self.version
    }
}
