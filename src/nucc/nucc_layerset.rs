use super::*;

#[derive(Debug, Clone)]
pub struct NuccLayerSet {
    pub struct_info: NuccStructInfo,
    pub version: u16,

    pub data: Vec<u8>,
}

impl_nucc_info!(NuccLayerSet, struct_info);

impl From<NuccStructConverter> for NuccLayerSet {
    fn from(converter: NuccStructConverter) -> Self {
        let NuccStructConverter {
            nucc_chunk,
            struct_infos: _,
            struct_references: _,
        } = converter;

        let chunk = nucc_chunk
            .downcast::<NuccChunkLayerSet>()
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

impl From<NuccChunkConverter> for Box<NuccChunkLayerSet> {
    fn from(converter: NuccChunkConverter) -> Self {
        let NuccChunkConverter {
            nucc_struct,
            struct_info_map: _,
            struct_reference_map: _,
        } = converter;

        let layerset = nucc_struct
            .downcast::<NuccLayerSet>()
            .map(|c| *c)
            .ok()
            .unwrap();

        Box::new(NuccChunkLayerSet {
            version: layerset.version,
            data: layerset.data,
        })
    }
}

impl NuccStruct for NuccLayerSet {
    fn chunk_type(&self) -> NuccChunkType {
        NuccChunkType::NuccChunkLayerSet
    }

    fn version(&self) -> u16 {
        self.version
    }
}
