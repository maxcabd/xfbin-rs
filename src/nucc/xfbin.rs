use binrw::{binrw, BinReaderExt, NullString};

use super::nucc::*;
use anyhow::{Result, Context};

#[binrw]
#[derive(Debug, Clone)]
pub struct Xfbin {
    pub header: NuccHeader,
    pub chunk_table: NuccChunkTable,
    #[br(count = 0)]
    pub pages: Vec<Page>
}

impl Xfbin {
    pub fn read<R: std::io::Read + std::io::Seek>(mut reader: R) -> binrw::BinResult<Self>
    where
        R: std::io::Read + std::io::Seek,
    { 
        let header = reader.read_be::<NuccHeader>()?;
        let chunk_table = reader.read_be::<NuccChunkTable>()?;

        let mut current_page_start = 0;
        
        // Add the pages to the pages vector until we reach the end of the file
        let mut pages: Vec<Page> = Vec::new();
        loop {
            match Page::read_page(reader.by_ref(), &chunk_table, current_page_start) {
                Ok(page) => {
                    pages.push(page);
                    // Get the page size of the last page (always a NuccPage chunk) and add it to the current page start
                    let page_size = pages.last().unwrap().get_page_size();
                    current_page_start += page_size
                }
                Err(ref e) if e.is_eof() => {
                    // End of file reached, exit the loop
                    break;
                }
                Err(e) => return Err(e.into()), // Propagate other errors
            }
        }
        
        Ok(Self { header, chunk_table, pages })
    }

    pub fn read_from_file(filepath: &std::path::Path) -> binrw::BinResult<Self> {
        let file = std::fs::File::open(filepath)?;
        let reader = std::io::BufReader::new(file);
        let xfbin = Self::read(reader)?;
    
        Ok(xfbin)
    }

    pub fn get_chunk_by_type(&self, chunk_type: &str) -> Vec<&Chunk> {
        let mut chunks: Vec<&Chunk> = Vec::new();
        for page in &self.pages {
            for chunk in &page.chunks {
                let chunk_type_index = self.chunk_table.chunk_maps[chunk.chunk_map_index as usize].chunk_type_index;
                let chunk_type_name = &self.chunk_table.chunk_types[chunk_type_index as usize].to_string();

                if chunk_type_name == chunk_type {
                    chunks.push(chunk);
                }
            }
        }
        
        return chunks
    }
}

#[binrw]
#[brw(magic = b"NUCC")]
#[derive(Debug, Clone)]
pub struct NuccHeader {
    pub version: u32,

    #[brw(pad_before = 8)]
    pub chunk_table_size: u32,
    pub min_page_size: u32,
    pub file_id: u16,
    pub field1_a: u16,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct NuccChunkTable {
    pub chunk_type_count: u32,
    pub chunk_type_size: u32,

    pub filepath_count: u32,
    pub filepath_size: u32,

    pub chunk_name_count: u32,
    pub chunk_name_size: u32,

    pub chunk_map_count: u32,
    pub chunk_map_size: u32,

    pub chunk_map_indices_count: u32,
    pub chunk_map_references_count: u32,

    #[br(count = chunk_type_count)]
    pub chunk_types: Vec<NullString>,

    #[br(count = filepath_count)]
    pub filepaths: Vec<NullString>,

    #[br(count = chunk_name_count)]
    pub chunk_names: Vec<NullString>,

    #[brw(align_before = 4)]
    #[br(count = chunk_map_count)]
    pub chunk_maps: Vec<ChunkMap>,

    #[br(count = chunk_map_references_count)]
    pub chunk_map_references: Vec<ChunkReference>,

    #[br(count = chunk_map_indices_count)]
    pub chunk_map_indices: Vec<u32>
}

impl NuccChunkTable {
    pub fn get_chunk_info(&self, chunk_map: &ChunkMap) -> (String, String, String) {
        let chunk_type = get_chunk_type(chunk_map.chunk_type_index, self);
        let filepath = get_chunk_filepath(chunk_map.filepath_index, self);
        let chunk_name = get_chunk_name(chunk_map.chunk_name_index, self);

        return (chunk_type, filepath, chunk_name)
    }
}

#[binrw]
#[derive(Debug, Clone, Default)]
pub struct ChunkMap {
    pub chunk_type_index: u32,
    pub filepath_index: u32,
    pub chunk_name_index: u32,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct ChunkReference {
    pub chunk_name_index: u32,
    pub chunk_map_index: u32,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct Chunk {
    pub size: u32,
    pub chunk_map_index: u32,
    pub version: u16,
    pub unk: u16, // Might be an extra version field, not important for now

    #[brw(ignore)] // This won't be read or written just stored for later use
    pub chunk_map: ChunkMap,

    pub data: NuccChunk
}

impl Chunk {
    pub fn read_chunk<R: std::io::Read + std::io::Seek>(mut reader: R, chunk_table: &NuccChunkTable, page_start: u32) -> binrw::BinResult<Self>
    where
        R: std::io::Read + std::io::Seek,
    {   
        let size = reader.read_be::<u32>()?;
        let chunk_map_index = reader.read_be::<u32>()?;

        let chunk_map = chunk_table.chunk_maps[get_chunk_map_index(chunk_map_index, page_start, chunk_table) as usize].clone();
        let chunk_type = chunk_table.get_chunk_info(&chunk_map).0;

        let version = reader.read_be::<u16>()?;
        let unk = reader.read_be::<u16>()?; 
        let data = Self::read_chunk_data(size, chunk_type.as_str(), reader.by_ref())?;


        // Read the chunk map based on an offset since the chunk map index is not the same as the chunk map's index in the chunk map vector
        //println!("page start: {}", page_start);
        //println!("chunk_map_index: {}", get_chunk_map_index(chunk_map_index, page_start, chunk_table));
        let chunk_map = chunk_table.chunk_maps[get_chunk_map_index(chunk_map_index, page_start, chunk_table) as usize].clone();

        let chunk = Self {
            size,
            chunk_map_index,
            version,
            unk,
            chunk_map,
            data
        };
   
        Ok(chunk)
    }

    fn read_chunk_data<R: std::io::Read + std::io::Seek>(
        size: u32,
        chunk_type: &str,
        reader: &mut R,
    ) -> binrw::BinResult<NuccChunk> {

        match chunk_type {
            "nuccChunkPage" => Ok(NuccChunk::NuccPage(reader.read_be::<NuccPage>()?)),

            "nuccChunkBinary" => {
                let mut bytes = vec![0; size as usize];
                reader.read_exact(&mut bytes)?;
                Ok(NuccChunk::NuccBinary(bytes))
            }

            _ => {
                // Read the unknown chunk data as a byte array
                let mut bytes = vec![0; size as usize];
                reader.read_exact(&mut bytes)?;
                Ok(NuccChunk::Unknown(bytes))
            }
        }
    }
}

#[binrw]
#[derive(Debug, Clone, Default)]
pub struct Page {
    #[br(count = 0)]
    pub chunks: Vec<Chunk>
}

impl Page {
    pub fn read_page<R: std::io::Read + std::io::Seek>(mut reader: R, chunk_table: &NuccChunkTable, current_page_start: u32) -> binrw::BinResult<Self>
    where
        R: std::io::Read + std::io::Seek,
    {   
        let page_start = current_page_start;

        let mut page = reader.read_be::<Page>()?;

        let mut chunks: Vec<Chunk> = Vec::new();

        loop { // Keep reading chunks until we reach the end of the page which is when the chunk_type is "nuccChunkPage"
            let chunk = Chunk::read_chunk(reader.by_ref(), chunk_table, page_start)?;
            let chunk_type = chunk_table.get_chunk_info(&chunk.chunk_map).0;

            chunks.push(chunk);

            if chunk_type == "nuccChunkPage" {
                break;
            }
        }

        page.chunks = chunks;

        Ok(page)
    }

    pub fn get_page_size(&self) -> u32 {
        if let Some(last_chunk) = self.chunks.last() {
            if let NuccChunk::NuccPage(nucc_page) = &last_chunk.data {
                return nucc_page.page_size;
            }
        }
        return 0 // Return 0 if the last chunk is not a NuccPage chunk
    }

    pub fn get_reference_size(&self) -> u32 {
        if let Some(last_chunk) = self.chunks.last() {
            if let NuccChunk::NuccPage(nucc_page) = &last_chunk.data {
                return nucc_page.reference_size;
            }
        }
        return 0
    }

}

pub fn get_chunk_map_index(index: u32, page_start: u32, chunk_table: &NuccChunkTable) -> u32 {
    let chunk_map_index = chunk_table.chunk_map_indices[(page_start + index) as usize];

    return chunk_map_index

}

pub fn get_chunk_type(chunk_type_index: u32, chunk_table: &NuccChunkTable) -> String {
    let chunk_type = chunk_table.chunk_types[chunk_type_index as usize].clone();
    return chunk_type.to_string()
}

pub fn get_chunk_filepath(chunk_map_index: u32, chunk_table: &NuccChunkTable) -> String {
    let chunk_map = chunk_table.chunk_maps[chunk_map_index as usize].clone();
    let filepath_index = chunk_map.filepath_index;
    let filepath = chunk_table.filepaths[filepath_index as usize].clone();

    return filepath.to_string();
    
}

pub fn get_chunk_name(chunk_map_index: u32, chunk_table: &NuccChunkTable) -> String {
    let chunk_map = chunk_table.chunk_maps[chunk_map_index as usize].clone();
    let chunk_name_index = chunk_map.chunk_name_index;
    let chunk_name = chunk_table.chunk_names[chunk_name_index as usize].clone();
 
    return chunk_name.to_string();
}