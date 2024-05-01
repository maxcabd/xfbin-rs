use super::*;

#[derive(Debug, Clone)]
pub struct NuccCamera {
    pub struct_info: NuccStructInfo,
    pub version: u16,

    pub fov: f32
}

impl_nucc_info!(NuccCamera, struct_info);

impl From<NuccStructConverter> for NuccCamera {
    fn from(converter: NuccStructConverter) -> Self {
        let NuccStructConverter {
            nucc_chunk,
            struct_infos: _,
            struct_references: _,
        } = converter;

        let chunk = nucc_chunk
            .downcast::<NuccChunkCamera>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Self {
            struct_info: Default::default(),
            version: chunk.version,
            fov: chunk.fov,
        }
    }
}

impl From<NuccChunkConverter> for Box<NuccChunkCamera> {
    fn from(converter: NuccChunkConverter) -> Self {
        let NuccChunkConverter {
            nucc_struct,
            struct_info_map: _,
            struct_reference_map: _,
        } = converter;

        let cam = nucc_struct
            .downcast::<NuccCamera>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Box::new(NuccChunkCamera {
            version: cam.version,
            fov: cam.fov,
        })
    }
}

impl NuccStruct for NuccCamera {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkCamera
    }

    fn version(&self) -> u16 {
        self.version
    }
}
