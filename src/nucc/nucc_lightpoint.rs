use super::*;

#[derive(Debug, Clone)]
pub struct NuccLightPoint {
    pub struct_info: NuccStructInfo,
    pub version: u16,

    pub data: Vec<u8>,
}

impl_nucc_info!(NuccLightPoint, struct_info);

impl From<NuccStructConverter> for NuccLightPoint {
    fn from(converter: NuccStructConverter) -> Self {
        let NuccStructConverter {
            nucc_chunk,
            struct_infos: _,
            struct_references: _,
        } = converter;

        let chunk = nucc_chunk
            .downcast::<NuccChunkLightPoint>()
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

impl From<NuccChunkConverter> for Box<NuccChunkLightPoint> {
    fn from(converter: NuccChunkConverter) -> Self {
        let NuccChunkConverter {
            nucc_struct,
            struct_info_map: _,
            struct_reference_map: _,
        } = converter;

        let lightpoint = nucc_struct
            .downcast::<NuccLightPoint>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Box::new(NuccChunkLightPoint {
            version: lightpoint.version,
            data: lightpoint.data,
        })
    }
}

impl NuccStruct for NuccLightPoint {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkLightPoint
    }

    fn version(&self) -> u16 {
        self.version
    }
}
