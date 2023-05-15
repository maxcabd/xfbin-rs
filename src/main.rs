// Parser for the XFbin file format
use std::fs::File;
use binrw::BinReaderExt;
use structure::xfbin::Chunk;


// Import the structures from the folder structure

mod structure;
mod utils;

//use crate::structure::anm::NuccChunkAnm;
use crate::structure::xfbin::Xfbin;


fn main() {
    
    let mut file = File::open("C:\\Users\\13439\\Desktop\\Rust\\xfbin_lib\\dlc.bin.xfbin").unwrap();

    let xfbin = file.read_be::<Xfbin>().unwrap();

    println!("{:#?}", xfbin.chunk_table.get_chunk_info(0).0);

    println!("{:#?}", xfbin.pages[0].chunks);
    



}



