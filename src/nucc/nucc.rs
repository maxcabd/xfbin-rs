use binrw::{binrw, helpers::until_eof};

#[binrw]
#[derive(Debug, Clone)]
pub enum NuccChunk {
    NuccPage(NuccPage),
    NuccBinary(#[br(parse_with = until_eof)] Vec<u8>),
    Unknown(#[br(parse_with = until_eof)] Vec<u8>),
}

impl NuccChunk {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            NuccChunk::NuccBinary(bytes) => bytes.clone(),
            NuccChunk::Unknown(bytes) => bytes.clone(),
            _ => panic!("Cannot convert chunk to bytes!")
        }
    }
}

#[binrw]
#[derive(Debug, Clone)]
pub struct NuccPage {
    pub page_size: u32,
    pub reference_size: u32
}
