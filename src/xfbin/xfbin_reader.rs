use std::fs::File;
use binrw::BinReaderExt;

use crate::xfbin::structure::xfbin::Xfbin;


pub fn read_xfbin(filepath: &str) -> Xfbin {
    let mut xfbin = File::open(filepath).unwrap()
    .read_be::<Xfbin>().unwrap();

    xfbin.set_chunk_types();
    xfbin
}


