pub mod nucc;

use std::path::Path;
use anyhow::{Result, Context};

use crate::nucc::xfbin::Xfbin;

// Use the Path type for better compatibility with different OSes
pub fn read_xfbin(filepath: &Path) -> Result<Xfbin> {
    Xfbin::read_from_file(filepath).context(format!("Failed to read xfbin from file: {}", filepath.display()))
}

/*fn write_xfbin(filepath: &Path, xfbin: &Xfbin) {
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_xfbin_test() {
        let xfbin = read_xfbin(Path::new("2nrtbod1_col2.xfbin")).unwrap();
        
        let _ = dbg!(&xfbin.pages.len());    
    }   

    #[test]
    fn write_xfbin_test() {
    }
}