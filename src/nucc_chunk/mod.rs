mod nucc_chunk_null;
mod nucc_chunk_page;
mod nucc_chunk_index;
mod nucc_chunk_binary;
pub mod nucc_chunk_anm;
pub mod nucc_chunk_anmstrm;
pub mod nucc_chunk_anmstrmframe;
pub mod nucc_chunk_camera;
pub mod nucc_chunk_lightdirc;
pub mod nucc_chunk_lightpoint;
pub mod nucc_chunk_layerset;
pub mod nucc_chunk_ambient;
pub mod nucc_chunk_morphmodel;
mod nucc_chunk_unknown;
pub mod nucc_helper;

use binrw::{io::Cursor, BinRead, BinWrite};
use std::{fmt, str::FromStr};

use downcast_rs::{impl_downcast, Downcast};
use std::error::Error;
use strum_macros::{Display, EnumString};

pub use nucc_chunk_null::NuccChunkNull;
pub use nucc_chunk_page::NuccChunkPage;
pub use nucc_chunk_index::NuccChunkIndex;
pub use nucc_chunk_binary::NuccChunkBinary;
pub use nucc_chunk_anm::NuccChunkAnm;
pub use nucc_chunk_anmstrm::NuccChunkAnmStrm;
pub use nucc_chunk_anmstrmframe::NuccChunkAnmStrmFrame;
pub use nucc_chunk_camera::NuccChunkCamera;
pub use nucc_chunk_lightdirc::NuccChunkLightDirc;
pub use nucc_chunk_lightpoint::NuccChunkLightPoint;
pub use nucc_chunk_layerset::NuccChunkLayerSet;
pub use nucc_chunk_ambient::NuccChunkAmbient;
pub use nucc_chunk_morphmodel::NuccChunkMorphModel;
pub use nucc_chunk_unknown::NuccChunkUnknown;

pub trait NuccChunk: Downcast + fmt::Debug {
    fn chunk_type(&self) -> NuccChunkType;
    fn version(&self) -> u16;
    fn extension(&self) -> String {
        String::new()
    }

    fn read_boxed(
        input: &[u8],
        version: u16,
    ) -> Result<(Vec<u8>, Box<dyn NuccChunk>), Box<dyn Error>>
    where
        Self: Sized + BinRead<Args = u16>,
    {
        // Deserialize the data using binrw
        let mut cursor = Cursor::new(input);
        let result = Self::read_le_args(&mut cursor, version)?;

        // Return the remaining data and the boxed value
        Ok((input.into(), Box::new(result) as Box<dyn NuccChunk>))
    }

    fn write_boxed(
        boxed: Box<dyn NuccChunk>,
        output: &mut Cursor<Vec<u8>>,
        version: u16,
    ) -> Result<(), Box<dyn Error>>
    where
        Self: Sized + BinWrite<Args = ()>,
    {
        // Serialize the data using binrw
        Ok(Self::write_le(
            &boxed.downcast::<Self>().map(|c| *c).ok().unwrap(),
            output,
        )?)
    }
}

impl_downcast!(NuccChunk);

#[derive(Debug, Clone, Display, EnumString, PartialEq, Eq)]
#[strum(serialize_all = "camelCase")]
pub enum NuccChunkType {
    NuccChunkNull,
    NuccChunkPage,
    NuccChunkIndex,
    NuccChunkBinary,
    NuccChunkAnm,
    NuccChunkAnmStrm,
    NuccChunkAnmStrmFrame,
    NuccChunkCamera,
    NuccChunkLightDirc,
    NuccChunkLightPoint,
    NuccChunkLayerSet,
    NuccChunkAmbient,
    NuccChunkMorphModel,
    NuccChunkUnknown,
   
    

}

impl Default for NuccChunkType {
    fn default() -> Self {
        NuccChunkType::NuccChunkUnknown
    }
}

impl NuccChunkType {
    pub fn read_data(
        data: Vec<u8>,
        chunk_type: &str,
        version: u16,
    ) -> Result<(Vec<u8>, Box<dyn NuccChunk>), Box<dyn Error>> {
        match NuccChunkType::from_str(chunk_type).unwrap_or_default() {
            NuccChunkType::NuccChunkNull => Ok((data, Box::new(NuccChunkNull(version)))),
            NuccChunkType::NuccChunkPage => NuccChunkPage::read_boxed(&data, version),
            NuccChunkType::NuccChunkIndex => Ok((data, Box::new(NuccChunkIndex))),
            NuccChunkType::NuccChunkBinary => NuccChunkBinary::read_boxed(&data, version),
            
            NuccChunkType::NuccChunkAnm => NuccChunkAnm::read_boxed(&data, version), // Fix: Change `u16` to `()`
            NuccChunkType::NuccChunkAnmStrm => NuccChunkAnmStrm::read_boxed(&data, version),
            NuccChunkType::NuccChunkAnmStrmFrame => { NuccChunkAnmStrmFrame::read_boxed(&data, version)}
            NuccChunkType::NuccChunkCamera => NuccChunkCamera::read_boxed(&data, version),
            NuccChunkType::NuccChunkLightDirc => NuccChunkLightDirc::read_boxed(&data, version),
            NuccChunkType::NuccChunkLightPoint => NuccChunkLightPoint::read_boxed(&data, version),
            NuccChunkType::NuccChunkLayerSet => NuccChunkLayerSet::read_boxed(&data, version),
            NuccChunkType::NuccChunkAmbient => NuccChunkAmbient::read_boxed(&data, version),
            NuccChunkType::NuccChunkMorphModel => NuccChunkMorphModel::read_boxed(&data, version),


            NuccChunkType::NuccChunkUnknown => Ok((
                data.clone(),
                Box::new(NuccChunkUnknown {
                    version,
                    chunk_type: chunk_type.to_string(),
                    data: data.into(),
                }),
            )),
        }
    }

    pub fn write_data(boxed: Box<dyn NuccChunk>, version: u16) -> Result<Vec<u8>, Box<dyn Error>> {
        // Create a new cursor for writing
        let mut output = Cursor::new(Vec::new());

        // Downcast the boxed trait object to the specific chunk type
        match boxed.chunk_type() {
            NuccChunkType::NuccChunkNull | NuccChunkType::NuccChunkIndex => { return Ok(output.into_inner()); }
            NuccChunkType::NuccChunkPage => { NuccChunkPage::write_boxed(boxed, &mut output, version)?; }
            NuccChunkType::NuccChunkBinary => { NuccChunkBinary::write_boxed(boxed, &mut output, version)?; }
            NuccChunkType::NuccChunkAnm => { NuccChunkAnm::write_boxed(boxed, &mut output, version)?; }
            NuccChunkType::NuccChunkAnmStrm => { NuccChunkAnmStrm::write_boxed(boxed, &mut output, version)?; }
            NuccChunkType::NuccChunkAnmStrmFrame => { NuccChunkAnmStrmFrame::write_boxed(boxed, &mut output, version)?; }
            NuccChunkType::NuccChunkCamera => { NuccChunkCamera::write_boxed(boxed, &mut output, version)?; }
            NuccChunkType::NuccChunkLightDirc => { NuccChunkLightDirc::write_boxed(boxed, &mut output, version)?; }
            NuccChunkType::NuccChunkLightPoint => { NuccChunkLightPoint::write_boxed(boxed, &mut output, version)?; }
            NuccChunkType::NuccChunkLayerSet => { NuccChunkLayerSet::write_boxed(boxed, &mut output, version)?; }
            NuccChunkType::NuccChunkAmbient => { NuccChunkAmbient::write_boxed(boxed, &mut output, version)?; }
            NuccChunkType::NuccChunkMorphModel => { NuccChunkMorphModel::write_boxed(boxed, &mut output, version)?; }

            NuccChunkType::NuccChunkUnknown => {
                let unknown = boxed
                    .downcast::<NuccChunkUnknown>()
                    .map(|x| x.data)
                    .unwrap();
                unknown.write(&mut output)?;
            }
        }

        // Get the written data from the cursor and return it
        Ok(output.into_inner())
    }
}
