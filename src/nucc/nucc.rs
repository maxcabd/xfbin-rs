use binrw::{binrw, helpers::until_eof};

#[binrw]
#[derive(Debug, Clone)]
pub enum NuccChunkType {
    NuccPage(NuccPage),
    NuccBinary(#[br(parse_with = until_eof)] Vec<u8>),
    Unknown(#[br(parse_with = until_eof)] Vec<u8>),



}

#[binrw]
#[derive(Debug, Clone)]
pub struct NuccPage {
    pub page_size: u32,
    pub reference_size: u32
}
