pub mod nucc;
pub mod nucc_chunk;
pub mod page;
pub mod xfbin;
pub mod xfbin_file;

use anyhow::{Context, Result};
use binrw::{io::Cursor, BinReaderExt, BinWrite};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub use xfbin::Xfbin;
use xfbin_file::XfbinFile;

pub fn read_xfbin(filepath: &dyn AsRef<Path>) -> Result<Xfbin> {
    read_xfbin_buf(fs::read(filepath)?)
}

pub fn read_xfbin_buf(buf: Vec<u8>) -> Result<Xfbin> {
    let mut reader = std::io::Cursor::new(buf);

    let xfbin_file = reader
        .read_be::<XfbinFile>()
        .with_context(|| format!("Failed to read xfbin from buffer"))?;

    Ok(xfbin_file.into())
}

pub fn write_xfbin(xfbin: Xfbin, filepath: &dyn AsRef<Path>) -> Result<()> {
    let buf = write_xfbin_buf(xfbin)?;

    let mut file = File::create(filepath)?;

    Ok(file.write_all(&buf)?)
}

pub fn write_xfbin_buf(xfbin: Xfbin) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(Vec::new());

    Ok(XfbinFile::from(xfbin) // Convert the Xfbin to an XfbinFile
        .write_be(&mut cursor)
        .map(|_| cursor.into_inner())?)
}

#[cfg(test)]
mod tests {

    use self::{
        nucc::{NuccAnm, NuccChunkConverter, NuccStruct},
        nucc_chunk::{nucc_chunk_anm, NuccChunk, NuccChunkAnm, NuccChunkType}, xfbin::XfbinPage,
    };

    use super::*;

    #[test]
    fn read_xfbin_test() -> Result<()> {
        let xfbin = read_xfbin(&Path::new("4rincharsel.xfbin"))?;

        for page in xfbin.pages {
            let (page_structs, page_struct_infos, page_struct_references) = page.destructure();

            for page_struct in page_structs {
                match page_struct.chunk_type() {
                    NuccChunkType::NuccChunkAnm => {
                        let anm = page_struct.downcast_ref::<NuccAnm>().unwrap().clone();
                        let boxed = Box::<dyn NuccChunk>::from(NuccChunkConverter {
                            nucc_struct: Box::new(anm.clone()),
                            struct_info_map: page_struct_infos.clone(),
                            struct_reference_map: page_struct_references.clone(),
                        });

                        let anm_chunk = boxed.downcast::<NuccChunkAnm>().unwrap();
                        let extension = anm_chunk.extension();
                        println!("Extension: {}", extension);

                        // Remove the Box from nuccChunkAnm and save it to a file
                        let mut cursor = Cursor::new(Vec::new());
                        NuccChunkAnm::write_boxed(anm_chunk, &mut cursor, anm.version).unwrap();

                        let mut file = File::create(format!("{}{}", anm.struct_info.chunk_name, extension))?;

                        file.write_all(&cursor.into_inner())?;


                        
                       
                    }
                    _ => {}
                }
            }

        }

        

        Ok(())
    }

    #[test]
    fn write_xfbin_test() -> Result<()> {
        let mut xfbin = read_xfbin(&Path::new("4rincharsel.xfbin"))?;

        let mut new_page = XfbinPage::default();

        // Create a new ANM chunk info
        let anm_struct_info = nucc::NuccStructInfo {
            chunk_name: String::from("4rincharsel30"),
            filepath: String::from("c\\crsel\\4rin\\anm\\4rincharsel30.max"),
            chunk_type: nucc_chunk::NuccChunkType::NuccChunkAnm.to_string(),
        };

        // Create a new ANM chunk with default
        let anm_chunk = NuccAnm {
            struct_info: anm_struct_info.clone(),
            version: 121,
            frame_count: 0,
            is_looped: false,
            clumps: vec![],
            other_entries_indices: vec![],
            unk_entry_indices: vec![],
            coord_parents: vec![],
            entries: vec![],
        };

        // Cast the ANM struct to a NuccStruct
        let anm_struct = Box::new(anm_chunk) as Box<dyn nucc::NuccStruct>;

        // Add the ANM struct to the new page
        new_page.structs.push(anm_struct);
        new_page.struct_infos.push(anm_struct_info.clone());
        new_page.struct_references = xfbin.pages[0].struct_references.clone();

        xfbin.pages.push(new_page);

        write_xfbin(xfbin, &Path::new("4rincharsel-new.xfbin"))?;

        Ok(())
    }
}
