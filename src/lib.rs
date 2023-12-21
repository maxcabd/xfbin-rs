use std::path::Path;
use std::error::Error;   

pub mod nucc;

use crate::nucc::xfbin::Xfbin;


// Use Path type to allow for platform compatibility
pub fn read_xfbin(filepath: &Path) -> Result<Xfbin, Box<dyn Error>> {
    let xfbin = Xfbin::read_from_file(filepath).unwrap();

    Ok(xfbin)
}

// Write xfbin



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn read_xfbin_test() {
        let xfbin = read_xfbin(Path::new("characode.bin.xfbin")).unwrap();
   


        let nucc_binary_chunk = xfbin.get_chunk_by_type("nuccChunkBinary");
        
        
    }

    #[test]
    fn write_xfbin_test() {
    }
}