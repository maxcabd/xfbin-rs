mod xfbin;
use binrw::{binrw, BinReaderExt, NullString, BinRead};
use binrw::io::Cursor;
use crate::xfbin::xfbin_reader::read_xfbin;



fn main() {
    let xfbin = read_xfbin("C:\\Users\\User\\Desktop\\Projects\\Rust\\xfbin\\dlc.bin.xfbin");
    
    
   





}


// 
#[binrw]
#[derive(Debug)]
pub struct NARUTOUNS4DLC008 {
    pub unk1: u32,
    pub version: u32,
    pub count: u32,
    pub unk2: u32,

    #[br(count = count)]
    pub entry: Vec<Entry>


}

#[binrw]
#[derive(Debug)]
pub struct Entry {
    pub unk1: u32,  
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub path: String,

}




