pub mod nucc_binary;
pub mod nucc_anm;
pub mod nucc_anmstrm;
pub mod nucc_anmstrmframe;
pub mod nucc_camera;
pub mod nucc_lightdirc;
pub mod nucc_lightpoint;
pub mod nucc_layerset;
pub mod nucc_ambient;
pub mod nucc_morphmodel;

pub mod nucc_unknown;

use downcast_rs::{impl_downcast, Downcast};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::nucc_chunk::*;
use super::xfbin_file::{XfbinChunkMap, XfbinChunkReference};

pub use nucc_binary::NuccBinary;
pub use nucc_anm::NuccAnm;
pub use nucc_anmstrm::NuccAnmStrm;
pub use nucc_anmstrmframe::NuccAnmStrmFrame;
pub use nucc_camera::NuccCamera;
pub use nucc_lightdirc::NuccLightDirc;
pub use nucc_lightpoint::NuccLightPoint;
pub use nucc_layerset::NuccLayerSet;
pub use nucc_ambient::NuccAmbient;
pub use nucc_morphmodel::NuccMorphModel;
pub use nucc_unknown::NuccUnknown;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq, Hash)]
pub struct NuccStructInfo {
    pub chunk_name: String,
    pub chunk_type: String,
    pub filepath: String,
}

impl fmt::Display for NuccStructInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ Name: \"{}\", Type: \"{}\", Path: \"{}\" }}",
            self.chunk_name, self.chunk_type, self.filepath
        )
    }
}

pub struct XfbinChunkMapConverter {
    pub chunk_maps: Vec<XfbinChunkMap>,
    pub chunk_names: Vec<String>,
    pub chunk_types: Vec<String>,
    pub filepaths: Vec<String>,
}

impl From<XfbinChunkMapConverter> for Vec<NuccStructInfo> {
    fn from(converter: XfbinChunkMapConverter) -> Self {
        let XfbinChunkMapConverter {
            chunk_maps,
            chunk_names: names,
            chunk_types: types,
            filepaths: paths,
        } = converter;

        chunk_maps
            .into_iter()
            .map(|c| NuccStructInfo {
                chunk_name: names[c.chunk_name_index as usize].clone(),
                chunk_type: types[c.chunk_type_index as usize].clone(),
                filepath: paths[c.filepath_index as usize].clone(),
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq, Hash)]
pub struct NuccStructReference {
    pub chunk_name: String,
    pub struct_info: NuccStructInfo,
}

pub struct XfbinChunkReferenceConverter {
    pub references: Vec<XfbinChunkReference>,
    pub chunk_names: Vec<String>,
    pub struct_infos: Vec<NuccStructInfo>,
}

impl From<XfbinChunkReferenceConverter> for Vec<NuccStructReference> {
    fn from(converter: XfbinChunkReferenceConverter) -> Self {
        let XfbinChunkReferenceConverter {
            references,
            chunk_names: names,
            struct_infos: infos,
        } = converter;

        references
            .into_iter()
            .map(|r| NuccStructReference {
                chunk_name: names[r.chunk_name_index as usize].clone(),
                struct_info: infos[r.chunk_map_index as usize].clone(),
            })
            .collect()
    }
}

pub trait NuccInfo {
    fn struct_info(&self) -> &NuccStructInfo;
    fn struct_info_mut(&mut self) -> &mut NuccStructInfo;
}

macro_rules! impl_nucc_info {
    ($struct:ident,$field:ident) => {
        impl NuccInfo for $struct {
            fn struct_info(&self) -> &NuccStructInfo {
                &self.$field
            }

            fn struct_info_mut(&mut self) -> &mut NuccStructInfo {
                &mut self.$field
            }
        }
    };
}

pub(crate) use impl_nucc_info;

pub trait NuccStruct: NuccInfo + Downcast {
    fn chunk_type(&self) -> NuccChunkType;
    fn version(&self) -> u16;
}

impl std::fmt::Debug for dyn NuccStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NuccStruct {{ chunk_type: {:?}, version: {} }}",
            self.chunk_type(),
            self.version()
        )
    }
}


impl_downcast!(NuccStruct);

//// Converts a NuccStruct to a NuccChunk
pub struct NuccStructConverter {
    pub nucc_chunk: Box<dyn NuccChunk>,
    pub struct_infos: Vec<NuccStructInfo>,
    pub struct_references: Vec<NuccStructReference>,
}

pub struct NuccChunkConverter {
    pub nucc_struct: Box<dyn NuccStruct>,
    pub struct_info_map: IndexMap<NuccStructInfo, u32>,
    pub struct_reference_map: IndexMap<NuccStructReference, u32>,
}

impl From<NuccStructConverter> for Box<dyn NuccStruct> {
    fn from(converter: NuccStructConverter) -> Self {
        match converter.nucc_chunk.chunk_type() {
            NuccChunkType::NuccChunkBinary => Box::new(NuccBinary::from(converter)),
            NuccChunkType::NuccChunkAnm => Box::new(NuccAnm::from(converter)),
            NuccChunkType::NuccChunkAnmStrm => Box::new(NuccAnmStrm::from(converter)),
            NuccChunkType::NuccChunkAnmStrmFrame => Box::new(NuccAnmStrmFrame::from(converter)),
            NuccChunkType::NuccChunkCamera => Box::new(NuccCamera::from(converter)),
            NuccChunkType::NuccChunkLightDirc => Box::new(NuccLightDirc::from(converter)),
            NuccChunkType::NuccChunkLightPoint => Box::new(NuccLightPoint::from(converter)),
            NuccChunkType::NuccChunkLayerSet => Box::new(NuccLayerSet::from(converter)),
            NuccChunkType::NuccChunkAmbient => Box::new(NuccAmbient::from(converter)),
            NuccChunkType::NuccChunkMorphModel => Box::new(NuccMorphModel::from(converter)),
            NuccChunkType::NuccChunkUnknown => Box::new(NuccUnknown::from(converter)),
            any => panic!("Unexpected NuccChunkType: {any}"),
        }
    }
}

impl From<NuccChunkConverter> for Box<dyn NuccChunk> {
    fn from(converter: NuccChunkConverter) -> Self {
        match converter.nucc_struct.chunk_type() {
            NuccChunkType::NuccChunkBinary => { Box::<NuccChunkBinary>::from(converter) as Box<dyn NuccChunk> }
            NuccChunkType::NuccChunkAnm => { Box::<NuccChunkAnm>::from(converter) as Box<dyn NuccChunk> }
            NuccChunkType::NuccChunkAnmStrm => { Box::<NuccChunkAnmStrm>::from(converter) as Box<dyn NuccChunk> }
            NuccChunkType::NuccChunkAnmStrmFrame => { Box::<NuccChunkAnmStrmFrame>::from(converter) as Box<dyn NuccChunk> }
            NuccChunkType::NuccChunkCamera => { Box::<NuccChunkCamera>::from(converter) as Box<dyn NuccChunk> }
            NuccChunkType::NuccChunkLightDirc => { Box::<NuccChunkLightDirc>::from(converter) as Box<dyn NuccChunk> }
            NuccChunkType::NuccChunkLightPoint => { Box::<NuccChunkLightPoint>::from(converter) as Box<dyn NuccChunk> }
            NuccChunkType::NuccChunkLayerSet => { Box::<NuccChunkLayerSet>::from(converter) as Box<dyn NuccChunk> }
            NuccChunkType::NuccChunkAmbient => { Box::<NuccChunkAmbient>::from(converter) as Box<dyn NuccChunk> }
            NuccChunkType::NuccChunkMorphModel => { Box::<NuccChunkMorphModel>::from(converter) as Box<dyn NuccChunk> }
            NuccChunkType::NuccChunkUnknown => { Box::<NuccChunkUnknown>::from(converter) as Box<dyn NuccChunk> }



            any => panic!("Unexpected NuccChunkType: {any}"),
        }
    }
}
