use binrw::{binrw, BinReaderExt, NullString};

use super::nucc::*;


#[binrw]
#[derive(Debug, Clone)]
pub struct Xfbin {
    pub header: NuccHeader,

    pub chunk_table: NuccChunkTable,

    #[br(count = 0)]
    pub pages: Vec<Page>
}



// Custom read function for Xfbin
impl Xfbin {
    pub fn read<R: std::io::Read + std::io::Seek>(mut reader: R) -> binrw::BinResult<Self>
    where
        R: std::io::Read + std::io::Seek,
    { 
        let header = reader.read_be::<NuccHeader>()?;
        let chunk_table = reader.read_be::<NuccChunkTable>()?;
        
        // Add the pages to the pages vector until we reach the end of the file
        let mut pages: Vec<Page> = Vec::new();
        loop {
            match Page::read_page(reader.by_ref(), &chunk_table) {
                Ok(page) => {
                    pages.push(page);
                    
                }
                Err(ref e) if e.is_eof() => {
                    // End of file reached, exit the loop
                    break;
                }
                Err(e) => return Err(e.into()), // Propagate other errors
            }
        }

        Ok(Self {
            header,
            chunk_table,
            pages
        })
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
        
        chunks
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


#[binrw]
#[derive(Debug, Clone)]
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
    pub field0_a: u16,
    
    pub data: NuccChunkType
}

impl Chunk {
    pub fn read_chunk<R: std::io::Read + std::io::Seek>(mut reader: R, chunk_table: &NuccChunkTable) -> binrw::BinResult<Self>
    where
        R: std::io::Read + std::io::Seek,
    {   
        let size = reader.read_be::<u32>()?;
        let chunk_map_index = reader.read_be::<u32>()?;
        let chunk_type = get_chunk_type(chunk_map_index, chunk_table);
        let version = reader.read_be::<u16>()?;
        let field0_a = reader.read_be::<u16>()?;
        let data = Self::read_chunk_data(size, chunk_type.as_str(), reader.by_ref())?;

        let chunk = Self {
            size,
            chunk_map_index,
            version,
            field0_a,
            data
        };
   
        Ok(chunk)
    }

    fn read_chunk_data<R: std::io::Read + std::io::Seek>(
        size: u32,
        chunk_type: &str,
        reader: &mut R,
    ) -> binrw::BinResult<NuccChunkType> {

        match chunk_type {
            "nuccChunkPage" => Ok(NuccChunkType::NuccPage(reader.read_be::<NuccPage>()?)),

            "nuccChunkBinary" => {
                let mut bytes = vec![0; size as usize];
                reader.read_exact(&mut bytes)?;
                Ok(NuccChunkType::NuccBinary(bytes))
            }
            
            _ => {
                // Read the chunk data as a byte array
                let mut bytes = vec![0; size as usize];
                reader.read_exact(&mut bytes)?;
                Ok(NuccChunkType::Unknown(bytes))
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
    pub fn read_page<R: std::io::Read + std::io::Seek>(mut reader: R, chunk_table: &NuccChunkTable) -> binrw::BinResult<Self>
    where
        R: std::io::Read + std::io::Seek,
    { 
        let mut page = reader.read_be::<Page>()?;

        let mut chunks: Vec<Chunk> = Vec::new();
        loop { // Keep reading chunks until we reach the end of the page which is when the chunk_type is "nuccChunkPage"
            let chunk = Chunk::read_chunk(reader.by_ref(), chunk_table)?;
            let chunk_type = get_chunk_type(chunk.chunk_map_index, chunk_table);

            chunks.push(chunk);

            if chunk_type == "nuccChunkPage" {
                break;
            }
        }


        page.chunks = chunks;

        Ok(page)
    }
    pub fn get_last_chunk_page_size(&self) -> u32 {
        if let Some(last_chunk) = self.chunks.last() {
            if let NuccChunkType::NuccPage(nucc_page) = &last_chunk.data {
                return nucc_page.page_size;
            }
        }
        0
    }

}


fn get_chunk_type(chunk_type_index: u32, chunk_table: &NuccChunkTable) -> String {
    let chunk_type = chunk_table.chunk_types[chunk_type_index as usize].clone();
    let chunk_type = chunk_type.to_string();

    chunk_type
}